use std::{cmp::Ordering, fmt::Display};

use super::point::Point;

#[derive(Clone)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

pub enum Position {
    Start,
    Contains,
    End,
    Outside,
}

impl Segment {
    /// It is not important the order of p1 and p2
    pub fn new(p1: impl Into<Point>, p2: impl Into<Point>) -> Self {
        let start: Point = p1.into();
        let end: Point = p2.into();

        match start.sweep_plane_cmp(&end) {
            Ordering::Less | Ordering::Equal => Self { start, end },
            Ordering::Greater => Self {
                start: end,
                end: start,
            },
        }
    }

    pub fn upper_limit(&self, p: &Point) -> bool {
        return *p == self.start;
    }

    pub fn lower_limit(&self, p: &Point) -> bool {
        return *p == self.end;
    }

    pub fn contains(&self, p: &Point) -> bool {
        f64::min(self.start.x, self.end.x) <= p.x
            && p.x <= f64::max(self.start.x, self.end.x)
            && f64::min(self.start.y, self.end.y) <= p.y
            && p.y <= f64::max(self.start.y, self.end.y)
    }

    pub fn point_position(&self, p: &Point) -> Position {
        if *p == self.start {
            return Position::Start;
        }

        if *p == self.end {
            return Position::End;
        }

        if self.contains(p) {
            return Position::Contains;
        }

        Position::Outside
    }

    pub fn find_interpolation(s1: &Segment, s2: &Segment) -> Option<Point> {
        let a1 = s1.end.y - s1.start.y;
        let b1 = s1.start.x - s1.end.x;
        let c1 = a1 * s1.start.x + b1 * s1.start.y;

        let a2 = s2.end.y - s2.start.y;
        let b2 = s2.start.x - s2.end.x;
        let c2 = a2 * s2.start.x + b2 * s2.start.y;

        let denominator = a1 * b2 - a2 * b1;
        if denominator == 0.0 {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / denominator;
        let y = (a1 * c2 - a2 * c1) / denominator;

        let interpolation_point = Point::from2d(x, y);
        if s1.contains(&interpolation_point) == false || s2.contains(&interpolation_point) == false
        {
            return None;
        }

        Some(interpolation_point)
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.sweep_plane_cmp(&other.start) {
            core::cmp::Ordering::Equal => {}
            ord => return Some(ord),
        }
        Some(self.end.sweep_plane_cmp(&other.end))
    }
}

impl Eq for Segment {}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.sweep_plane_cmp(&other.start) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.end.sweep_plane_cmp(&other.end)
    }
}

impl From<Segment> for [Point; 2] {
    fn from(s: Segment) -> Self {
        [s.start, s.end]
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;

    use super::Segment;

    #[test]
    fn new_segment() {
        let s = Segment::new([2.0, 2.0], [4.0, 4.0]);

        assert_eq!(s.start, [4.0, 4.0].into());
        assert_eq!(s.end, [2.0, 2.0].into());
    }

    #[test]
    fn new_segment_reverse() {
        let s = Segment::new([4.0, 4.0], [2.0, 2.0]);

        assert_eq!(s.start, [4.0, 4.0].into());
        assert_eq!(s.end, [2.0, 2.0].into());
    }

    #[test]
    fn partial_eq_segment() {
        //todo!()
    }

    #[test]
    fn segment_contains_point() {
        let s = Segment::new([0.0, 0.0], [2.0, 2.0]);
        let p = Point::from2d(2.0, 2.0);

        assert!(s.contains(&p));
    }

    #[test]
    fn segment_constains_point_limits() {
        let s = Segment::new([0.0, 4.0], [2.0, 2.0]);
        let p = Point::from2d(2.0, 2.0);

        assert!(s.contains(&p));
    }

    #[test]
    fn segment_doesnt_contains_point() {
        let s = Segment::new([0.0, 0.0], [2.0, 2.0]);
        let p = Point::from2d(3.0, 3.0);

        assert_eq!(s.contains(&p), false);
    }

    #[test]
    fn find_interpolation() {
        let s1 = Segment::new([0.0, -2.0], [0.0, 2.0]);
        let s2 = Segment::new([-2.0, 0.0], [2.0, 0.0]);

        let opt_point = Segment::find_interpolation(&s1, &s2);
        assert!(opt_point.is_some());

        let point = opt_point.unwrap();
        assert_eq!(point, Point::from2d(0.0, 0.0));
    }

    #[test]
    fn find_interpolation_parallel() {
        let s1 = Segment::new([0.0, 0.0], [4.0, 0.0]);
        let s2 = Segment::new([2.0, 0.0], [4.0, 0.0]);

        assert_eq!(Segment::find_interpolation(&s1, &s2), None)
    }

    #[test]
    fn find_interpolation_incident_none() {
        let s1 = Segment::new([0.0, 0.0], [4.0, 4.0]);
        let s2 = Segment::new([0.0, 4.0], [1.0, 3.0]);

        assert_eq!(Segment::find_interpolation(&s1, &s2), None)
    }

    #[test]
    fn find_interpolation_incident() {
        let s1 = Segment::new([0.0, 0.0], [4.0, 4.0]);
        let s2 = Segment::new([0.0, 4.0], [2.0, 2.0]);

        assert!(s1.contains(&Point::from2d(2.0, 2.0)));
        assert!(s2.contains(&Point::from2d(2.0, 2.0)));
        assert_eq!(
            Segment::find_interpolation(&s1, &s2).unwrap(),
            Point::from2d(2.0, 2.0)
        );
    }

    #[test]
    fn display_segment() {
        let s = Segment::new([0.0, 0.0], [4.0, 4.0]);
        assert_eq!(format!("{}", s), "[(4.0, 4.0), (0.0, 0.0)]");
    }

    #[test]
    fn equals() {
        let s1 = Segment::new([4.2, 7.2], [4.0, 4.0]);
        let s2 = Segment::new([4.2, 7.2], [4.0, 4.0]);

        assert_eq!(s1.eq(&s2), true);
    }

    #[test]
    fn segment_into_point_2() {
        let seg: [Point; 2] = Segment::new([2.0, 3.0], [4.0, 5.0]).into();

        assert_eq!(seg[0].x, 4.0);
        assert_eq!(seg[0].y, 5.0);

        assert_eq!(seg[1].x, 2.0);
        assert_eq!(seg[1].y, 3.0);
    }
}
