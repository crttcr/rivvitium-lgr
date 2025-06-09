use crate::app::rivvitium_app::RivvitiumApp;
use eframe::emath::Align;
use egui::Layout;
use crate::ui::regions::ActiveAction;

pub fn draw_main_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	match app.ui_state.active_panel() {
		ActiveAction::Ready  => {draw_ready_panel(app, ui)}
            	ActiveAction::Run    => {draw_run_panel(   app, ui)}
            	ActiveAction::Result => {draw_result_panel(app, ui)}
            	_                    => {draw_action_panel(app, ui)}
	}
}

fn draw_ready_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	ui.label("Ready to parse.");
	ui.separator();
	ui.add_space(3.0);
	if ui.button("Click to simulate run").clicked() {
            app.ui_state.set_active_panel(ActiveAction::Run);
            app.app_state.capture_click();
   }
}


fn draw_run_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	ui.label("Running like the wind.");
	ui.separator();
    ui.add_space(5.0); // Add some space before the horizontal group
	if ui.button("Click to reset").clicked() {
            app.ui_state.set_active_panel(ActiveAction::Result);
            app.app_state.set_click_count(2_000_000);
   }
}

fn draw_result_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
	ui.label("Here's a result.");
	ui.separator();
	ui.add_space(10.0); // Add some space before the horizontal group
	if ui.button("Click to set panel to Home").clicked() {
        		app.ui_state.set_active_panel(ActiveAction::Home);
            app.app_state.set_click_count(2_000_000);
   }
}

fn draw_action_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
    ui.label("Now, a label and a button side-by-side:");
    ui.add_space(10.0); // Add some space before the horizontal group
    ui.horizontal(|ui| {
        // <--- This is the key: a horizontal sub-UI
        if ui.button("Click to make ready").clicked() {
        		app.ui_state.set_active_panel(ActiveAction::Ready);
        }
        ui.label(format!("Button clicked {} times!", app.app_state.click_count()));
    });

    ui.add_space(10.0);
    ui.label(format!("Button clicked {} times!", app.app_state.click_count()));
    if ui.button("Click me to increase the count!").clicked() {
        app.app_state.capture_click();
    }

    ui.add_space(20.0);
    if ui.button("Reset Counter").clicked() {
        app.app_state.reset_clicks();
    }

    ui.add_space(20.0);
    
    // Another horizontal group for the reset button, also centered and same width
    ui.with_layout(
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Reset the counter:");
                if ui.button("Reset Counter").clicked() {
        					app.app_state.reset_clicks();
                }
            });
        },
    );

    ui.with_layout(
        Layout::top_down_justified(Align::Min), // vertical stack, every child gets max width
        |ui| {
            if ui.button("Full-width action").clicked() { /* … */ }
        },
    );
    ui.separator();
}
