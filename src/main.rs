// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::App;

mod app;
mod tracker;
mod widgets;
fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(egui::Vec2 { x: 900.0, y: 600.0 });

    eframe::run_native(
        "Expenses Tracker",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
