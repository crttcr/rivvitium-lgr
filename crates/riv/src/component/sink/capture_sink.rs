// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::{Sink, SinkKind};

use std::fmt::{Debug, Display};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct CaptureSink {
	atoms: Vec<Atom>,
}

impl CaptureSink {
	pub fn new() -> Self {
		Self { atoms: Vec::new() }
	}

	pub fn into_atoms(self) -> Vec<Atom> {
		self.atoms
	}
}

impl Sink<Vec<Atom>> for CaptureSink {
	fn kind(&self) -> SinkKind { SinkKind::Capture }
        
	#[instrument]
	fn initialize<C: Display + Debug>(&mut self, _cfg: &C) -> Result<(), Error> {
		self.atoms.clear();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		self.atoms.push(atom);
		Ok(())
	}

	#[instrument]
	fn finish(&mut self) -> Result<Vec<Atom>, Error> {
		let rv: Vec<Atom> = self.atoms.drain(..).collect();
		Ok(rv)
	}
}
