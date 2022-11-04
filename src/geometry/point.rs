use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Range, Sub},
};

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.sweep_plane_cmp(other)
    }
}

impl From<[f64; 2]> for Point {
    fn from(p: [f64; 2]) -> Self {
        Self::from2d(p[0], p[1])
    }
}

impl From<Point> for [f64; 2] {
    fn from(p: Point) -> Self {
        [p.x, p.y]
    }
}

impl From<Point> for [f64; 3] {
    fn from(p: Point) -> Self {
        [p.x, p.y, p.z]
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

impl Point {
    /// create new Point starting from (x, y, z) coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn random(r: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(r.clone()),
            y: rng.gen_range(r.clone()),
            z: rng.gen_range(r.clone()),
        }
    }

    pub fn lexicograph_cmp(&self, p: &Point) -> Ordering {
        self.partial_cmp(&p).unwrap()
    }

    pub fn sweep_plane_cmp(&self, p: &Point) -> Ordering {
        match self.y.partial_cmp(&p.y) {
            // self.y < p.y
            Some(Ordering::Less) => Ordering::Greater,
            // self.y > p.y
            Some(Ordering::Greater) => Ordering::Less,
            // self.y  == p.y && self.x < p.x
            Some(Ordering::Equal) => self.x.partial_cmp(&p.x).unwrap(),
            // NaN value
            None => panic!("Comparision is impossible!"),
        }
    }

    /// create new Point starting from (x, y) coordinates
    pub fn from2d(x: f64, y: f64) -> Self {
        Point { x, y, z: 0.0 }
    }

    /// distance from p2
    pub fn distance_from(&self, p2: &Point) -> f64 {
        ((p2.x - self.x).powi(2) + (p2.y - self.y).powi(2) + (p2.z - self.z).powi(2)).sqrt()
    }

    pub fn distance(p1: &Point, p2: &Point) -> f64 {
        p1.distance_from(p2)
    }

    /// Given 3 Points it returns true if they make a right turn, false otherwise
    pub fn make_right_turn(p1: &Point, p2: &Point, p3: &Point) -> bool {
        Point::clockwise(&(*p3 - *p1), &(*p2 - *p1))
    }

    /// cross product p1 x p2 > 0
    pub fn clockwise(p1: &Point, p2: &Point) -> bool {
        (p1.x * p2.y - p1.y * p2.x) > 0.0
    }

    pub fn anti_clockwise(p1: &Point, p2: &Point) -> bool {
        !Point::clockwise(p1, p2)
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::point::Point;
    use std::cmp::Ordering;

    #[test]
    fn new_point() {
        let p = Point::new(5.0, 5.0, 5.0);

        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 5.0);
        assert_eq!(p.z, 5.0);
    }

    #[test]
    fn sub_point() {
        let p1 = Point::new(5.0, 5.0, 5.0);
        let p2 = Point::new(6.0, 5.0, 5.0);
        let p3 = p2 - p1;

        assert_eq!(p3.x, 1.0);
        assert_eq!(p3.y, 0.0);
        assert_eq!(p3.z, 0.0);
    }

    #[test]
    fn distance_from() {
        let p1 = Point::new(5.0, 5.0, 5.0);
        let p2 = Point::new(6.0, 5.0, 5.0);

        let distance = p1.distance_from(&p2);
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn distance_from_2d() {
        let p1 = Point::from2d(0.0, 0.0);
        let p2 = Point::from2d(3.0, 4.0);

        let distance = p1.distance_from(&p2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn get_distance() {
        let p1 = Point::from2d(0.0, 0.0);
        let p2 = Point::from2d(3.0, 4.0);

        let distance = Point::distance(&p1, &p2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_lexicographic_order() {
        let mut p1 = Point::from2d(2.0, 4.0);
        let mut p2 = Point::from2d(3.0, 0.0);

        assert!(p1 < p2);

        p1 = Point::from2d(3.0, 0.0);
        p2 = Point::from2d(3.0, 4.0);

        assert!(p1 < p2);

        p1 = Point::from2d(3.0, 4.0);
        p2 = Point::from2d(3.0, 0.0);

        assert!(p1 > p2);
    }

    #[test]
    fn clockwise() {
        let p1 = Point::from2d(3.0, 2.0);
        let p2 = Point::from2d(2.0, 3.0);

        assert!(Point::clockwise(&p1, &p2));
    }

    #[test]
    fn anti_clockwise() {
        let p1 = Point::from2d(1.0, 3.0);
        let p2 = Point::from2d(3.0, 2.0);

        assert!(Point::anti_clockwise(&p1, &p2));
    }

    #[test]
    fn make_right_turn_false() {
        let p1 = Point::from2d(0.0, 0.0);
        let p2 = Point::from2d(0.5, 0.5);
        let p3 = Point::from2d(0.0, 1.0);

        assert_eq!(Point::make_right_turn(&p1, &p2, &p3), false);
    }

    #[test]
    fn make_right_turn_true() {
        let p1 = Point::from2d(0.0, 0.0);
        let p2 = Point::from2d(1.0, 1.0);
        let p3 = Point::from2d(2.0, 0.0);

        assert_eq!(Point::make_right_turn(&p1, &p2, &p3), true);
    }

    #[test]
    fn sweep_line_cmp_points() {
        let p1 = Point::from2d(0.0, 0.0);
        let p2 = Point::from2d(1.0, 1.0);

        assert_eq!(p1.cmp(&p2), Ordering::Greater);
    }

    #[test]
    fn f64_2_from_point() {
        let array_point: [f64; 2] = Point::from2d(2.0, 3.0).into();

        assert_eq!(array_point[0], 2.0);
        assert_eq!(array_point[1], 3.0);
    }

    #[test]
    fn f64_3_from_point() {
        let array_point: [f64; 3] = Point::new(2.0, 3.0, 4.0).into();

        assert_eq!(array_point[0], 2.0);
        assert_eq!(array_point[1], 3.0);
        assert_eq!(array_point[2], 4.0);
    }

    #[test]
    fn display_point() {
        let p = Point::from2d(2.0, 2.0);

        assert_eq!(format! {"{}", p}, "(2.0, 2.0)");
    }

    #[test]
    fn panic_comparision() {
        let p1 = Point::from2d(f64::NAN, 1.0);
        let p2 = Point::from2d(f64::NAN, 1.0);

        let result = std::panic::catch_unwind(|| p1.lexicograph_cmp(&p2));
        assert!(result.is_err());
    }
}
