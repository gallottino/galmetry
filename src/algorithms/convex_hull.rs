use crate::geometry::point::{Point, Points};

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

        self.points.0.sort();

        upper_l.0.push(self.points.0[0]);
        upper_l.0.push(self.points.0[1]);
        for i in 0..self.points.0.len() {
            upper_l.0.push(self.points.0[i]);
            upper_l = self.half_calculate(&mut upper_l);
        }

        lower_l.0.push(self.points.0[self.points.0.len() - 1]);
        lower_l.0.push(self.points.0[self.points.0.len() - 2]);
        for i in (0..(self.points.0.len() - 2)).rev() {
            lower_l.0.push(self.points.0[i]);
            lower_l = self.half_calculate(&mut lower_l);
        }

        upper_l.0.pop();
        lower_l.0.pop();
        upper_l.0.append(&mut lower_l.0);
        upper_l
    }

    fn half_calculate(&self, half_l: &mut Points) -> Points {
        let mut half_l_len = half_l.0.len();
        while half_l_len > 2
            && !Point::make_right_turn(
                &half_l.0[half_l_len - 3],
                &half_l.0[half_l_len - 2],
                &half_l.0[half_l_len - 1],
            )
        {
            half_l.0[half_l_len - 2] = half_l.0[half_l_len - 1];
            half_l.0.pop();
            half_l_len = half_l.0.len();
        }
        half_l.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::MonotoneConvexHull;
    use crate::geometry::point::{Point, Points};

    #[test]
    fn convex_hull() {
        let vec_points: &mut Vec<Point> = &mut vec![[0.0, 0.0], [0.0, 4.0], [4.0, 0.0], [4.0, 4.0]]
            .iter()
            .map(|p| (*p).into())
            .collect();
        let assert_vec: Vec<Point> = vec_points.clone();

        let mut points = Points::random(20, 0.1..3.9);
        points.0.append(vec_points);

        let mut algo = MonotoneConvexHull::build(points);
        let mut convex_hull = algo.calculate();
        convex_hull.0.sort();

        assert_eq!(assert_vec, convex_hull.0);
    }
}
