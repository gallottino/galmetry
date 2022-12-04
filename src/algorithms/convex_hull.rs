use crate::geometry::{point::Point, points::Points};

use super::algorithm::Algorithm;

pub struct MonotoneConvexHull {
    points: Points,
}

impl Algorithm for MonotoneConvexHull {
    type Output = Points;

    fn calculate(&mut self) -> Self::Output {
        let mut upper_l = Points::new();
        let mut lower_l = Points::new();

        self.points.lexicograph_sort();

        upper_l.push(self.points[0]);
        upper_l.push(self.points[1]);
        for i in 0..self.points.len() {
            upper_l.push(self.points[i]);
            upper_l = self.assure_make_turn_right(upper_l);
        }
        upper_l.pop();

        lower_l.push(self.points[self.points.len() - 1]);
        lower_l.push(self.points[self.points.len() - 2]);
        for i in (0..(self.points.len() - 2)).rev() {
            lower_l.push(self.points[i]);
            lower_l = self.assure_make_turn_right(lower_l);
        }
        lower_l.pop();

        let mut convex_hull = Points::new();
        convex_hull.append(&mut upper_l);
        convex_hull.append(&mut lower_l);

        convex_hull
    }

    fn step(&mut self) {
        todo!();
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl MonotoneConvexHull {
    pub fn build(points: Points) -> Self {
        Self { points }
    }

    /// this method takes a list of point and it assure that they turn right
    fn assure_make_turn_right(&self, mut half_l: Points) -> Points {
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

#[cfg(test)]
mod tests {
    use super::MonotoneConvexHull;
    use crate::{algorithms::algorithm::Algorithm, geometry::points::Points};

    #[test]
    fn convex_hull() {
        let vec_points: &mut Points =
            &mut vec![[0.0, 0.0], [0.0, 4.0], [4.0, 0.0], [4.0, 4.0]].into();

        let assert_points = vec_points.clone();
        let mut points = Points::random(10, 0.1..3.9);
        points.append(vec_points);

        let mut algo = MonotoneConvexHull::build(points);
        let mut convex_hull = algo.calculate();
        convex_hull.lexicograph_sort();

        assert_eq!(assert_points, convex_hull);
    }
}
