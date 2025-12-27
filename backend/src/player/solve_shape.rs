use log::debug;
use opencv::core::{Point, Point2d, Rect};

use crate::{
    bridge::MouseKind,
    ecs::{Resources, transition, transition_if, try_ok_transition},
    player::{
        Player, PlayerAction, PlayerContext, PlayerEntity, next_action,
        timeout::{Lifecycle, Timeout, next_timeout_lifecycle},
        transition_from_action,
    },
    tracker::{ByteTracker, Detection, STrack},
};

/// Representing the current state of transparent shape (e.g. lie detector) solving.
#[derive(Debug, Clone, Copy, Default)]
pub enum State {
    #[default]
    Waiting,
    Solving(Timeout),
    Completed,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SolvingShape {
    state: State,
    lie_detector_region: Option<Rect>,
    current_track_id: Option<u64>,
    candidate_track_id: Option<u64>,
    last_cursor: Option<Point>,
    bg_direction: Point2d,
    bg_velocity: Point2d,
}

/// Updates the [`Player::SolvingShape`] contextual state.
///
/// Note: This state does not use any [`Task`], so all detections are blocking. But this should be
/// acceptable for this state.
pub fn update_solving_shape_state(resources: &Resources, player: &mut PlayerEntity) {
    let Player::SolvingShape(mut solving_shape) = player.state else {
        panic!("state is not solving shape");
    };

    match solving_shape.state {
        State::Waiting => update_waiting(resources, &mut player.context, &mut solving_shape),
        State::Solving(_) => update_solving(
            resources,
            player.context.shape_tracker(),
            &mut solving_shape,
        ),
        State::Completed => unreachable!(),
    }

    let player_next_state = if matches!(solving_shape.state, State::Completed) {
        Player::Idle
    } else {
        Player::SolvingShape(solving_shape)
    };

    match next_action(&player.context) {
        Some(PlayerAction::SolveShape) => transition_from_action!(
            player,
            player_next_state,
            matches!(player_next_state, Player::Idle)
        ),
        Some(_) => unreachable!(),
        None => transition!(player, Player::Idle), // Force cancel if not from action
    }
}

fn update_waiting(
    resources: &Resources,
    player_context: &mut PlayerContext,
    solving_shape: &mut SolvingShape,
) {
    const CHECK_INTERVAL: u64 = 30;

    let State::Waiting = solving_shape.state else {
        panic!("solving shape state is not waiting")
    };

    if !resources.tick.is_multiple_of(CHECK_INTERVAL) {
        return;
    }
    if resources.detector().detect_lie_detector_preparing() {
        return;
    }

    let title = try_ok_transition!(
        solving_shape,
        State::Completed,
        resources.detector().detect_lie_detector()
    );

    transition!(solving_shape, State::Solving(Timeout::default()), {
        let tl = title.tl();
        let br = title.br() + Point::new(660, 530);
        let region = Rect::from_points(tl, br);
        player_context.reset_shape_tracker();
        solving_shape.lie_detector_region = Some(region);
        debug!(target: "player", "lie detector transparent shape region: {region:?}");
    });
}

fn update_solving(
    resources: &Resources,
    tracker: &mut ByteTracker,
    solving_shape: &mut SolvingShape,
) {
    const CHECK_INTERVAL: u64 = 30;

    let State::Solving(timeout) = solving_shape.state else {
        panic!("solving shape state is not solving")
    };

    if resources.tick.is_multiple_of(CHECK_INTERVAL) {
        transition_if!(
            solving_shape,
            State::Completed,
            resources.detector().detect_lie_detector().is_err()
        );
    }

    match next_timeout_lifecycle(timeout, 545) {
        Lifecycle::Ended => transition!(solving_shape, State::Completed),
        Lifecycle::Started(timeout) | Lifecycle::Updated(timeout) => {
            transition!(solving_shape, State::Solving(timeout), {
                perform_solving(resources, tracker, solving_shape);
            })
        }
    }
}

fn perform_solving(
    resources: &Resources,
    tracker: &mut ByteTracker,
    solving_shape: &mut SolvingShape,
) {
    let region = solving_shape.lie_detector_region.expect("has region");
    let shapes = resources.detector().detect_transparent_shapes(region);
    let tracks = tracker.update(shapes.into_iter().map(Detection::new).collect());

    if solving_shape.current_track_id.is_none() {
        let region_mid = mid_point(Rect::new(0, 0, region.width, region.height));
        if let Some(track) = find_track_closest_to(region_mid, &tracks) {
            solving_shape.current_track_id = Some(track.track_id());
            solving_shape.last_cursor = Some(mid_point(track.rect()));
        }
    }

    if let Some((direction, velocity)) = estimate_background_direction_velocity(&tracks) {
        solving_shape.bg_direction = direction;
        solving_shape.bg_velocity = velocity;
    }

    match select_best_track(solving_shape, &tracks) {
        Some(track) => {
            let next_cursor = predicted_center(track);
            let absolute_next_cursor = next_cursor + region.tl();
            if solving_shape.current_track_id != Some(track.track_id()) {
                debug!(target: "player", "shape id switches from {:?} to {}", solving_shape.current_track_id, track.track_id());
            }
            resources.input.send_mouse(
                absolute_next_cursor.x,
                absolute_next_cursor.y,
                MouseKind::Move,
            );
            solving_shape.current_track_id = Some(track.track_id());
            solving_shape.last_cursor = Some(next_cursor);

            #[cfg(debug_assertions)]
            #[cfg(feature = "debug_transparent_shape")]
            debug_transparent_shapes(resources, solving_shape, &tracks);
        }
        None => {
            let Some(last_cursor) = solving_shape.last_cursor else {
                return;
            };
            let scaled = solving_shape.bg_velocity * 0.4;
            let next_cursor =
                last_cursor + Point::new(-scaled.x.round() as i32, -scaled.y.round() as i32);
            let absolute_next_cursor = next_cursor + region.tl();
            resources.input.send_mouse(
                absolute_next_cursor.x,
                absolute_next_cursor.y,
                MouseKind::Move,
            );
            solving_shape.last_cursor = Some(next_cursor);

            #[cfg(debug_assertions)]
            #[cfg(feature = "debug_transparent_shape")]
            debug_transparent_shapes(resources, solving_shape, &tracks);
        }
    }
}

#[cfg(debug_assertions)]
#[cfg(feature = "debug_transparent_shape")]
fn debug_transparent_shapes(
    resources: &Resources,
    solving_shape: &SolvingShape,
    tracks: &[STrack],
) {
    use opencv::core::MatTraitConst;

    use crate::debug::debug_tracks;

    debug_tracks(
        &resources
            .detector()
            .mat()
            .roi(solving_shape.lie_detector_region.unwrap())
            .unwrap(),
        tracks.to_vec(),
        solving_shape.last_cursor.unwrap(),
        solving_shape.bg_direction,
    );
}

fn find_track_closest_to(point: Point, tracks: &[STrack]) -> Option<&STrack> {
    tracks.iter().min_by_key(|track| {
        let track_region = track.rect();
        let track_mid =
            track_region.tl() + Point::new(track_region.width / 2, track_region.height / 2);

        (point - track_mid).norm() as i32
    })
}

fn select_best_track<'a>(
    solving_shape: &mut SolvingShape,
    tracks: &'a [STrack],
) -> Option<&'a STrack> {
    let current_track_id = solving_shape.current_track_id?;
    let bg_direction = solving_shape.bg_direction;
    let match_tracks = tracks
        .iter()
        .filter(|track| {
            track.tracklet_len() >= 1
                && track.track_id() != current_track_id
                && is_track_opposite_background_direction(track, bg_direction)
        })
        .collect::<Vec<&STrack>>();
    if match_tracks.len() == 1 {
        let track = match_tracks[0];
        if solving_shape.candidate_track_id == Some(track.track_id()) {
            solving_shape.candidate_track_id = None;
            return Some(track);
        }

        solving_shape.candidate_track_id = Some(track.track_id());
    }

    tracks
        .iter()
        .find(|track| track.track_id() == current_track_id)
}

