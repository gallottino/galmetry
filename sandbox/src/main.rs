#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::epaint::Vec2;

mod ui;
use crate::ui::sandbox::GalmetrySandbox;

fn main() {
    let mut options = eframe::NativeOptions::default();

    options.min_window_size = Some(Vec2::new(600.0, 600.0));
    eframe::run_native(
        "Galmetry Sandbox",
        options,
        Box::new(|_cc| Box::new(GalmetrySandbox::random())),
    );
}
