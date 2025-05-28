// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::Sink;

use std::fmt::{Debug, Display};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct CaptureSink<'a> {
	atoms: Vec<Atom<'a>>,
}

impl<'a> CaptureSink<'a> {
	pub fn new() -> Self {
		Self { atoms: Vec::new() }
	}

	pub fn into_atoms(self) -> Vec<Atom<'a>> {
		self.atoms
	}
}

impl<'a> Sink<'_, Vec<Atom<'a>>> for CaptureSink<'a> {
	#[instrument]
	fn initialize<C: Display + Debug>(&mut self, _cfg: &C) -> Result<(), Error> {
		self.atoms.clear();
		Ok(())
	}

	fn accept(&mut self, atom: Atom<'_>) -> Result<(), Error> {
		self.atoms.push(atom);
		Ok(())
	}

	#[instrument]
	fn finish(&mut self) -> Result<Vec<Atom<'a>>, Error> {
		let rv: Vec<Atom> = self.atoms.drain(..).collect();
		Ok(rv)
	}
}
