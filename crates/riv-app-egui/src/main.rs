//! src/main.rs
//! egui starter app that shows a PNG inside a dialog (“window”) when you
//! press the **Show Dialog** button.

use eframe::{egui, NativeOptions, Renderer};

#[derive(Default)]
struct MyApp {
    // --- existing state ----------------------------------------------------
    click_count: u32,

    // --- new state ---------------------------------------------------------
    show_dialog:         bool,                 // should the dialog be visible?
    image_about:                Option<egui::TextureHandle>, // GPU texture for the PNG
}

impl MyApp {
    /// Decode the embedded PNG and upload it to the GPU _once_.
    fn ensure_logo_loaded(&mut self, ctx: &egui::Context) {
        if self.image_about.is_some() { return; }

        let bytes = include_bytes!("assets/riv.bars.png");                           // 1. read the bytes that we embedded in the binary:
        let img   = image::load_from_memory(bytes).expect("valid png").to_rgba8();    // 2. turn them into rgba pixels with the `image` crate:
        let size  = [img.width() as usize, img.height() as usize];
        let image = egui::ColorImage::from_rgba_premultiplied(size, &img);            // 3. give the pixels to egui so it becomes a GPU texture:
        let tex   = ctx.load_texture("logo_texture", image, egui::TextureOptions::default());
        self.image_about = Some(tex);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Make sure the texture exists before we might want to show it.
        self.ensure_logo_loaded(ctx);
        // ------------------------  menu bar  ------------------------------
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // ----- “File” menu ---------------------------------------
                ui.menu_button("File", |ui| {
                    if ui.button("Parse file ...").clicked() { 
								self.show_dialog = true;
                    };
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // ----- “Relay" menu ---------------------------------------
                ui.menu_button("Relay", |ui| {
                    ui.label("Add Relay");
                    ui.label("Remove Relay");
                    ui.label("Reorder Relays");
                });

                // ----- “Destination" menu ---------------------------------------
                ui.menu_button("Destination", |ui| {
                    ui.label("Csv file");
                    ui.label("JSON array file");
                    ui.label("JSON object file");
                    ui.label("Sql Server");
                    ui.label("Sqlite");
                });

                // ----- "Help" menu ---------------------------
                ui.menu_button("Help", |ui| {
                    ui.label("What's new in Rivvitium");
                    if ui.button("About").clicked() {
                        self.show_dialog = true; // open the window next frame
                        ui.close_menu(); // collapse the menu
                    }
                });
            });
        });

        // -------------------------------------------------------------------
        //  Main UI
        // -------------------------------------------------------------------
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Simple Starter App");
            ui.add_space(10.0);

            ui.label(format!("Button clicked {} times!", self.click_count));

            if ui.button("Click me!").clicked() {
                self.click_count += 1;
            }

            ui.add_space(20.0);

            if ui.button("Reset Counter").clicked() {
                self.click_count = 0;
            }

            ui.add_space(20.0);

            // --------------------------------------------------------------
            //    NEW BUTTON that toggles the dialog
            // --------------------------------------------------------------
            if ui.button("Show Dialog").clicked() {
                self.show_dialog = true; // open the dialog next frame
            }
        });

        // -------------------------------------------------------------------
        //  Dialog window that shows the PNG
        // -------------------------------------------------------------------
        if self.show_dialog {
            egui::Window::new("About Rivvitium")
                .open(&mut self.show_dialog) // adds the little **×** close button
                .resizable(true)
                .collapsible(false)
                .show(ctx, |ui| {
                    if let Some(tex) = &self.image_about {
                        let size  = tex.size_vec2();
                        let small = size * 0.25;        // scale to reasonable size
                        ui.image((tex.id(), small));
                    } else {
                        ui.label("Loading image …");
                    }
                });
        }
    }
}

fn main() -> eframe::Result<()> {
	let size       = egui::vec2(800.0, 600.0);
	let viewport   = egui::ViewportBuilder::default().with_app_id("Rivvitium").with_inner_size(size);
	let renderer   = Renderer::Wgpu;
	let options    = NativeOptions { viewport, renderer, ..Default::default() };
	eframe::run_native("Rivvitium", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}
