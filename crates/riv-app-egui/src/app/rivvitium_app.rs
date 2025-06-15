
use apex::AppState;
// use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::regions::footer::draw_footer;
use crate::ui::regions::header::draw_header;
use crate::ui::{dialogs, UiState};
use crate::ui::menu::create_menu_bar;
use std::fmt::Debug;
use std::sync::mpsc;
use apex::engines::riv::RivCommand;
use crate::ui::regions::ApplicationStatus;
use crate::ui::visuals::colors::ColorTheme;
use tracing::{info, warn};
use zero::component::telemetry::component_metrics::ComponentMetrics;
use crate::ui::layouts::draw_main_screen;

// This is the main application. It both drawing particulars
// and state values
//
pub struct RivvitiumApp {
    pub app_state:      AppState,
    pub app_settings:   ColorTheme,
    pub metric_rx:      mpsc::Receiver<ComponentMetrics>,
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

impl RivvitiumApp {
	pub fn fire_parse_command(&mut self) {
		match self.app_state.start_parse() {
			Ok(component_id) => {
				println!("Do something with this! {}", component_id);
				self.ui_state.set_application_status(ApplicationStatus::Running);
			}
			Err(err) => {
					warn!("start parse failed: {}", err);
			}
		}
	}
	pub fn fire_analyze_command(&mut self) {}
	pub fn fire_blueprint_command(&mut self) {}
	
	pub fn fire_publish_command(&mut self) {
		if !self.app_state.can_publish() {
			warn!("Application state does have a publishable configuration. Publish command was not sent.");
			return
		}
		let _cmd  = RivCommand::Publish;
		info!("Publish command handled.");
		// Create a sink.
		// Connect self.atom_rx to the sink
		// Send a publish command
	}
}

impl Default for RivvitiumApp {
	fn default() -> Self {
		let (metric_tx, metric_rx) = mpsc::channel::<ComponentMetrics>();
		let app_state              = AppState::new(metric_tx);
		let app_settings           = ColorTheme::random();
		let ui_state               = UiState::default();
//		RivParser::spawn(cmd_rx, metric_tx);
		RivvitiumApp{app_state, app_settings, metric_rx, ui_state}
	}
}

impl Debug for RivvitiumApp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ApplicationState")
		.field("about_visible", &self.ui_state.is_about_dialog_visible())
		.finish()
	}
}

impl eframe::App for RivvitiumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Prep
        //
        // ── 1. pump the progress channel  ───────────────────────────────
        while let Ok(metrics) = self.metric_rx.try_recv() {
				println!("Received metrics: {:?}", metrics);
/*				match atom {
					Atom::HeaderRow(row) => {
						info!("Headers: {:?}", row);
						println!("Headers: {:?}", row);
						if let Some(dto) = &self.app_state.get_parse_detail() {
							self.ui_state.set_active_panel(ActiveAction::ParseInProgress);
							println!("Capture headers in DTO{:?}", dto);
						}
					},
					Atom::EndTask => {
						println!("End task: {:?}", atom);
						if let Some(dto) = &self.app_state.get_parse_detail() {
							self.ui_state.set_active_panel(ActiveAction::ParseComplete);
							let revised = dto.finished();
							self.app_state.with_dto(revised)
						}
					},
					_ => println!("Received metrics: {:?}", metrics),
				}
 */
            ctx.request_repaint();  // keep UI fluid even if worker is slow
        }
        self.ensure_logo_loaded(ctx);



        // Menu
        //
        create_menu_bar(self, ctx);

			if self.ui_state.is_sink_dialog_visible() {
        		if let Some(cfg) = self.ui_state.sink_dialog.show(ctx) {
            	println!("Sink chosen: {cfg:?}");
        			self.app_state.set_sink_config(&cfg);
        			self.ui_state.set_sink_dialog_invisible();
			}
        }
        
        /*     // Draw a Sink Dialog box ...
       */
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
			
		draw_main_screen(self, ctx);	

        //  Dialogs
        //
        if self.ui_state.is_about_dialog_visible() {
			dialogs::about_dialog::open_about_dialog(self, ctx);
        }
    }
}
