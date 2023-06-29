use std::collections::{BTreeMap, BTreeSet};

use crate::geometry::{point::Point, segment::Segment};

#[derive(PartialEq)]
pub struct SweepPoint(pub Point);

impl PartialOrd for SweepPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.sweep_plane_cmp(&other.0)
    }
}

impl Eq for SweepPoint {}

impl Ord for SweepPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct SweepLine {
    // this queue contains as K the event Point and as value the U(p)
    pub queue: BTreeMap<SweepPoint, Vec<Segment>>,

    pub status: BTreeSet<Segment>,

    // for render
    pub segments: Vec<Segment>,
    pub event_point: Point,
    pub u_p: Vec<Segment>,
    pub l_p: Vec<Segment>,
    pub c_p: Vec<Segment>,
}

#[cfg(test)]
mod tests {

    use super::SweepPoint;

    #[test]
    fn sweep_point_ordering() {
        let p1 = SweepPoint([1.0, 2.0].into());
        let p2 = SweepPoint([2.0, 1.0].into());
        let p3 = SweepPoint([3.0, 2.0].into());
        let p4 = SweepPoint([1.0, 2.0].into());

        assert!(p1 < p2);
        assert!(p2 > p1);
        assert!(p1 < p3);
        assert!(p3 > p1);
        assert!(p1 == p4);
        assert!(p4 == p1);
    }

    #[test]
    fn test_queue() {
        let p1 = SweepPoint([1.0, 3.0].into());
        let p2 = SweepPoint([3.0, 1.0].into());
        let p3 = SweepPoint([1.0, 1.0].into());
        let p4 = SweepPoint([3.0, 3.0].into());

        assert!(p1 < p4);
        assert!(p4 < p3);
        assert!(p3 < p2);
    }
}
