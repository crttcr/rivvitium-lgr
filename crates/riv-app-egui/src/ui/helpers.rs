
use eframe::epaint::Color32;
use egui::RichText;
use zero::component::telemetry::component_metrics::ComponentStatus;

pub const ROW_HEIGHT: f32 = 20.0;

pub fn right_label(ui: &mut egui::Ui, txt: impl Into<String>) {
    use egui::{Align, Layout};
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.label(RichText::new(txt).monospace());
    });
}

pub fn row_u64(label: &str, val: u64, body: &mut egui_extras::TableBody) {
	let v = apex::utils::format_utils::format_u64(val);
    body.row(ROW_HEIGHT, |mut row| {
        row.col(|ui| {ui.label(label);});
        row.col(|ui| {right_label(ui, v);});
    });
}

pub fn row_u32(label: &str, val: u32, body: &mut egui_extras::TableBody) {
	let v = apex::utils::format_utils::format_u32(val);
    body.row(ROW_HEIGHT, |mut row| {
        row.col(|ui| {ui.label(label);});
        row.col(|ui| {right_label(ui, v);});
    });
}

pub fn status_as_rich_text(status: ComponentStatus) -> RichText {
	match status {
		ComponentStatus::Idle      => RichText::new("Idle"     ).color(Color32::GRAY),
      ComponentStatus::Active    => RichText::new("Active"   ).color(Color32::GREEN),
      ComponentStatus::Completed => RichText::new("Completed").color(Color32::DARK_BLUE),
      ComponentStatus::Failed    => RichText::new("Failed"   ).color(Color32::RED),
	}
}
