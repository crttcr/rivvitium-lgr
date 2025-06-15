// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::{Sink, SinkKind};

use std::fmt::{Debug, Display};
use std::sync::mpsc::Sender;
use std::time::Instant;
use rusqlite::Connection;
use tracing::{info, instrument, warn};
use zero::component::telemetry::component_metrics::{ComponentMetrics, ComponentStatus};
use zero::component::telemetry::provides_metrics::ProvidesMetrics;
use crate::component::sink::sink_settings::SinkSettings;

#[derive(Debug)]
pub struct SqlServerSink {
	component_id: u32,
	server:       String,
	port:         u16,
	user_name:    String,
	password:     String,
	db_name:      String,
	created_utc:  Instant,
	started_utc:  Instant,
	metrics:      ComponentMetrics,
	tx:           Sender<ComponentMetrics>
}

impl SqlServerSink {
	pub fn new(component_id: u32, server: String, port: u16,
		user_name: String, password: String, db_name: String,
		tx: Sender<ComponentMetrics>
		) -> Self {
		let created_utc = Instant::now();
		let started_utc = created_utc;
		let metrics     = ComponentMetrics::new(component_id);
		Self {
			component_id,
			server,
			port,
			user_name,
			password,
			db_name,
			created_utc, started_utc, metrics, tx}
	}

	pub fn start(&mut self) {
		self.started_utc = Instant::now();
		self.metrics.activate();
	}

	pub fn close(&mut self) {
		self.metrics.complete();
	}
}

impl Sink for SqlServerSink {
	fn kind(&self) -> SinkKind { SinkKind::Capture }

	#[instrument]
	fn initialize(&mut self, _cfg: &SinkSettings) -> Result<(), Error> {
		self.metrics.reset();
		Ok(())
	}

	fn accept(&mut self, _atom: Atom) -> Result<(), Error> {
		self.metrics.increment_messages();
		Ok(())
	}

	#[instrument]
	fn close(&mut self) {}
}

impl ProvidesMetrics for SqlServerSink {
    fn metrics(&self) -> ComponentMetrics {
    	self.metrics.clone()
    }

	fn take_metrics(&mut self) -> ComponentMetrics {
		let rv = self.metrics.clone();
		self.metrics.reset();
		rv
	}
}
