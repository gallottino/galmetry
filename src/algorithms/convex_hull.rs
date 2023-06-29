use crate::geometry::point::Point;

use super::algorithm::Algorithm;

pub struct MonotoneConvexHull {
    pub points: Vec<Point>,
    pub convex_hull: Vec<Point>,
    step: u32,
}

impl Algorithm for MonotoneConvexHull {
    type Output = Vec<Point>;

    fn calculate(&mut self) -> Self::Output {
        let mut upper_l: Vec<Point> = Vec::new();
        let mut lower_l = Vec::new();

        self.points.sort_by(|p1, p2| p1.lexicograph_cmp(p2));
        let vec_points = self.points.clone();

        upper_l.push(vec_points[0]);
        upper_l.push(vec_points[1]);
        for i in 0..self.points.len() {
            upper_l.push(vec_points[i]);
            upper_l = self.assure_make_turn_right(upper_l);
        }
        upper_l.pop();

        lower_l.push(vec_points[vec_points.len() - 1]);
        lower_l.push(vec_points[vec_points.len() - 2]);
        for i in (0..(vec_points.len() - 2)).rev() {
            lower_l.push(vec_points[i]);
            lower_l = self.assure_make_turn_right(lower_l);
        }
        lower_l.pop();

        let mut convex_hull = Vec::new();
        convex_hull.append(&mut upper_l);
        convex_hull.append(&mut lower_l);

        convex_hull
    }

    fn step(&mut self) {
        todo!();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.convex_hull = Vec::new();
    }
}

impl MonotoneConvexHull {
    pub fn build(points: Vec<Point>) -> Self {
        Self {
            points,
            convex_hull: Vec::new(),
            step: 0,
        }
    }

    /// this method takes a list of point and it assure that they turn right
    fn assure_make_turn_right(&self, mut half_l: Vec<Point>) -> Vec<Point> {
        let mut half_l_len = half_l.len();
        while half_l_len > 2
            && !Point::make_right_turn(
                &half_l[half_l_len - 3],
                &half_l[half_l_len - 2],
                &half_l[half_l_len - 1],
            )
        {
            half_l[half_l_len - 2] = half_l[half_l_len - 1];
            half_l.pop();
            half_l_len = half_l.len();
        }
        half_l
    }
}
