use crate::app::rivvitium_app::RivvitiumApp;
use crate::ui::visuals::colors::{apply_color_theme, ACCENT, ACCENT_BORDER, TEXT_ON_ACCENT};
use eframe::epaint::Stroke;
use egui::{Button, RichText};
use crate::ui::dialogs::file_dialog::choose_file_with_native_dialog;
use crate::ui::regions::ActiveAction;

/// Draw a horizontal array of buttons and wire behavior
///
/// Note, there are 2 sets of buttons. The first set configures the
/// pipeline and the second set performs data operations
///
pub fn draw_button_bar(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        apply_color_theme(ui, app.app_settings);
        draw_source_button(app, ui);
        draw_relays_button(app, ui);
        draw_destination_button(app, ui);
        ui.separator();
        draw_parse_button(app, ui);
        draw_analyze_button(app, ui);
        draw_blueprint_button(app, ui);
        draw_publish_button(app, ui);
    });
    ui.separator();
}

//
// Configuration buttons
//

fn draw_source_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = app.app_state.has_selected_file() == false;
	let text    = RichText::new("Source");
	let button  = Button::new(text);
	if ui.add_enabled(enabled, button).clicked() {
		choose_file_with_native_dialog(&mut app.app_state, &mut app.ui_state);
	}
}

fn draw_relays_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = true;
	let text    = RichText::new("Relays");
	let button  = Button::new(text);
	if ui.add_enabled(enabled, button).clicked() {
		app.app_state.capture_click();
	}
}

fn draw_destination_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = true;
	let text    = RichText::new("Destination");
	let button  = Button::new(text);
	if ui.add_enabled(enabled, button).clicked() {
		println!("Destination button clicked");
		app.ui_state.set_sink_dialog_visible();
		app.app_state.capture_click();
	}
}

//
// Action buttons
//

fn draw_parse_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let text    = RichText::new("Parse").color(TEXT_ON_ACCENT).strong();
	let stroke  = Stroke::new(1.0, ACCENT_BORDER);
	let button  = Button::new(text)
		.fill(ACCENT)
		.stroke(stroke);
	if app.app_state.has_selected_file() {
			if ui.add_enabled(true, button).clicked() {
				app.fire_parse_command();
			}
	}	else {
			if ui.add_enabled(false, button).clicked() {}
	}
}

fn draw_analyze_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = app.app_state.has_selected_relays();
	let text    = RichText::new("Analyze").color(TEXT_ON_ACCENT).strong();
	let stroke  = Stroke::new(1.0, ACCENT_BORDER);
	let button  = Button::new(text)
		.fill(ACCENT)
		.stroke(stroke);
	if ui.add_enabled(enabled, button).clicked() {
		app.app_state.capture_click();
	}
}

fn draw_blueprint_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = app.app_state.has_selected_file();
	let text    = RichText::new("Blueprint").color(TEXT_ON_ACCENT).strong();
	let stroke  = Stroke::new(1.0, ACCENT_BORDER);
	let button  = Button::new(text)
		.fill(ACCENT)
		.stroke(stroke);
	if ui.add_enabled(enabled, button).clicked() {
		app.app_state.capture_click();
	}
}

fn draw_publish_button(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	let enabled = app.app_state.sink_permits_publish();
	let text    = RichText::new("Publish").color(TEXT_ON_ACCENT).strong();
	let stroke  = Stroke::new(1.0, ACCENT_BORDER);
	let button  = Button::new(text)
		.fill(ACCENT)
		.stroke(stroke);
	if ui.add_enabled(enabled, button).clicked() {
		app.ui_state.active_panel = ActiveAction::PostPublication;
	}
}
