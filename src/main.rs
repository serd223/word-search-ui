#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800., 600.)),
        ..Default::default()
    };

    eframe::run_native(
        "Word Search",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!();
    }
}
