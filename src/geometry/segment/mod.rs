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

pub mod segment_impl;

mod tests;
