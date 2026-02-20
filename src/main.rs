use crate::words::generate_test;
use eframe::egui;
use egui::{Color32, Key, RichText, Vec2};
use std::collections::HashSet;
mod words;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("Aero", options, Box::new(|_cc| Ok(Box::<Aero>::default())))
}

struct Aero {
    show_main_viewport: bool,
    show_settings_viewport: bool,
    test: words::Test,
    active_indices: HashSet<usize>,
}

impl Aero {
    fn new() -> Self {
        // "customize egui here - docs"
        Self {
            test: words::generate_test(10),
            show_main_viewport: true,
            show_settings_viewport: false,
            active_indices: HashSet::new(),
        }
    }
}

impl Default for Aero {
    fn default() -> Self {
        Self {
            test: generate_test(10),
            show_main_viewport: true,
            show_settings_viewport: false,
            active_indices: HashSet::new(),
        }
    }
}

impl eframe::App for Aero {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let mut characters = "abcdefghijklmnopqrstuvwxyz1234567890";
        let mut qwerty = "qwertyuiopasdfghjklzxcvbnm";
        let mut buttons: Vec<egui::Button> = Vec::new();
        let chars: Vec<char> = qwerty.chars().collect();
        let mut active_index: Option<usize> = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Aero");

            if ui.button("settings").clicked() {
                self.show_settings_viewport = true;
            }

            // Generated Words
            egui::Grid::new("words").show(ui, |ui| {
                for word in &self.test.words {
                    ui.label(RichText::new(word).size(18.0).strong());
                }
            });

            // Reactive Keyboard
            egui::Grid::new("keys").show(ui, |ui| {
                let row_sizes = [10, 9, 7];

                // detect pressed key
                self.active_indices.clear();

                for (index, ch) in qwerty.chars().enumerate() {
                    if let Some(key) = Key::from_name(&ch.to_string()) {
                        if ui.input(|i| i.key_down(key)) {
                            self.active_indices.insert(index);
                        }
                    }
                }

                // draw buttons
                let mut index = 0;

                for size in row_sizes {
                    for i in 0..size {
                        let key_index = index + i;
                        let ch = chars[key_index];

                        let mut button =
                            egui::Button::new(ch.to_string()).min_size(Vec2::new(60.0, 60.0));

                        // highlight
                        if self.active_indices.contains(&key_index) {
                            button = button
                                .fill(Color32::from_rgb(0, 0, 0))
                                .stroke(egui::Stroke::new(2.0, Color32::WHITE));
                        }

                        ui.add(button);
                    }

                    ui.end_row();
                    index += size;
                }
            });

            // viewport stuff
            if self.show_settings_viewport {
                egui::Window::new("settings").show(ctx, |ui| {
                    if ui.button("close").clicked() {
                        self.show_settings_viewport = false;
                    }
                });
            }
        });
    }
}
