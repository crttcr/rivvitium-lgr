
use tracing::{info, instrument, warn};
use zero::util::file_utils::assert_readable;
use std::path::PathBuf;
use riv::component::sink::SinkKind;
use crate::state::parse_detail_dto::ParseDetailDTO;
use crate::state::sink_config::SinkConfig;

pub struct AppState {
	click_count:       u32,
	selected_file:     Option<PathBuf>,   // ① store the path
	parse_detail:      Option<ParseDetailDTO>, 
	sink_config:       SinkConfig,
}

impl Default for AppState {
	fn default() -> Self {
		let click_count   = 0;
		let selected_file = None;
		let parse_detail  = None;
		let sink_config   = SinkConfig::default();
		Self{click_count, selected_file, parse_detail, sink_config}
	}
}

impl AppState {
	pub fn with_source(self, selected_file: PathBuf) -> Self {
		match assert_readable(&selected_file) { 
			Ok(_)  => Self {selected_file : Some(selected_file),..self},
			Err(e) => {
				  warn!("⚠ Bad file. Not updating source: {}: {}", selected_file.display(), e);
				  self
			}
		 }
	}
	
	pub fn set_source_path(&mut self, selected_file: PathBuf) {
		match assert_readable(&selected_file) { 
			Ok(_)  => {
				info!("Updating source: {}", selected_file.display());
				self.selected_file = Some(selected_file)
			},
			Err(e) => {
				  warn!("⚠ Bad file. Not updating source: {}: {}", selected_file.display(), e);
			}
		 }
	}
	
	pub fn with_dto(&mut self, dto: ParseDetailDTO) -> () {
		self.parse_detail.replace(dto);
	}
	
	pub fn set_sink_config(&mut self, cfg: SinkConfig) -> () {
		self.sink_config = cfg;
	}
		
	pub fn get_sink_config(&self) -> SinkConfig {
		self.sink_config.clone()
	}
	
	pub fn get_sink_config_mut(&mut self) -> &mut SinkConfig {
		&mut self.sink_config
	}
	
	pub fn get_parse_detail_mut(&mut self) -> &mut Option<ParseDetailDTO> {
		&mut self.parse_detail
	}
	pub fn get_parse_detail(&self) -> Option<&ParseDetailDTO> {
		self.parse_detail.as_ref()
	}
	
	pub fn get_selected_file(&self) -> Option<&PathBuf> {
		self.selected_file.as_ref()
	}
	
	pub fn get_selected_file_mut(&mut self) -> Option<&mut PathBuf> {
		self.selected_file.as_mut()
	}
	
	#[instrument(skip(self))]	
	pub fn teardown(&mut self) {
		self.selected_file = None;
	}
	
	pub fn close_file(&mut self) {
		match &self.selected_file {
		Some(path) => {
				info!("Closing file: {}", path.display());
				self.selected_file = None;
			},
			None => {
				warn!("No file to close");
			}
		}
	}
	
	pub fn click_count(&self) -> u32 { self.click_count }
	pub fn reset_clicks(&mut self) {
			self.click_count = 0
	}
	
	pub fn capture_click(& mut self) {
			self.click_count += 1;
	}
	
	pub fn set_click_count(&mut self, count: u32) {
			self.click_count += count;
	}
	
	
	// Predicates
	//
	pub fn has_selected_file(&self)        -> bool { self.selected_file.is_some() }
	pub fn has_selected_relays(&self)      -> bool { false                        }
	pub fn can_run_pipeline(&self)         -> bool { self.has_selected_file()     }
	pub fn sink_permits_publish(&self)     -> bool { 
		match self.sink_config.kind() {
			SinkKind::Csv    => true,
			SinkKind::Json   => true,
			SinkKind::Kafka  => true,
			SinkKind::Sqlite => true,
			_ => false,
		}
	}
}

/*
fn pick_file(&mut self, ctx: &egui::Context) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Choose input")
        .pick_file()
    {
        self.selected_file = Some(path);
    }
}

fn process(&mut self) -> anyhow::Result<()> {
    let path = self.selected_file.as_ref().ok_or_else(|| anyhow!("no file"))?;
    let mut reader = std::fs::File::open(path)?;
    // … read / parse …
    Ok(())
}
*/