
use std::fmt::Debug;
use eframe::egui;
use crate::ui::dialogs;
use crate::ui::menu::draw_main_menu;
use crate::ui::regions::action_panel::draw_action_panel;
use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::regions::footer::draw_footer;
use crate::ui::regions::header::draw_header;

// This is the main application. It both drawing particulars
// and state values
//
pub struct RivvitiumApp {
    pub click_count:    u32,
    pub show_dialog:    bool,
    pub image_about:    Option<egui::TextureHandle>,
}

impl RivvitiumApp {
    /// Decode the embedded PNG and upload it to the GPU _once_.
    fn ensure_logo_loaded(&mut self, ctx: &egui::Context) {
        if self.image_about.is_some() {
            return;
        }

        let bytes = include_bytes!("../assets/riv.bars.png"); // 1. read the bytes that we embedded in the binary:
        let img  = image::load_from_memory(bytes)
            .expect("valid png")
            .to_rgba8(); // 2. turn them into rgba pixels with the `image` crate:
        let size = [img.width() as usize, img.height() as usize];
        let image = egui::ColorImage::from_rgba_premultiplied(size, &img); // 3. give the pixels to egui so it becomes a GPU texture:
        let tex = ctx.load_texture("logo_texture", image, egui::TextureOptions::default());
        self.image_about = Some(tex);
    }
}

impl Default for RivvitiumApp {
	fn default() -> Self {
		let click_count     = 0;
		let show_dialog     = false;
		let image_about     = None;
		RivvitiumApp{click_count, show_dialog, image_about}
	}
}

impl Debug for RivvitiumApp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ApplicationState")
		.field("click_count", &self.click_count)
		.field("show_dialog", &self.show_dialog)
		.finish()
	}
}

impl eframe::App for RivvitiumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Prep
        //
        self.ensure_logo_loaded(ctx);
        
        // Menu
        //
        draw_main_menu(self, ctx);
        
        // Draw
        //
        egui::CentralPanel::default().show(ctx, |ui| {
		      draw_header(ui);
            ui.add_space(10.0);
            ui.separator();
            draw_button_bar(self, ui);
            draw_action_panel(self, ui);
		      draw_footer(ui);
		      ui.add_space(10.0);
        });

        //  Dialogs
        //
        if self.show_dialog {
			dialogs::about_dialog::open_about_dialog(self, ctx);
        }
    }
}
