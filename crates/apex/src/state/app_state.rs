
use tracing::{info, instrument, warn};
use zero::util::file_utils::assert_readable;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use riv::component::sink::SinkKind;
use riv::component::source::path_buf_config::PathBufConfig;
use riv::component::source::SourceConfig;
use riv::Error;
use zero::telemetry::component::ComponentMetrics;
use crate::engines::riv::engine::Engine;
use crate::engines::riv::parse_helper::open_source;
use crate::engines::riv::config::Config;
use crate::state::parse_detail_dto::ParseDetailDTO;
use crate::state::sink_config::SinkConfig;

pub struct AppState {
	metric_tx:      Sender<ComponentMetrics>,
	parse_detail:   Option<ParseDetailDTO>,
	sink_config:    SinkConfig,
	config:         Config,
	engine:         Option<Engine>,
}

impl AppState {
	pub fn new(metric_tx: Sender<ComponentMetrics>) -> Self {
		let parse_detail     = None;
		let sink_config      = SinkConfig::default();
		let pipeline_builder = Config::default();
		Self{metric_tx, parse_detail, sink_config, config:pipeline_builder, engine: None}
	}
}

// Actions
impl AppState {

	// FIXME: Build the engine and start it ..
	pub fn start_parse(&mut self) -> Result<u32, Error> {
		if !self.can_parse() {
			let error = Error::General("Must have a valid input to start parse.".to_string());
			return Err(error);
		}
		match self.config.build() {
			Ok(engine) => {
			self.engine = Some(engine);
				Ok(0)
			}
			Err(error) => {
				let msg = format!("Failed to construct pipeline: {:?}", error);
				let err = Error::General(msg);
				Err(err)
			}
		}
	}
}

impl AppState {
	pub fn set_source_path(&mut self, selected_file: PathBuf) {
		match assert_readable(&selected_file) { 
			Ok(_)  => {
				info!("Updating source: {}", selected_file.display());
				match open_source(&selected_file) {
					Ok(src)  => {
						let config = PathBufConfig::new(selected_file);
						let config = Box::new(config);
						self.config.source(config);
					}
					Err(e) => {
						warn!("⚠ Bad file. Not updating source: {}: {}", selected_file.display(), e);
					}
				}
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
	
	
	#[instrument(skip(self))]	
	pub fn teardown(&mut self) {
	}
	
	pub fn close_source_file(&mut self) {
		&self.config.source_reset();
	}

	pub fn clear_relays(&mut self) {
		self.config.relay_clear();
	}

	// Predicates
	//
	pub fn can_parse(&self)                -> bool { self.config.can_parse() }
	pub fn can_analyze(&self)              -> bool { false }
	pub fn can_blueprint(&self)            -> bool { false }
	pub fn can_publish(&self)              -> bool { self.config.can_publish() }

	pub fn has_selected_relays(&self)      -> bool { false                        }
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
