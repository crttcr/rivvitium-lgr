// capture_sink.rs

use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::sink::Sink;

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

impl Sink<u64> for CaptureSink {
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error> {
		println!("--- CaptureSink initialized ---");
		println!("{cfg}");
		self.atoms.clear();
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		self.atoms.push(atom);
		Ok(())
	}

	fn finalize(&mut self) -> Result<u64, Error> {
		Ok(self.atoms.len() as u64)
	}
}