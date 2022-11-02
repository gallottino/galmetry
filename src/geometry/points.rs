use std::ops::{Index, IndexMut, Range};

use super::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct Points(Vec<Point>);
impl Points {
    pub fn new() -> Self {
        Self { 0: vec![] }
    }

    pub fn from(data: impl Into<Vec<Point>>) -> Self {
        Self { 0: data.into() }
    }

    pub fn random(capacity: u32, r: Range<f64>) -> Self {
        let mut random_points = Self::new();
        for _i in 0..capacity {
            random_points.0.push(Point::random(r.clone()));
        }
        random_points
    }

    pub fn lexicograph_sort(&mut self) {
        self.0.sort()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, p: impl Into<Point>) {
        self.0.push(p.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn append(&mut self, points: &mut Points) {
        self.0.append(&mut points.0);
    }
}

impl Index<usize> for Points {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Points {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Points> for Vec<[f64; 2]> {
    fn from(points: Points) -> Self {
        points.0.iter().map(|p| (*p).into()).collect()
    }
}

impl From<Vec<[f64; 2]>> for Points {
    fn from(points: Vec<[f64; 2]>) -> Self {
        Self::from(
            points
                .iter()
                .map(|array_p| {
                    let p: Point = (*array_p).into();
                    p
                })
                .collect::<Vec<Point>>(),
        )
    }
}

impl From<Points> for Vec<[f64; 3]> {
    fn from(points: Points) -> Self {
        points.0.iter().map(|p| (*p).into()).collect()
    }
}

impl From<Points> for Vec<Point> {
    fn from(points: Points) -> Self {
        points.0
    }
}

#[cfg(test)]
mod tests {}
