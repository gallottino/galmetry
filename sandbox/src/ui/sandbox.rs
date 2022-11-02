use eframe::{
    egui::{
        self,
        plot::{Legend, Line, Plot, PlotPoints, Points},
    },
    epaint::Color32,
};
use galmetry::{algorithms::convex_hull::MonotoneConvexHull, geometry::point::Point};

pub struct GalmetrySandbox {
    points: galmetry::geometry::point::Points,
    convex_hull: galmetry::geometry::point::Points,
}

impl Default for GalmetrySandbox {
    fn default() -> Self {
        Self {
            points: galmetry::geometry::point::Points::new(),
            convex_hull: galmetry::geometry::point::Points::new(),
        }
    }
}

impl GalmetrySandbox {
    pub fn random() -> Self {
        Self {
            points: galmetry::geometry::point::Points::random(50, 0.0..10.0),
            convex_hull: galmetry::geometry::point::Points::new(),
        }
    }

    fn get_plot_points(&self, p: &galmetry::geometry::point::Points) -> PlotPoints {
        let vec_points: Vec<[f64; 2]> = p.clone().into();
        vec_points.into()
    }
}

impl eframe::App for GalmetrySandbox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Monotone Algorithm").clicked() {
                    let mut algo = MonotoneConvexHull::build(self.points.clone());
                    self.convex_hull = algo.calculate();
                }
                if ui.button("Reset").clicked() {
                    self.points = galmetry::geometry::point::Points::random(50, 0.0..10.0);
                    self.convex_hull = galmetry::geometry::point::Points::new();
                }
            });

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
                        .shape(egui::plot::MarkerShape::Square),
                );

                if plot_ui.plot_clicked() {
                    let click_plot = plot_ui.pointer_coordinate();
                    match click_plot {
                        Some(p) => self.points.0.push(Point::from2d(p.x, p.y)),
                        None => (),
                    }
                }
            });
        });
    }
}
