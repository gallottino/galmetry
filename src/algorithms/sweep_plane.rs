use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use crate::geometry::{point::Point, segment::Segment};
use std::ops::Bound::{Excluded, Unbounded};
use super::algorithm::Algorithm;

#[derive(PartialEq, Eq, Clone)]
struct StatusValue(RefCell<Point>, Segment);

impl PartialOrd for StatusValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let event_point = self.0.borrow();
        let sweep_line = Segment::new([-1000.0, event_point.y], [1000.0, event_point.y]);

        println!("{} with {}", self.1, other.1);

        if self.1 == other.1 { return Some(std::cmp::Ordering::Equal) };

        let self_intersection = Segment::find_intersection(&self.1, &sweep_line).unwrap();
        let other_intersection = Segment::find_intersection(&other.1, &sweep_line).unwrap();

        Some(self_intersection.sweep_plane_cmp(&other_intersection))
    }
}

impl Ord for StatusValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct SweepPlane {
    // this queue contains as K the event Point and as value the U(p)
    queue: BTreeMap<Point, BTreeSet<Segment>>,

    // current status of the algorithm
    status: BTreeSet<StatusValue>,

    // the list of segments used by the algorithm
    segments: BTreeSet<Segment>,

    // the output of the algorithm
    intersections: BTreeSet<Point>,

    // the reference of the current event_point
    event_point: Point,
}

impl Algorithm for SweepPlane {
    type Output = BTreeSet<Point>;

    fn calculate(&mut self) -> Self::Output {
        while let Some(segment) = self.segments.pop_first() {
            let mut l_p = BTreeSet::<Segment>::new();
            l_p.insert(segment.clone());
            self.queue.insert(segment.start.clone(), l_p);
            self.queue.insert(segment.end.clone(), BTreeSet::<Segment>::new());
        }

        while self.queue.is_empty() == false {
            self.step();
        }

        self.intersections.clone()
    }

    fn step(&mut self) {
        self.handle_event_point();
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl SweepPlane {
    pub fn build(segments: Vec<Segment>) -> Self {
        let mut segs = BTreeSet::<Segment>::new();
        for seg in segments {
            segs.insert(seg);
        }

        Self {
            segments: segs,
            event_point: Point::random(0.0..1.0),
            queue: BTreeMap::new(),
            status: BTreeSet::new(),
            intersections: BTreeSet::new(),
        }
    }

    fn handle_event_point(&mut self) {

        let (event_point, u_p) = self.queue.pop_first().unwrap();
        let (c_p, l_p) = self.get_contains_and_lower(&event_point);

        self.event_point = event_point;

        println!("{}", event_point);
        if u_p.len() + c_p.len() + l_p.len() > 1 {
            self.intersections.insert(event_point.clone());
        }

        for seg in l_p.union(&c_p) {
            self.status
                .remove(&StatusValue(RefCell::new(self.event_point), seg.clone()));
        }

        for seg in u_p.union(&c_p) {
            self.status
                .insert(StatusValue(RefCell::new(self.event_point), seg.clone()));
        }

        if u_p.len() + c_p.len() == 0 {

            let status = self.status.clone();
            let left = status
                .range((
                    Unbounded,
                    Excluded(&StatusValue(
                        RefCell::new(self.event_point.clone()),
                        Segment::new(event_point, event_point),
                    )),
                ))
                .last()
                .unwrap();

            let right = status
                .range((
                    Excluded(&StatusValue(
                        RefCell::new(self.event_point.clone()),
                        Segment::new(event_point, event_point),
                    )),
                    Unbounded,
                ))
                .next()
                .unwrap();

            self.find_new_event(&left.1, &right.1, &event_point);
        } else {
            let status = self.status.clone();

            let mut union_u_c = u_p.union(&c_p);

            let leftmost = union_u_c.next().unwrap();
            let left_leftmost = status.range((
                Unbounded,
                Excluded(&StatusValue(
                        RefCell::new(self.event_point.clone()),
                        leftmost.clone(),
                    )))
            ).last();
            if left_leftmost.is_none() {return;}

            self.find_new_event(&left_leftmost.unwrap().1, leftmost, &event_point);

            let rightmost = union_u_c.last().unwrap_or(leftmost);
            let right_rightmost = status
                .range((
                    Excluded(&StatusValue(
                        RefCell::new(self.event_point.clone()),
                        rightmost.clone(),
                    )),
                    Unbounded,
                )).next();

            if right_rightmost.is_none() {return;}
            self.find_new_event(rightmost, &right_rightmost.unwrap().1, &event_point);
        }
    }

    fn get_contains_and_lower(&mut self, event_point: &Point) -> (BTreeSet<Segment>, BTreeSet<Segment>) {
        let mut c_p = BTreeSet::<Segment>::new();
        let mut l_p = BTreeSet::<Segment>::new();

        for seg in self.status.clone().into_iter() {
            if seg.1.end == *event_point {
                l_p.insert(seg.1.clone());
            }
            if seg.1.contains(event_point) {
                c_p.insert(seg.1.clone());
            }
        }

        (c_p, l_p)
    }

    fn find_new_event(&mut self, seg_left: &Segment, seg_right: &Segment, event_point: &Point) {
        match Segment::find_intersection(seg_left, seg_right) {
            Some(point) => {
                // intersect where?
                // below the seep line
                if point.y < event_point.y
                    // on the right of the event point
                    || point.x > event_point.x
                    // on the event point
                    || (point == event_point.clone())
                {
                    self.queue.entry(point).or_insert(BTreeSet::new());
                }
            }
            _ => return,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        algorithms::algorithm::Algorithm,
        geometry::{point::Point, points::Points, segment::Segment},
    };

    use super::SweepPlane;

    #[test]
    fn test_sweep_plane() {
        let s1 = Segment::new([4.0, 4.0], [0.0, 0.0]);
        let s2 = Segment::new([0.0, 2.0], [2.0, 0.0]);

        let mut segments = Vec::<Segment>::new();
        segments.push(s1);
        segments.push(s2);

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

        let segments = vec![s1,s2,s3,s4];

        let mut algo = SweepPlane::build(segments);
        let res = algo.calculate();

        let assert_res: Points = vec![[1.0, 1.0], [2.0, 2.0], [3.0, 1.0], [2.0, 0.0]].into();
        for p in assert_res {
            assert!(res.contains(&p));
        }
    }

    #[test]
    fn test_sweep_plane_3() {
        let s1 = Segment::new([0.85, 0.9], [0.8, 0.3]);
        let s2 = Segment::new([0.4, 0.8], [0.67, 0.68]);
        let s3 = Segment::new([0.81, 0.77], [0.82, 0.78]);
        let s4 = Segment::new([0.32, 0.74], [0.41, 0.51]);
        let s5 = Segment::new([0.75, 0.61], [0.36, 0.54]);

        let segments = vec![s1,s2,s3,s4,s5];

        let mut algo = SweepPlane::build(segments);
        let res = algo.calculate();
        for p in res.clone() {
            println!("{}", p)
        }
        assert_eq!(res.len(), 2);
    }
}
