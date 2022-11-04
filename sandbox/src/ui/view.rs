use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait AlgorithmSection {
    fn label(&self) -> &'static str;

    fn calculate(&mut self);

    fn reset(&mut self);

    fn show(&mut self, ctx: &eframe::egui::Context);
}
