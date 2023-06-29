use crate::ui::view::{AlgorithmSection, View};

use eframe::{
    egui::{
        self,
        plot::{Legend, Line, Plot, PlotPoint, PlotPoints, Points, Text},
    },
    epaint::Color32,
};
use galmetry::{
    algorithms::{algorithm::Algorithm, convex_hull::MonotoneConvexHull},
    geometry::point::Point,
};

pub struct ConvexHullView {
    points: Vec<Point>,
    convex_hull: Vec<Point>,
}

impl Default for ConvexHullView {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            convex_hull: Vec::new(),
        }
    }
}

impl ConvexHullView {
    pub fn random(capacity: usize) -> Self {
        Self {
            points: Point::random_vec(capacity, 0.1..0.9),
            convex_hull: Vec::new(),
        }
    }

    fn get_plot_points(&self, p: Vec<Point>) -> PlotPoints {
        let vec_points: Vec<[f64; 2]> = p.into_iter().map(|p| p.into()).collect();
        vec_points.into()
    }
}

impl View for ConvexHullView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        let plot = Plot::new("Convex Hull")
            .legend(Legend::default())
            .view_aspect(1.0)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .height(580.0);

        plot.show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(self.get_plot_points(self.convex_hull.clone()))
                    .color(Color32::from_rgb(0, 255, 0)),
            );

            for p in self.points.clone() {
                plot_ui.text(Text::new(PlotPoint::new(p.x, p.y + 0.02), p.to_string()));
            }

            plot_ui.points(
                Points::new(self.get_plot_points(self.points.clone()))
                    .radius(3.0)
                    .color(Color32::from_rgb(200, 0, 0))
                    .shape(eframe::egui::plot::MarkerShape::Square),
            );

            if plot_ui.plot_clicked() {
                let click_plot = plot_ui.pointer_coordinate();
                match click_plot {
                    Some(p) => self.points.push(Point::from2d(p.x, p.y)),
                    None => (),
                }
            }
        });
    }

    fn debug_ui(&mut self, _ui: &mut egui::Ui) {}
}

impl AlgorithmSection for ConvexHullView {
    fn label(&self) -> &'static str {
        &"Convex Hull  "
    }

    fn calculate(&mut self) {
        let mut algo = MonotoneConvexHull::build(self.points.clone());
        self.convex_hull = algo.calculate();
        self.convex_hull.push(self.convex_hull[0]);
    }

    fn reset(&mut self) {
        *self = ConvexHullView::random(50);
    }

    fn show(&mut self, ctx: &eframe::egui::Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            self.ui(ui);
        });

        egui::SidePanel::right("Debug")
            .default_width(600.0)
            .show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Your name:                                               ");
                })
            });
    }
}
