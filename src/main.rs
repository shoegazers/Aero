use eframe::egui;
use egui::TextBuffer;

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
        let mut label_responses: Vec<egui::Response> = Vec::new();

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Aero");

            if ui.button("settings").clicked() {
                self.show_settings_viewport = true;
            }

            egui::Grid::new("keys").show(ui, |ui|{
                let chars: Vec<char> = qwerty.chars().collect();
                let row_sizes = [10, 9, 7];

                let mut index = 0;
                for size in row_sizes {
                    for ch in &chars[index..index + size] {
                        let mut l = ui.label(ch.to_string());
                        label_responses.push(l);
                    }
                    ui.end_row();
                    index += size;
                }
            }); 

            if ui.input(|i| i.key_pressed(egui::Key::A)) {
                for l in label_responses {
                    
                }
            }

            /* 
            for c in characters.chars() {
                if ui.button(c.to_string()).clicked() {

                }
            }
            */
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
