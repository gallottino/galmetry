use super::{Position, Segment};
use crate::geometry::point::Point;
use std::{cmp::Ordering, fmt::Display, ops::Range};

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

    pub fn random(r: Range<f64>) -> Self {
        let p1 = Point::random(r.clone());
        let p2 = Point::random(r.clone());
        Self::new(p1, p2)
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

    pub fn find_intersection(s1: &Segment, s2: &Segment) -> Option<Point> {
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
    use crate::geometry::{segment::Segment, point::Point};


    #[test]
    fn test_intersection() {
        let s1 = Segment::new([0.4248, 0.7734], [0.4525, 0.3885]);
        let s2 = Segment::new([0.1710, 0.7903], [0.2802, 0.7221]);

        let sweep_line = Segment::new([-1000.0, 0.7734],[1000.0, 0.7734]);

        let point_1 = Segment::find_intersection(&s1, &sweep_line);
        let point_2 = Segment::find_intersection(&s2, &sweep_line);

        let right_point_2 = Point::new(0.1980598240469, 0.7734,0.0);

        assert_eq!(right_point_2,point_2.unwrap());
        println!("p1:{}, p2:{}", point_1.unwrap(), point_2.unwrap());
    }
}