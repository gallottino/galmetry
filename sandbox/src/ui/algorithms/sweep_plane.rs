use std::collections::BTreeSet;

use crate::ui::view::{AlgorithmSection, View};

use eframe::{
    egui::{
        self,
        plot::{Legend, Line, Plot, Points},
    },
    epaint::Color32,
};
use galmetry::{self, algorithms::sweep_plane::SweepPlane};
use galmetry::{
    algorithms::algorithm::Algorithm,
    geometry::{point::Point, segment::Segment},
};

pub struct SweepPlaneView {
    segments: BTreeSet<Segment>,
    intersection_points: BTreeSet<Point>,
}

impl Default for SweepPlaneView {
    fn default() -> Self {
        Self {
            segments: BTreeSet::new(),
            intersection_points: BTreeSet::new(),
        }
    }
}

impl SweepPlaneView {
    pub fn random(capacity: usize) -> Self {
        let mut random_segments = BTreeSet::<Segment>::new();
        for _i in 0..capacity {
            random_segments.insert(Segment::random(0.1..0.9));
        }
        Self {
            segments: random_segments,
            intersection_points: BTreeSet::new(),
        }
    }
}

impl View for SweepPlaneView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        let plot = Plot::new("Sweep Plane")
            .legend(Legend::default())
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .allow_drag(false);

        plot.show(ui, |plot_ui| {
            for s in self.segments.clone() {
                plot_ui.line(
                    Line::new(vec![[s.start.x, s.start.y], [s.end.x, s.end.y]])
                        .color(Color32::from_rgb(255, 255, 255)),
                );
            }

            for p in self.intersection_points.clone() {
                plot_ui.points(
                    Points::new(vec![[p.x, p.y]])
                        .radius(5.0)
                        .color(Color32::from_rgb(200, 10, 10))
                        .shape(eframe::egui::plot::MarkerShape::Square),
                );
            }
        });
    }
}

impl AlgorithmSection for SweepPlaneView {
    fn label(&self) -> &'static str {
        &"Sweep Plane"
    }

    fn calculate(&mut self) {
        let mut algo = SweepPlane::build(self.segments.clone());
        self.intersection_points = algo.calculate();
    }

    fn reset(&mut self) {
        *self = SweepPlaneView::random(20);
    }

    fn show(&mut self, ctx: &eframe::egui::Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            self.ui(ui);
        });
    }
}
