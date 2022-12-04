use eframe::{
    egui::{self, Button},
    epaint::Color32,
};

use super::{
    algorithms::{convex_hull::ConvexHullView, sweep_plane::SweepPlaneView},
    view::AlgorithmSection,
};

pub struct GalmetrySandbox {
    pub algorithms: Vec<Box<dyn AlgorithmSection>>,
    pub current: String,
}

impl Default for GalmetrySandbox {
    fn default() -> Self {
        let mut algorithms: Vec<Box<dyn AlgorithmSection>> = vec![];
        algorithms.push(Box::new(ConvexHullView::random(50)));
        algorithms.push(Box::new(SweepPlaneView::random(20)));

        let current = algorithms[0].label().to_owned();
        Self {
            algorithms,
            current,
        }
    }
}

impl eframe::App for GalmetrySandbox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Algorithms").show(ctx, |ui| {
            ui.heading("Algorithms");
            ui.separator();
            for view in &mut self.algorithms {
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if view.label() == self.current {
                        ui.add(Button::new("   ").fill(Color32::from_rgb(255, 255, 255)));
                    } else {
                        ui.add(Button::new("   "));
                    }

                    let mut section_b = egui::Button::new(view.label());

                    if view.label() == self.current {
                        section_b = egui::Button::new(view.label())
                    }
                    if ui.add(section_b).clicked() {
                        self.current = view.label().to_owned();
                    }

                    ui.add_space(20.0);

                    if ui.button("Run ▶").clicked() {
                        view.calculate();
                    }

                    if ui.button("⟲").clicked() {
                        view.reset();
                        self.current = view.label().to_owned();
                    }
                });
            }
        });

        for section in &mut self.algorithms {
            match section.label() == self.current {
                true => section.show(ctx),
                false => {}
            }
        }
    }
}
