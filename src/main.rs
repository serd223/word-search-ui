#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/*
Word sources:
turkish_words.txt: https://github.com/mertemin/turkish-word-list
wordle-nyt-allowed-guesses.txt: https://github.com/fredoverflow/wordle
wordle-nyt-answers-alphabetical.txt: https://github.com/fredoverflow/wordle
words_alpha.txt: https://github.com/dwyl/english-words/
*/

use std::fs;

use eframe::{
    egui::{self, RichText},
    epaint::Color32,
};
use word_search::Library;

// TODO dont hardcode the source file, let users change their source file during runtime
const SOURCE_FILE: &str = "./res/words_alpha.txt";

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800., 600.)),
        ..Default::default()
    };

    let source = match fs::read_to_string(SOURCE_FILE) {
        Ok(s) => s,
        Err(_) => String::new(),
    };

    let mut app = App::default();
    app.word_lib.set_source(source);

    eframe::run_native("Word Search", options, Box::new(|_cc| Box::new(app)));
}

#[derive(Default)]
struct App {
    word_lib: Library,
    input: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Which word do you want to search?")
                        .color(Color32::WHITE)
                        .heading(),
                );
                ui.text_edit_singleline(&mut self.input);
            });

            let search_results = self.word_lib.search(&self.input);
            // println!("{:?}", search_results);
            ui.vertical_centered(|ui| {
                for (s, _) in search_results {
                    ui.label(RichText::new(s));
                }
            })
        });
    }
}