fn mid_point(rect: Rect) -> Point {
    rect.tl() + Point::new(rect.width / 2, rect.height / 2)
}

fn predicted_center(track: &STrack) -> Point {
    let (vx, vy) = track.kalman_velocity();
    let point = mid_point(track.kalman_rect());

    Point::new(
        (point.x as f32 + vx).round() as i32,
        (point.y as f32 + vy).round() as i32,
    )
}

fn is_track_opposite_background_direction(track: &STrack, bg_direction: Point2d) -> bool {
    let diff = mid_point(track.rect()) - mid_point(track.last_rect());
    let norm = diff.norm();
    if norm < 1e-3 {
        return false;
    }
    let unit = diff.to::<f64>().unwrap() / norm;
    let dot = unit.dot(bg_direction);
    if dot >= -0.1 {
        return false;
    }

    true
}

fn estimate_background_direction_velocity(tracks: &[STrack]) -> Option<(Point2d, Point2d)> {
    let filtered = tracks
        .iter()
        .map(track_velocity)
        .filter(|velocity| velocity.norm() >= 2.0)
        .collect::<Vec<Point2d>>();
    if filtered.len() < 3 {
        return None;
    }

    let len = filtered.len() as f64;
    let velocity_sum = filtered
        .into_iter()
        .fold(Point2d::default(), |acc, v| acc + v);
    let velocity_norm = velocity_sum.norm();
    if velocity_norm < 1e-3 {
        return None;
    }

    Some((velocity_sum / velocity_norm, velocity_sum / len))
}

fn track_velocity(track: &STrack) -> Point2d {
    let (vx, vy) = track.kalman_velocity();
    Point2d::new(vx as f64, vy as f64)
}
