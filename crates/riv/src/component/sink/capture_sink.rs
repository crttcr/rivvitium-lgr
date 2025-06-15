// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::{Sink, SinkKind};

use std::fmt::{Debug, Display};
use std::mem;
use std::sync::mpsc::Sender;
use std::time::Instant;
use rusqlite::Connection;
use tracing::{info, instrument, warn};
use zero::component::telemetry::component_metrics::{ComponentMetrics, ComponentStatus};
use zero::component::telemetry::provides_metrics::ProvidesMetrics;
use crate::component::sink::sink_settings::SinkSettings;

#[derive(Debug)]
pub struct CaptureSink {
	component_id: u32,
	created_utc:  Instant,
	started_utc:  Instant,
	atoms:        Vec<Atom>,
	metrics:      ComponentMetrics,
	tx:           Sender<ComponentMetrics>
}

impl CaptureSink {
	pub fn new(component_id: u32, tx: Sender<ComponentMetrics>) -> Self {
		let atoms       = Vec::new();
		let created_utc = Instant::now();
		let started_utc = created_utc;
		let metrics     = ComponentMetrics::new(component_id);
		Self {component_id, created_utc, started_utc, atoms, metrics, tx}
	}

	pub fn new_with_atoms(component_id: u32, atoms: Vec<Atom>, tx: Sender<ComponentMetrics>) -> Self {
		let created_utc = Instant::now();
		let started_utc = created_utc;
		let metrics     = ComponentMetrics::new(component_id);
		Self {component_id, created_utc, started_utc, atoms, metrics, tx}
	}

	pub fn start(&mut self) {
		self.started_utc = Instant::now();
		self.metrics.activate();
	}
	
	pub fn close(&mut self) {
		self.metrics.complete();
	}
	
	pub fn into_atoms(self) -> Vec<Atom> {
		println!("CaptureSink::into_atoms");
		self.atoms
	}
}

impl Sink for CaptureSink {
	fn kind(&self) -> SinkKind { SinkKind::Capture }
        
	#[instrument]
	fn initialize(&mut self, _cfg: &SinkSettings) -> Result<(), Error> {
		self.atoms.clear();
		self.metrics.reset();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		println!("CaptureSink::accept: {:?}", atom);
		self.metrics.increment_messages();
		if self.metrics.status == ComponentStatus::Completed {
			let msg = "CaptureSink: Completed. No new atoms";
			warn!(msg);
			Err(Error::InvalidInput(msg.into()))
		} else {
			self.atoms.push(atom);
			Ok(())
		}
	}
	
	fn drain_atoms(&mut self) -> Vec<Atom> {
		mem::take(&mut self.atoms)
   }

	#[instrument]
	fn close(&mut self) {}
}

impl ProvidesMetrics for CaptureSink {
    fn metrics(&self) -> ComponentMetrics {
    	self.metrics.clone()
    }

	fn take_metrics(&mut self) -> ComponentMetrics {
		let rv = self.metrics.clone();
		self.metrics.reset();
		rv
	}
}
