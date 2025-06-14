// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::{Sink, SinkConfig, SinkKind};

use std::fmt::{Debug, Display};
use rusqlite::Connection;
use tracing::{info, instrument, warn};

#[derive(Debug)]
pub struct CaptureSink {
	atoms:       Vec<Atom>,
	is_closed:   bool,
}

impl CaptureSink {
	pub fn new() -> Self {
		Self { atoms: Vec::new(), is_closed: false }
	}

	pub fn into_atoms(self) -> Vec<Atom> {
		self.atoms
	}
}

impl Sink for CaptureSink {
	fn kind(&self) -> SinkKind { SinkKind::Capture }
        
	#[instrument]
	fn initialize(&mut self, _cfg: &dyn SinkConfig) -> Result<(), Error> {
		self.atoms.clear();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		if self.is_closed {
			let msg = "CaptureSink already closed. Not accepting new atoms.";
			warn!(msg);
			Err(Error::InvalidInput(msg.into()))
		} else {
			self.atoms.push(atom);
			Ok(())
		}
	}

	#[instrument]
	fn close(&mut self) {}
}
