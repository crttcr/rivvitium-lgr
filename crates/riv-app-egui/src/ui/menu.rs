
use egui::{Button, RichText};
use crate::app::rivvitium_app::RivvitiumApp;
use crate::ui::dialogs::file_dialog::choose_file_with_native_dialog;
use crate::ui::visuals::colors::ColorTheme;

// ------------------------  menu bar  ------------------------------
pub fn create_menu_bar(state: &mut RivvitiumApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            draw_file_menu(ui, state);
            draw_relay_menu(ui, state);
            draw_destination_menu(ui, state);
            draw_run_menu(ui, state);
            draw_settings_menu(ui, state);
            draw_help_menu(ui, state);
        });
    });
}

// File Menu
//
fn draw_file_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("File", |ui| {
	    inject_button_file_open(ui, state);
	    inject_button_file_close(ui, state);
	    inject_button_file_exit(ui, state);
    });
}

fn inject_button_file_open(ui: &mut egui::Ui, app: &mut RivvitiumApp) {
	let enabled = true;
	let text    = RichText::new("Open ...");
	let button  = Button::new(text);
	if ui.add_enabled(enabled, button).clicked() {
		choose_file_with_native_dialog(&mut app.app_state, &mut app.ui_state);
      ui.close_menu(); // collapse the menu
	}
}

fn inject_button_file_close(ui: &mut egui::Ui, app: &mut RivvitiumApp) {
	let enabled = app.app_state.has_selected_file();
	let text    = RichText::new("Close");
	let button  = Button::new(text);
	if ui.add_enabled(enabled, button).clicked() {
		app.app_state.close_file();
      ui.close_menu(); // collapse the menu
	}
}

fn inject_button_file_exit(ui: &mut egui::Ui, app: &mut RivvitiumApp) {
	let cmd_close = egui::ViewportCommand::Close;
	if ui.button("Exit").clicked() {
		app.app_state.teardown();
		ui.ctx().send_viewport_cmd(cmd_close);
	}
}

// Relay Menu
//
fn draw_relay_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Relay", |ui| {
        if ui.button("Add a relay").clicked() {
            state.ui_state.show_about_dialog();
        };
        ui.label("Remove relay");
        ui.label("Clear all relays");
    });
}

// Destination Menu
//
fn draw_destination_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Destination", |ui| {
        if ui.button("Csv file").clicked() {
            state.ui_state.show_about_dialog();
        };
        ui.label("JSON array file");
        ui.label("JSON object file");
        ui.label("Sql Server");
        ui.label("Sqlite");
    });
}

// Run Menu
//
fn draw_run_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Run", |ui| {
        if ui.button("Parse file").clicked() {
            if state.ui_state.is_about_dialog_visible() {
                state.ui_state.hide_about_dialog();
            } else {
                state.ui_state.show_about_dialog();
            }
        };
        ui.label("Parse file");
    });
}

// Settings Menu
//
fn draw_settings_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Settings", |ui| {
        if ui.button("Default").clicked() {
            state.app_settings = ColorTheme::default();
            ui.close_menu(); // collapse the menu
        };
        if ui.button("Forest").clicked() {
            state.app_settings = ColorTheme::ForestCanopy;
            ui.close_menu(); // collapse the menu
        };
        if ui.button("Ocean").clicked() {
            state.app_settings = ColorTheme::OceanBreeze;
            ui.close_menu(); // collapse the menu
        };
        if ui.button("Sunset").clicked() {
            state.app_settings = ColorTheme::SunsetGlow;
            ui.close_menu(); // collapse the menu
        };
    });
}


// Help Menu
//
fn draw_help_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Help", |ui| {
        ui.label("What's new in Rivvitium");
        if ui.button("About").clicked() {
            state.ui_state.show_about_dialog();
            ui.close_menu(); // collapse the menu
        }
    });
}
