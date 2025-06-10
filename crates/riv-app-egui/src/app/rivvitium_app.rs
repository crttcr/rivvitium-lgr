
use apex::AppState;
use crate::ui::regions::action_panel::draw_main_panel;
use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::regions::footer::draw_footer;
use crate::ui::regions::header::draw_header;
use crate::ui::{dialogs, UiState};
use crate::ui::menu::create_menu_bar;
use std::fmt::Debug;
use crate::ui::visuals::colors::ColorTheme;

// This is the main application. It both drawing particulars
// and state values
//
pub struct RivvitiumApp {
    pub app_state:      AppState,
    pub app_settings:   ColorTheme,
    pub ui_state:       UiState,
}

impl RivvitiumApp {
    /// Decode the embedded PNG and upload it to the GPU _once_.
    fn ensure_logo_loaded(&mut self, ctx: &egui::Context) {
        if self.ui_state.has_about_dialog_texture() { return; }

        let bytes = include_bytes!("../assets/riv.bars.png");                      // 1. read the bytes to embed in binary
        let img   = image::load_from_memory(bytes).expect("valid png").to_rgba8(); // 2. turn into rgba pixels
        let size  = [img.width() as usize, img.height() as usize];
        let image = egui::ColorImage::from_rgba_premultiplied(size, &img);         // 3. give pixels to egui. Becomes a GPU texture
        let tex   = ctx.load_texture("logo_texture", image, egui::TextureOptions::default());
        self.ui_state.set_about_dialog_texture(tex);
    }
}

impl Default for RivvitiumApp {
	fn default() -> Self {
		let app_state       = AppState::default();
		let app_settings    = ColorTheme::random();
		let ui_state        = UiState::default();
		RivvitiumApp{app_state, app_settings, ui_state}
	}
}

impl Debug for RivvitiumApp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ApplicationState")
		.field("click_count",    &self.app_state.click_count())
		.field("about_visiblie", &self.ui_state.is_about_dialog_visible())
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
			// ───────────────────── header ──────────────────────
		egui::TopBottomPanel::top("header").show(ctx, |ui| {
			draw_header(ui);
		});

			// ───────────────────── footer ──────────────────────
			egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
		      draw_footer(ui);
	    });        
			// ───────────────────── main content ────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            draw_button_bar(self, ui);
            draw_main_panel(self, ui);
		      ui.add_space(10.0);
        });

        //  Dialogs
        //
        if self.ui_state.is_about_dialog_visible() {
			dialogs::about_dialog::open_about_dialog(self, ctx);
        }
    }
}
