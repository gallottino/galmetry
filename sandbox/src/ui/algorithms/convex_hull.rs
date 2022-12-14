use crate::ui::view::{AlgorithmSection, View};

use eframe::{
    egui::{
        self,
        plot::{Legend, Line, Plot, PlotPoints, Points},
    },
    epaint::Color32,
};
use galmetry::{
    algorithms::{algorithm::Algorithm, convex_hull::MonotoneConvexHull},
    geometry::point::Point,
};

pub struct ConvexHullView {
    points: galmetry::geometry::points::Points,
    convex_hull: galmetry::geometry::points::Points,
}

impl Default for ConvexHullView {
    fn default() -> Self {
        Self {
            points: galmetry::geometry::points::Points::new(),
            convex_hull: galmetry::geometry::points::Points::new(),
        }
    }
}

impl ConvexHullView {
    pub fn random(capacity: usize) -> Self {
        Self {
            points: galmetry::geometry::points::Points::random(capacity, 0.1..0.9),
            convex_hull: galmetry::geometry::points::Points::new(),
        }
    }

    fn get_plot_points(&self, p: &galmetry::geometry::points::Points) -> PlotPoints {
        let vec_points: Vec<[f64; 2]> = p.clone().into();
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
            .allow_drag(false);

        plot.show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(self.get_plot_points(&self.convex_hull))
                    .color(Color32::from_rgb(0, 255, 0)),
            );

            plot_ui.points(
                Points::new(self.get_plot_points(&self.points))
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
    }
}
