use eframe::epaint::Margin;
use egui::{Color32, Frame, RichText};



/* helper that draws a blue banner across the entire inner width */
pub fn caption_banner(ui: &mut egui::Ui, text: &str) {
    Frame::default()
        .fill(Color32::from_rgb(225, 235, 255))        // light blue
        .inner_margin(Margin::symmetric(6, 4))         // i8 values in egui 0.31+
        .corner_radius(4)                                   // rounded corners
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .strong()
                    .color(Color32::DARK_BLUE),
            );
        });
}
