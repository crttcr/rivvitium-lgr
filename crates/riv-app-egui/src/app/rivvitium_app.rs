
use apex::AppState;
use crate::ui::regions::action_panel::{draw_action_panel, draw_ready_panel, draw_run_panel, draw_result_panel};
use crate::ui::regions::ActiveAction;
use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::regions::footer::draw_footer;
use crate::ui::regions::header::draw_header;
use crate::ui::dialogs;
use crate::ui::menu::create_menu_bar;
use std::fmt::Debug;

// This is the main application. It both drawing particulars
// and state values
//
pub struct RivvitiumApp {
//    pub click_count:    u32,
    pub show_dialog:    bool,
    pub image_about:    Option<egui::TextureHandle>,
    pub active_panel:   ActiveAction,
    pub app_state:      AppState,
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
		let show_dialog     = false;
		let image_about     = None;
		let active_panel    = ActiveAction::Home;
		let app_state       = AppState::default();
		RivvitiumApp{show_dialog, image_about, active_panel, app_state}
	}
}

impl Debug for RivvitiumApp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ApplicationState")
		.field("click_count", &self.app_state.click_count())
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
        create_menu_bar(self, ctx);
        
        // Draw
        //
        egui::CentralPanel::default().show(ctx, |ui| {
		      draw_header(ui);
            ui.add_space(10.0);
            ui.separator();
            draw_button_bar(self, ui);
            match self.active_panel {
            	ActiveAction::Ready  => {draw_ready_panel(self, ui)}
            	ActiveAction::Run    => {draw_run_panel(self, ui)}
            	ActiveAction::Result => {draw_result_panel(self, ui)}
            	_                    => {draw_action_panel(self, ui)}
            }
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
