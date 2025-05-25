
use std::fmt::Display;
use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::model::ir::atom::Atom::ErrorAtom;
use crate::source::{Source, SourceState};

/// A `Source` that yields atoms from an in-memory Vec.
pub struct VectorSource {
	atoms:           Vec<Atom>,
	state:           SourceState<()>,
	error_atom_sent: bool,
}

impl VectorSource {
	/// Create a new VectorSource with the given atoms.
	pub fn new(atoms: Vec<Atom>) -> Self {
		let state           = SourceState::Uninitialized;
		let error_atom_sent = false;
		Self {atoms, state, error_atom_sent}
	}
}

impl Iterator for VectorSource {
	type Item = Atom;

	fn next(&mut self) -> Option<Self::Item> {
		match &mut self.state {
			SourceState::Uninitialized => None,
			SourceState::Completed     => None,
			SourceState::Ready(_)      => {
				let rv = self.atoms.pop();
				if rv.is_none() { self.state = SourceState::Completed }
				rv
			},
			SourceState::Broken(err)   => {
				if self.error_atom_sent { None }
				else {
					self.error_atom_sent = true;
					let x = err.clone();
					let a = ErrorAtom(x);
					Some(a)
				}
			}
		}
	}
}

impl Source for VectorSource {
	fn initialize<CFG: Display>(&mut self, cfg: &CFG) -> Result<(), Error> {
		let msg = format!("[VectorSource]: initialized with config: {}", cfg);
		println!("{msg}");
		self.state = SourceState::Ready(());
		Ok(())
	}

	// Only return Ok(true) if iteration has completed
	//
	fn finish(&mut self) -> Result<bool, Error> {
		println!("finish() called");
		match &self.state {
			SourceState::Completed => Ok(true),
			SourceState::Broken(x) => Err(x.to_owned()),
			_                      => Ok(false),
		}
	}
}
