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
