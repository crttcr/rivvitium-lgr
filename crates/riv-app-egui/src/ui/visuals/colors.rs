
use egui::Color32;

pub const PRIMARY:        Color32 = Color32::from_rgb( 28,  90, 235);
pub const PRIMARY_HOVER:  Color32 = Color32::from_rgb( 48, 110, 255);
pub const PRIMARY_ACTIVE: Color32 = Color32::from_rgb( 18,  70, 205);

pub const ACCENT:         Color32 = Color32::from_rgb(128,   0, 192); // purple “Run” button
pub const ACCENT_BORDER:  Color32 = Color32::from_rgb( 90,   0, 140);
#[allow(dead_code)]
pub const ACCENT_HOVER:   Color32 = Color32::from_rgb(148,  30, 212);
#[allow(dead_code)]
pub const ACCENT_ACTIVE:  Color32 = Color32::from_rgb(108,   0, 162);

pub const TEXT_ON_ACCENT: Color32 = Color32::WHITE;


pub fn apply_standard_colors(ui: &mut egui::Ui) {
	let v = &mut ui.visuals_mut().widgets;
	v.inactive.bg_fill      = PRIMARY;
	v.hovered.bg_fill       = PRIMARY_HOVER;
	v.active.bg_fill        = PRIMARY_ACTIVE;
	v.inactive.weak_bg_fill = PRIMARY;
	v.hovered.weak_bg_fill  = PRIMARY_HOVER;
	v.active.weak_bg_fill   = PRIMARY_ACTIVE;
}
