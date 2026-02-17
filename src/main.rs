use std::num::NonZero;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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

        if self.show_main_viewport {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Aero");

                if ui.button("settings").clicked() {
                    self.show_settings_viewport = true;
                    self.show_main_viewport = false;
                }
            });
        }

        if self.show_settings_viewport {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("settings");

                if ui.button("back").clicked() {
                    self.show_settings_viewport = false;
                    self.show_main_viewport = true;
                }
            });
        }
    }
}
