use crate::geometry::{point::Point, points::Points};

pub struct MonotoneConvexHull {
    points: Points,
}

impl MonotoneConvexHull {
    pub fn build(points: Points) -> Self {
        Self { points }
    }

    pub fn calculate(&mut self) -> Points {
        let mut upper_l = Points::new();
        let mut lower_l = Points::new();

        self.points.lexicograph_sort();

        upper_l.push(self.points[0]);
        upper_l.push(self.points[1]);
        for i in 0..self.points.len() {
            upper_l.push(self.points[i]);
            upper_l = self.half_calculate(&mut upper_l);
        }
        upper_l.pop();

        lower_l.push(self.points[self.points.len() - 1]);
        lower_l.push(self.points[self.points.len() - 2]);
        for i in (0..(self.points.len() - 2)).rev() {
            lower_l.push(self.points[i]);
            lower_l = self.half_calculate(&mut lower_l);
        }
        lower_l.pop();

        upper_l.append(&mut lower_l);
        upper_l
    }

    fn half_calculate(&self, half_l: &mut Points) -> Points {
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
        half_l.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::MonotoneConvexHull;
    use crate::geometry::points::Points;

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
