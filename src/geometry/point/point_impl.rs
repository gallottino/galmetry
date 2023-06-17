
use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Range, Sub},
};

use rand::Rng;

use super::Point;

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.lexicograph_cmp(other)
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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.z.partial_cmp(&other.z)
    }
}


impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.8}, {:.8})", self.x, self.y)
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
