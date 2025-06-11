
use apex::AppState;
use crate::ui::regions::action_panel::draw_main_panel;
use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::regions::footer::draw_footer;
use crate::ui::regions::header::draw_header;
use crate::ui::{dialogs, UiState};
use crate::ui::menu::create_menu_bar;
use std::fmt::Debug;
use std::sync::mpsc;
use apex::engines::riv::RivCommand;
use crate::ui::regions::ActiveAction;
use crate::ui::visuals::colors::ColorTheme;
use tracing::{info, warn};
use apex::engines::riv::riv_parser::RivParser;
use apex::state::parse_detail_dto::ParseDetailDTO;
use riv::model::ir::atom::Atom;

// This is the main application. It both drawing particulars
// and state values
//
pub struct RivvitiumApp {
    pub app_state:      AppState,
    pub app_settings:   ColorTheme,
    pub cmd_tx:         mpsc::Sender<RivCommand>,
    pub atom_rx:        mpsc::Receiver<Atom>,
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
		if let Some(path) = self.app_state.get_selected_file() {
			let file = path.clone();
			let cmd  = RivCommand::Parse {file};
			match self.cmd_tx.send(cmd) {
				Ok(_) => {
					let str = path.to_str().unwrap();
					let dto = ParseDetailDTO::new(str);
					self.app_state.with_dto(dto);
					self.ui_state.set_active_panel(ActiveAction::ParseInProgress);
				},
				Err(x) => {
					warn!("Send parse command failed: {}", x);
				}
			}
		} else {
			warn!("Application state does not contain an input file. Parse command was not sent.");
		}
	}
}

impl Default for RivvitiumApp {
	fn default() -> Self {
		let app_state          = AppState::default();
		let app_settings       = ColorTheme::random();
		let ui_state           = UiState::default();
		let (cmd_tx, cmd_rx)   = mpsc::channel();
		let (atom_tx, atom_rx) = mpsc::channel();
		RivParser::spawn(cmd_rx, atom_tx);
		RivvitiumApp{app_state, app_settings, cmd_tx, atom_rx, ui_state}
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
        // ── 1. pump the progress channel  ───────────────────────────────
        while let Ok(atom) = self.atom_rx.try_recv() {
				match atom {
					Atom::HeaderRow(row) => {
						info!("Headers: {:?}", row);
						println!("Headers: {:?}", row);
						if let Some(dto) = &self.app_state.get_parse_detail() {
							self.ui_state.set_active_panel(ActiveAction::ParseComplete);
							let revised = dto.finished();
							self.app_state.with_dto(revised)
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
					_ => println!("Received atom: {:?}", atom),
				}
            ctx.request_repaint();  // keep UI fluid even if worker is slow
        }
        self.ensure_logo_loaded(ctx);



        // Menu
        //
        create_menu_bar(self, ctx);

			if self.ui_state.is_sink_dialog_visible() {
        		if let Some(cfg) = self.ui_state.sink_dialog.show(ctx) {
            	println!("Sink chosen: {cfg:?}");
        			self.app_state.set_sink_config(cfg);
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
