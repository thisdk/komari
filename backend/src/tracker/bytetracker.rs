use crate::tracker::{
    Detection,
    strack::{STrack, TrackState},
    tlwh_to_xyah,
};

/// An extended [BYTETracker] implementation by GPT-5.
///
/// [BYTETracker]: https://github.com/ultralytics/ultralytics/blob/004d9730060e560c86ad79aaa1ab97167443be25/ultralytics/trackers/byte_tracker.py#L231
#[derive(Debug)]
pub struct ByteTracker {
    tracked: Vec<STrack>,
    lost: Vec<STrack>,
    frame_id: u64,
    max_time_lost: u64,
}

impl ByteTracker {
    pub fn new(frame_rate: u32) -> Self {
        Self {
            tracked: Vec::new(),
            lost: Vec::new(),
            frame_id: 0,
            max_time_lost: frame_rate as u64,
        }
    }

    pub fn frame_id(&self) -> u64 {
        self.frame_id
    }

    pub fn update(&mut self, detections: Vec<Detection>) -> Vec<STrack> {
        self.frame_id += 1;

        // 1. Predict all tracks
        for track in &mut self.tracked {
            track.predict();
        }
        for track in &mut self.lost {
            track.predict();
        }

        // 2. Convert detections to STrack (unactivated)
        let detection_tracks: Vec<STrack> = detections
            .into_iter()
            .map(|d| STrack::new(d.bbox))
            .collect();
        if self.tracked.is_empty() && self.lost.is_empty() {
            self.tracked = detection_tracks
                .into_iter()
                .map(|mut track| {
                    track.activate(self.frame_id);
                    track
                })
                .collect();
            return self.tracked.clone();
        }

        // 3. Match `tracked` and `lost` to detections
        let mut current_tracks = Vec::new();
        current_tracks.append(&mut self.tracked);
        current_tracks.append(&mut self.lost);

        let cost = iou_distance(&current_tracks, &detection_tracks);
        let (matches, unmatched_tracks, unmatched_detections) = linear_assignment(cost, 0.5);

        let mut activated = Vec::new();
        let mut reactivated = Vec::new();
        let mut lost = Vec::new();

        // 4. Update matched tracks
        for (ci, di) in matches {
            let mut track = current_tracks[ci].clone();
            let det = &detection_tracks[di];

            if track.state == TrackState::Tracked {
                track.update(det.tlwh, self.frame_id);
                activated.push(track);
            } else {
                track.reactivate(det.tlwh, self.frame_id);
                reactivated.push(track);
            }
        }

        // 5. Unmatched tracks to `lost`
        for ci in unmatched_tracks {
            let mut track = current_tracks[ci].clone();
            track.mark_lost();
            lost.push(track);
        }

        // 6. New tracks from unmatched detections
        for di in unmatched_detections {
            let mut track = detection_tracks[di].clone();
            track.activate(self.frame_id);
            activated.push(track);
        }

        // 7. Update state lists
        self.tracked = activated;
        self.tracked.extend(reactivated);
        self.lost = lost
            .into_iter()
            .filter(|track| self.frame_id - track.frame_id <= self.max_time_lost)
            .collect();

        self.tracked.clone()
    }
}

fn iou_tlwh(a: [f32; 4], b: [f32; 4]) -> f32 {
    let ax1 = a[0];
    let ay1 = a[1];
    let ax2 = a[0] + a[2];
    let ay2 = a[1] + a[3];

    let bx1 = b[0];
    let by1 = b[1];
    let bx2 = b[0] + b[2];
    let by2 = b[1] + b[3];

    let inter_x1 = ax1.max(bx1);
    let inter_y1 = ay1.max(by1);
    let inter_x2 = ax2.min(bx2);
    let inter_y2 = ay2.min(by2);

    let inter_w = (inter_x2 - inter_x1).max(0.0);
    let inter_h = (inter_y2 - inter_y1).max(0.0);
    let inter_area = inter_w * inter_h;

    let area_a = a[2] * a[3];
    let area_b = b[2] * b[3];

    inter_area / (area_a + area_b - inter_area + 1e-6)
}

fn iou_distance(tracks: &[STrack], detections: &[STrack]) -> Vec<Vec<f32>> {
    const GATING_THRESHOLD: f32 = 9.4877;

    let mut cost = vec![vec![0.0; detections.len()]; tracks.len()];

    for (i, t) in tracks.iter().enumerate() {
        for (j, d) in detections.iter().enumerate() {
            let meas = tlwh_to_xyah(d.tlwh);
            let gate = t.kalman.gating_distance(meas);

            if gate > GATING_THRESHOLD {
                cost[i][j] = 1e6; // forbid
            } else {
                cost[i][j] = 1.0 - iou_tlwh(t.kalman_tlwh(), d.tlwh);
            }
        }
    }

    cost
}

fn linear_assignment(
    costs: Vec<Vec<f32>>,
    thresh: f32,
) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    use lapjv::{Matrix, lapjv};

    let n = costs.len();
    let m = if n > 0 { costs[0].len() } else { 0 };
    if n == 0 || m == 0 {
        return (vec![], (0..n).collect(), vec![]);
    }

    let k = n.max(m);
    let mut data = vec![1_000_000.0; k * k];
    for i in 0..n {
        for j in 0..m {
            data[i * k + j] = costs[i][j];
        }
    }

    let mat = Matrix::from_shape_vec((k, k), data).unwrap();
    let (x, _) = lapjv(&mat).expect("lapjv failed");

    let mut matches = Vec::new();
    let mut unmatched_a = Vec::new();
    let mut unmatched_b = vec![true; m];

    for i in 0..n {
        let j = x[i];
        if j < m && costs[i][j] <= thresh {
            matches.push((i, j));
            unmatched_b[j] = false;
        } else {
            unmatched_a.push(i);
        }
    }

    let unmatched_b: Vec<usize> = unmatched_b
        .iter()
        .enumerate()
        .filter_map(|(j, &u)| if u { Some(j) } else { None })
        .collect();

    (matches, unmatched_a, unmatched_b)
}
