use std::collections::{BTreeMap, BTreeSet};

use crate::{
    algorithms::algorithm::Algorithm,
    geometry::{point::Point, segment::Segment},
};

use super::data_structure::{SweepLine, SweepPoint};

impl SweepLine {
    pub fn build(segments: Vec<Segment>) -> Self {
        let mut queue: BTreeMap<SweepPoint, Vec<Segment>> = BTreeMap::new();
        for s in segments.clone() {
            queue
                .entry(SweepPoint(s.start))
                .and_modify(|segs| segs.push(s.clone()))
                .or_insert(vec![s.clone()]);

            queue
                .entry(SweepPoint(s.end))
                .and_modify(|segs| segs.push(s.clone()))
                .or_insert(vec![]);
        }

        Self {
            queue,
            status: BTreeSet::new(),
            segments,
            event_point: Point::default(),
            u_p: vec![],
            l_p: vec![],
            c_p: vec![],
        }
    }

    fn handle_event_point(&mut self) {
        let (event_point, u_p) = match self.queue.pop_first().unwrap() {
            (sweep_point, segments) => (sweep_point.0, segments),
        };
        self.event_point = event_point;
        self.u_p = u_p;

        self.print_status();
    }

    fn print_status(&self) {
        println!("Processing {}", self.event_point);
        println!("--------------------------");
        println!();
        println!("Upper point of: ");

        let mut count = 0;
        for s in self.u_p.clone() {
            println!("[{}]: {}", count, s);
            count += 1;
        }
        println!();
        println!("##########END#############");
        println!();
    }
}

impl Algorithm for SweepLine {
    type Output = Vec<Point>;

    fn calculate(&mut self) -> Self::Output {
        todo!()
    }

    fn step(&mut self) {
        self.handle_event_point();
    }

    fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{algorithm::Algorithm, sweep_line::data_structure::SweepLine},
        geometry::{point::Point, segment::Segment},
    };

    #[test]
    fn check_build_queue() {
        let s1: Segment = [[1.0, 3.0], [3.0, 1.0]].into();
        let s2: Segment = [[1.0, 1.0], [3.0, 3.0]].into();
        let s3: Segment = [[1.0, 2.5], [3.0, 2.0]].into();
        let segments = vec![s1, s2, s3];

        let algo = SweepLine::build(segments);
        let queue: Vec<Point> = algo
            .queue
            .into_iter()
            .map(|key_value| key_value.0 .0)
            .collect();

        let assert_queue: Vec<Point> = vec![
            [1.0, 3.0].into(),
            [3.0, 3.0].into(),
            [1.0, 2.5].into(),
            [3.0, 2.0].into(),
            [1.0, 1.0].into(),
            [3.0, 1.0].into(),
        ];
        for i in 0..queue.len() {
            assert!(queue[i] == assert_queue[i])
        }
    }

    #[test]
    fn check_steps() {
        let s1: Segment = [[1.0, 3.0], [3.0, 1.0]].into();
        let s2: Segment = [[1.0, 1.0], [3.0, 3.0]].into();
        let s3: Segment = [[1.0, 2.5], [3.0, 2.0]].into();
        let segments = vec![s1, s2, s3];

        let mut algo = SweepLine::build(segments);
        while algo.queue.is_empty() == false {
            algo.step();
        }
    }
}
