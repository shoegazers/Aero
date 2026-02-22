use std::collections::HashSet;

use egui::{Color32, FontId, TextFormat, Ui, text::LayoutJob};

pub fn test(ui: &mut Ui, word: &str, pressed: &std::collections::HashSet<char>) {
    let mut layout = LayoutJob::default();
    let mut changed: HashSet<char> = HashSet::new();

    for ch in word.chars() {
        let lower = ch.to_ascii_lowercase();
        let color = if pressed.contains(&lower) && !changed.contains(&lower) {
            changed.insert(lower);
            Color32::GRAY
        } else {
            Color32::WHITE
        };

        layout.append(&ch.to_string(), 0.0, TextFormat { font_id: FontId::proportional(18.0), color, ..Default::default()});
    }

    ui.label(layout);
}