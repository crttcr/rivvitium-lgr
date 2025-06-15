use crate::error::{Error, IoErrorWrapper};
use crate::model::ir::atom::Atom;
use crate::component::sink::{Sink, SinkKind};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::time::Instant;
use tracing::{info, instrument, warn};
use crate::model::ir::atom_type::AtomType;
use csv::Writer;
use zero::component::telemetry::component_metrics::ComponentMetrics;
use zero::component::telemetry::provides_metrics::ProvidesMetrics;
use crate::component::sink::sink_settings::SinkSettings;

const OUTPUT_PATH: &str = "/tmp";

#[derive(Debug)]
pub struct CsvSink {
	component_id:     u32,
	file_path:        PathBuf,
	delimiter:        char,
	writer:           Option<Writer<File>>,
	created_utc:      Instant,
	started_utc:      Instant,
	metrics:          ComponentMetrics,
	tx:               Sender<ComponentMetrics>
}

impl CsvSink {
	pub fn new(component_id: u32, file_path: PathBuf, delimiter: char, tx: Sender<ComponentMetrics>) -> Self {
		let created_utc = Instant::now();
		let started_utc = created_utc;
		let metrics     = ComponentMetrics::new(component_id);
		Self {
			component_id,
			file_path,
			delimiter,
			writer: None,
			created_utc,
			started_utc,
			metrics,
			tx
		}
	}

	pub fn start(&mut self) {
		self.started_utc = Instant::now();
   	self.metrics.activate();
		}

   pub fn close(&mut self) {
   	self.metrics.complete();
   	}
}

impl Sink for CsvSink {
	fn kind(&self) -> SinkKind { SinkKind::Csv }
	
	#[instrument]
	fn initialize(&mut self, cfg: &SinkSettings) -> Result<(), Error> {
		let full_path = Path::new(OUTPUT_PATH).join(&self.file_path);             // 1) Build the full path: "/tmp/<file_path>"
		let file      = File::create(&full_path)                                         // 2) Attempt to create (or truncate) the file for writing
            .map_err(|e| {
                let wrapper = IoErrorWrapper::from(e);               // Wrap the std::io::Error into your Error::Io
                Error::from(wrapper)
            })?;
		let writer = Writer::from_writer(file);
		self.writer = Some(writer);
		self.metrics.reset();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		self.metrics.increment_messages();
		if atom.atom_type() == AtomType::Control {return Ok(())}
		let mut writer = self.writer.take().unwrap();
		let e_map      = |e: csv::Error| { Error::from(e) };
		
		match atom {
			Atom::ByteRowAtom(byte_row) => {
				let vec  = byte_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			Atom::StringRowAtom(string_row) => {
				let vec  = string_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			Atom::HeaderRow(header_row) => {
				let vec  = header_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			_ => {},
		}
		self.writer = Some(writer);
		Ok(())
	}

	fn close(&mut self) {
		if let Some(buf) = self.writer.take() {
			let mut inner = buf.into_inner();
			match inner {
				Err(error)       => { 
					let msg = format!("Error encountered with writer: {}", error);
					warn!("{}", msg);
					},
				Ok(ref mut file) => { let _ = file.flush(); },
			}
		} else {
			let msg = "Finish called but struct contains no writer.".to_owned();
			warn!("{}", msg);
		}
	}
}

impl ProvidesMetrics for CsvSink {
	fn metrics(&self) -> ComponentMetrics {
		self.metrics.clone()
	}

	fn take_metrics(&mut self) -> ComponentMetrics {
		let rv = self.metrics.clone();
		self.metrics.reset();
		rv
	}
}
