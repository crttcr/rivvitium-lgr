use crate::app::rivvitium_app::RivvitiumApp;
use crate::ui::visuals::colors::{apply_standard_colors, ACCENT, ACCENT_BORDER, TEXT_ON_ACCENT};
use eframe::epaint::Stroke;
use egui::{Button, RichText};

/// Draw a horizontal array of buttons and wire behavior
///
pub fn draw_button_bar(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        apply_standard_colors(ui);
        if ui.button("Source").clicked() { app.click_count += 1; }
        if ui.button("Relays").clicked() { app.click_count += 1; }
        if ui.button("Destination").clicked() { app.click_count += 1; }

        let purple_bg   = ACCENT;
        let darker_edge = ACCENT_BORDER;

        if ui.add(
            Button::new(RichText::new("Run").color(TEXT_ON_ACCENT).strong())
              .fill(purple_bg) // background
              .stroke(Stroke::new(1.0, darker_edge)), // <- width 1 px, colour
            )
            .clicked()
        {
            if app.click_count > 0 { app.click_count -= 1; }
        }
        
        if ui.button("Run").clicked() { app.click_count += 1; }
    });
    ui.separator();
}
