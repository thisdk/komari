use log::debug;
use opencv::core::{Point, Point2d, Rect};

use crate::{
    bridge::MouseKind,
    ecs::Resources,
    player::{
        Player, PlayerAction, PlayerContext, PlayerEntity, next_action,
        timeout::{Lifecycle, Timeout, next_timeout_lifecycle},
    },
    tracker::{ByteTracker, Detection, STrack},
    transition, transition_from_action, transition_if, try_ok_transition,
};

#[derive(Debug)]
struct SelectedTrack<'a> {
    new_track: Option<&'a STrack>,
    new_rejected_count: Option<u32>,
}

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
    current_track_rejected_count: u32,
    current_track_last_frame_id: Option<u64>,
    last_cursor: Option<Point>,
    bg_direction: Point2d,
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

    let title = try_ok_transition!(
        solving_shape,
        State::Completed,
        resources.detector().detect_lie_detector()
    );
    let Ok(in_progress) = resources.detector().detect_lie_detector_in_progress() else {
        return;
    };

    transition!(solving_shape, State::Solving(Timeout::default()), {
        let tl = title.tl() - Point::new(10, 0);
        let br = in_progress.br() + Point::new(220, 0);
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
            solving_shape.current_track_last_frame_id = Some(track.frame_id());
            solving_shape.last_cursor = Some(mid_point(track.rect()));
        }
    }

    if let Some(direction) = estimate_background_direction(&tracks) {
        if solving_shape.bg_direction.norm() == 0.0 {
            solving_shape.bg_direction = direction;
        } else {
            let smooth = solving_shape.bg_direction * 0.6 + direction * 0.4;
            let norm = smooth.norm();
            if norm >= 1e-3 {
                solving_shape.bg_direction = smooth / norm;
            }
        }
    }

    let selected_track = select_best_track(solving_shape, tracker.frame_id(), &tracks);
    if let Some(count) = selected_track.new_rejected_count {
        solving_shape.current_track_rejected_count = count;
    }

    match selected_track.new_track {
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
            solving_shape.current_track_last_frame_id = Some(track.frame_id());
            solving_shape.last_cursor = Some(next_cursor);

            #[cfg(debug_assertions)]
            #[cfg(feature = "debug_transparent_shape")]
            debug_transparent_shapes(resources, solving_shape, &tracks);
        }
        None => {
            let Some(last_cursor) = solving_shape.last_cursor else {
                return;
            };
            let scaled = solving_shape.bg_direction * 4.0;
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
    solving_shape: &SolvingShape,
    current_frame_id: u64,
    tracks: &'a [STrack],
) -> SelectedTrack<'a> {
    /// Threshold at which to reject the current track.
    ///
    /// It is currently chosen to account for background direction estimation can be unstable.
    const REJECT_COUNT_THRESHOLD: u32 = 5;

    /// The threshold for motion score.
    ///
    /// The score is currently the negative of the dot product of the background direction and
    /// the track motion direction. The true transparent object moves in the
    /// opposite background direction.
    const MOTION_SCORE_THRESHOLD: f64 = 0.2;

    /// Frames to wait before rejecting the current lost track.
    const TRACK_AGE_THRESHOLD: u64 = 2;

    let Some(current_track_id) = solving_shape.current_track_id else {
        return SelectedTrack {
            new_track: None,
            new_rejected_count: None,
        };
    };
    let bg_direction = solving_shape.bg_direction;
    let current_track = tracks
        .iter()
        .find(|track| track.track_id() == current_track_id);
    if let Some(track) = current_track {
        let score = motion_background_score(track, bg_direction);
        let above_threshold = score >= MOTION_SCORE_THRESHOLD;
        let count = if above_threshold {
            0
        } else {
            solving_shape.current_track_rejected_count + 1
        };

        if count > REJECT_COUNT_THRESHOLD {
            debug!(target: "player", "rejecting current track {current_track_id} with score {score:.2})");
        }

        if count <= REJECT_COUNT_THRESHOLD {
            return SelectedTrack {
                new_track: Some(track),
                new_rejected_count: Some(count),
            };
        }
    }

    let last_frame_id = solving_shape
        .current_track_last_frame_id
        .expect("set if id is set");
    let age = current_frame_id - last_frame_id;
    if age <= TRACK_AGE_THRESHOLD {
        return SelectedTrack {
            new_track: None,
            new_rejected_count: None,
        };
    }

    let scored_track = tracks
        .iter()
        .filter(|track| track.track_id() != current_track_id)
        .map(|track| (track, motion_background_score(track, bg_direction)))
        .filter(|(_, score)| *score >= MOTION_SCORE_THRESHOLD)
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(track, _)| track);

    SelectedTrack {
        new_track: scored_track,
        new_rejected_count: Some(0),
    }
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

fn motion_background_score(track: &STrack, bg_direction: Point2d) -> f64 {
    -track_motion(track).dot(bg_direction)
}

fn estimate_background_direction(tracks: &[STrack]) -> Option<Point2d> {
    let motions = tracks
        .iter()
        .filter(|track| track_velocity(track).norm() >= 1.0) // Ignore nearly-static tracks
        .map(track_motion)
        .collect::<Vec<Point2d>>();
    if motions.len() < 3 {
        return None;
    }

    let mut sum = Point2d::new(0.0, 0.0);
    for motion in motions {
        sum += motion;
    }

    let norm = sum.norm();
    if norm < 1e-3 {
        return None;
    }

    Some(sum / norm)
}

fn track_velocity(track: &STrack) -> Point2d {
    let (vx, vy) = track.kalman_velocity();
    Point2d::new(vx as f64, vy as f64)
}

fn track_motion(track: &STrack) -> Point2d {
    let v = track_velocity(track);
    let norm = v.norm();

    if norm < 1e-3 {
        Point2d::new(0.0, 0.0)
    } else {
        v / norm
    }
}
