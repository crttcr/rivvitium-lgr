// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::sink::Sink;

use std::fmt::Display;

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
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error> {
		let msg = format!("[CaptureSink  ]: Initializing {}. TODO: Actually use configuration", cfg);
		println!("{msg}");
		self.atoms.clear();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		self.atoms.push(atom);
		Ok(())
	}

	fn finish(&mut self) -> Result<Vec<Atom>, Error> {
		let rv: Vec<Atom> = self.atoms.drain(..).collect();
		Ok(rv)
	}
}
