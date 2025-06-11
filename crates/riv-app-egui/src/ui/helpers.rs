
use eframe::epaint::Color32;
use egui::RichText;
use apex::state::parse_detail_dto::ParseStatus;

pub const ROW_HEIGHT: f32 = 20.0;

fn right_label(ui: &mut egui::Ui, txt: impl Into<String>) {
    use egui::{Align, Layout, RichText};

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

pub fn status_as_rich_text(status: ParseStatus) -> RichText {
	match status {
		ParseStatus::NotParsed  => RichText::new("Not parsed" ).color(Color32::GRAY),
      ParseStatus::InProgress => RichText::new("In progress").color(Color32::DARK_BLUE),
      ParseStatus::Finished   => RichText::new("Finished"   ).color(Color32::GREEN),
      ParseStatus::Error      => RichText::new("Error"      ).color(Color32::RED),
	}
}