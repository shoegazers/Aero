use eframe::egui;
use egui::{Button, Color32, Key, Label, Vec2};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Aero",
        options,
        Box::new(|_cc| Ok(Box::<Aero>::default())),
    )
}

#[derive(Default)]
struct Aero {
    show_main_viewport: bool,
    show_settings_viewport: bool,
}

impl Aero {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // "customize egui here - docs"
        Self::default()
    }
}

impl eframe::App for Aero {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // let mut characters = "abcdefghijklmnopqrstuvwxyz1234567890";
        let mut qwerty = "qwertyuiopasdfghjklzxcvbnm";
        let mut buttons: Vec<egui::Button> = Vec::new();
        let chars: Vec<char> = qwerty.chars().collect();
        let mut active_index: Option<usize> = None;


        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Aero");

            if ui.button("settings").clicked() {
                self.show_settings_viewport = true;
            }

            egui::Grid::new("keys").show(ui, |ui|{
                let row_sizes = [10, 9, 7];

                // detect pressed key
                for (index, ch) in qwerty.chars().enumerate() {
                    if ui.input(|i| {
                        i.key_down(Key::from_name(&ch.to_string()).unwrap())
                    }) {
                        active_index = Some(index);
                    }
                }
                
                // draw buttons
                let mut index = 0;

                for size in row_sizes {
                    for i in 0..size {
                        let ch = chars[index + i];

                        let mut button = egui::Button::new(ch.to_string())
                            .min_size(Vec2::new(60.0, 60.0));

                        if Some(index + i) == active_index {
                            button = button.fill(Color32::from_rgb(0, 0, 0));
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
