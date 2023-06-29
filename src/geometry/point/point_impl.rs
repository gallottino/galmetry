use super::Point;
use crate::geometry::common;
use rand::Rng;
use std::{cmp::Ordering, ops::Range};
use std::{fmt::Display, ops::Sub};

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn from2d(x: f64, y: f64) -> Self {
        Point { x, y, z: 0.0 }
    }

    pub fn random_vec(capacity: usize, r: Range<f64>) -> Vec<Point> {
        let mut random_points = Vec::new();
        for _i in 0..capacity {
            random_points.push(Point::random(r.clone()));
        }
        random_points
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
        match self.eq(p) {
            true => Ordering::Equal,
            _ => {
                if self.x < p.x {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
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

    pub fn distance_from(&self, p2: &Point) -> f64 {
        ((p2.x - self.x).powi(2) + (p2.y - self.y).powi(2) + (p2.z - self.z).powi(2)).sqrt()
    }

    pub fn distance(p1: &Point, p2: &Point) -> f64 {
        p1.distance_from(p2)
    }

    pub fn make_right_turn(p1: &Point, p2: &Point, p3: &Point) -> bool {
        Point::clockwise(&(*p3 - *p1), &(*p2 - *p1))
    }

    /// cross product p1 x p2 > 0
    pub fn clockwise(p1: &Point, p2: &Point) -> bool {
        (p1.x * p2.y - p1.y * p2.x) > 0.0
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        f64::abs(self.x - other.x) <= common::DELTA
            && f64::abs(self.y - other.y) <= common::DELTA
            && f64::abs(self.z - other.z) <= common::DELTA
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}
