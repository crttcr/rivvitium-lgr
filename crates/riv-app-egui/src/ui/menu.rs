
use crate::app::rivvitium_app::RivvitiumApp;

// ------------------------  menu bar  ------------------------------
pub fn create_menu_bar(state: &mut RivvitiumApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            draw_file_menu(ui, state);
            draw_relay_menu(ui, state);
            draw_destination_menu(ui, state);
            draw_run_menu(ui, state);
            draw_help_menu(ui, state);
        });
    });
}

fn draw_file_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    let cmd_close = egui::ViewportCommand::Close;
    ui.menu_button("File", |ui| {
        if ui.button("Choose file to parse").clicked() {
            state.show_dialog = true;
        };
        if ui.button("Clear file").clicked() {
            state.show_dialog = true;
        };
        if ui.button("Exit").clicked() {
            ui.ctx().send_viewport_cmd(cmd_close);
        }
    });
}

fn draw_relay_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Relay", |ui| {
        if ui.button("Add a relay").clicked() {
            state.show_dialog = true;
        };
        ui.label("Remove relay");
        ui.label("Clear all relays");
    });
}

fn draw_destination_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Destination", |ui| {
        if ui.button("Csv file").clicked() {
            state.show_dialog = true;
        };
        ui.label("JSON array file");
        ui.label("JSON object file");
        ui.label("Sql Server");
        ui.label("Sqlite");
    });
}

fn draw_run_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Run", |ui| {
        if ui.button("Parse file").clicked() {
            state.show_dialog = true;
        };
        ui.label("Parse file");
    });
}

fn draw_help_menu(ui: &mut egui::Ui, state: &mut RivvitiumApp) {
    ui.menu_button("Help", |ui| {
        ui.label("What's new in Rivvitium");
        if ui.button("About").clicked() {
            state.show_dialog = true; // open the window next frame
            ui.close_menu(); // collapse the menu
        }
    });
}
