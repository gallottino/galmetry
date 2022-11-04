use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::geometry::{
    point::Point,
    segment::{Position, Segment},
};

use super::algorithm::Algorithm;

pub struct SweepPlane {
    segments: BTreeSet<Segment>,
    queue: BTreeMap<Point, (BTreeSet<Segment>, BTreeSet<Segment>, BTreeSet<Segment>)>,
    status: BTreeSet<Segment>,
}

impl Algorithm for SweepPlane {
    /// list of intersection points with segments that contain it
    type Output = BTreeSet<Point>;

    fn calculate(&mut self) -> Self::Output {
        let mut intersection_points = BTreeSet::new();

        while let Some(s) = self.segments.pop_first() {
            self.add_event_queue(s.start, s.clone());
        }

        while self.queue.is_empty() == false {
            self.handle_event_point(&mut intersection_points);
        }

        intersection_points
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl SweepPlane {
    pub fn build(segments: BTreeSet<Segment>) -> Self {
        Self {
            segments,
            queue: BTreeMap::new(),
            status: BTreeSet::new(),
        }
    }

    pub fn random(capacity: usize) -> Self {
        let mut random_segments = BTreeSet::<Segment>::new();
        for _i in 0..capacity {
            random_segments.insert(Segment::random(0.1..0.9));
        }
        Self::build(random_segments)
    }

    fn handle_event_point(&mut self, intersection_points: &mut BTreeSet<Point>) {
        let (event_point, (upper, contains, lower)) = self.queue.pop_first().unwrap();

        lower.union(&contains).for_each(|s| {
            self.status.remove(s);
        });

        upper.union(&contains).for_each(|s| {
            self.status.insert(s.clone());
        });

        if self.status.len() > 1 {
            let mut left = self.status.first().unwrap().clone();
            let mut right = self.status.first().unwrap().clone();
            for s in &self.status {
                match s.start.cmp(&event_point) {
                    std::cmp::Ordering::Less => left = s.clone(),
                    _ => {
                        right = s.clone();
                        break;
                    }
                }
            }

            if upper.len() == 0 && contains.len() == 0 {
                self.find_new_event(
                    left.clone(),
                    right.clone(),
                    event_point,
                    intersection_points,
                );
            } else {
                let leftmost = upper.union(&contains).min().unwrap();
                self.find_new_event(
                    left.clone(),
                    leftmost.clone(),
                    event_point,
                    intersection_points,
                );

                let rightmost = upper.union(&contains).max().unwrap();
                self.find_new_event(
                    right.clone(),
                    rightmost.clone(),
                    event_point,
                    intersection_points,
                );
            }
        }
    }

    fn find_new_event(
        &mut self,
        segment_left: Segment,
        segment_right: Segment,
        event_point: Point,
        interpolation_point: &mut BTreeSet<Point>,
    ) {
        match Segment::find_interpolation(&segment_left, &segment_right) {
            Some(p) => {
                if interpolation_point.contains(&p) == false {
                    interpolation_point.insert(p);
                }

                if p.y < event_point.y && self.queue.contains_key(&p) == false {
                    self.add_event_queue(p, segment_left.clone());
                    self.add_event_queue(p, segment_right.clone());
                }
            }
            None => {}
        }
    }

    fn add_event_queue(&mut self, p: Point, s: Segment) {
        let (mut upper, mut contains, mut lower) = match self.queue.get(&p) {
            Some(values) => values.clone(),
            None => (BTreeSet::new(), BTreeSet::new(), BTreeSet::new()),
        };

        match s.point_position(&p) {
            Position::Start => {
                upper.insert(s.clone());
            }
            Position::End => {
                lower.insert(s.clone());
            }
            Position::Contains => {
                contains.insert(s.clone());
            }
            Position::Outside => {}
        }

        self.queue.insert(p, (upper, contains, lower));
        self.status.insert(s);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{
        algorithms::algorithm::Algorithm,
        geometry::{point::Point, points::Points, segment::Segment},
    };

    use super::SweepPlane;

    #[test]
    fn test_sweep_plane() {
        let s1 = Segment::new([4.0, 4.0], [0.0, 0.0]);
        let s2 = Segment::new([0.0, 2.0], [2.0, 0.0]);

        let mut segments = BTreeSet::<Segment>::new();
        segments.insert(s1);
        segments.insert(s2);

        let mut algo = SweepPlane::build(segments);
        let res: Vec<Point> = algo.calculate().into_iter().collect();

        let assert_res: Points = vec![[1.0, 1.0]].into();
        for p in assert_res {
            assert!(res.contains(&p));
        }
    }

    #[test]
    fn test_sweep_plane_2() {
        let s1 = Segment::new([4.0, 4.0], [0.0, 0.0]);
        let s2 = Segment::new([0.0, 2.0], [2.0, 0.0]);
        let s3 = Segment::new([0.0, 4.0], [4.0, 0.0]);
        let s4 = Segment::new([1.0, -1.0], [3.0, 1.0]);

        let mut segments = BTreeSet::<Segment>::new();
        segments.insert(s1);
        segments.insert(s2);
        segments.insert(s3);
        segments.insert(s4);

        let mut algo = SweepPlane::build(segments);
        let res = algo.calculate();

        let assert_res: Points = vec![[1.0, 1.0], [2.0, 2.0], [3.0, 1.0], [2.0, 0.0]].into();
        for p in assert_res {
            assert!(res.contains(&p));
        }
    }
}
