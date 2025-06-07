use crate::app::rivvitium_app::RivvitiumApp;
use eframe::emath::Align;
use egui::Layout;

pub fn draw_action_panel(app: &mut RivvitiumApp, ui: &mut egui::Ui) {
    ui.label("Now, a label and a button side-by-side:");
    ui.add_space(10.0); // Add some space before the horizontal group
    ui.horizontal(|ui| {
        // <--- This is the key: a horizontal sub-UI
        if ui.button("Click me!").clicked() {
            app.click_count += 1;
        }
        ui.label(format!("Button clicked {} times!", app.click_count));
    });

    ui.add_space(10.0);
    ui.label(format!("Button clicked {} times!", app.click_count));
    if ui.button("Click me!").clicked() {
        app.click_count += 1;
    }

    ui.add_space(20.0);
    if ui.button("Reset Counter").clicked() {
        app.click_count = 0;
    }

    ui.add_space(20.0);
    
    // Another horizontal group for the reset button, also centered and same width
    ui.with_layout(
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Reset the counter:");
                if ui.button("Reset Counter").clicked() {
                    app.click_count = 0;
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
