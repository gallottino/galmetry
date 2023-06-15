/// A rappresentation of a point in a three-dimensional space
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub mod point_impl;

mod tests;
