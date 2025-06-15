
use std::collections::VecDeque;
use std::fmt::Display;
use tracing::info;
use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::model::ir::atom::Atom::ErrorAtom;
use crate::component::source::{Source, SourceConfig, SourceState};

/// A `Source` that yields atoms from an in-memory Vec.
pub struct VectorSource {
	atoms:           VecDeque<Atom>,
	state:           SourceState<()>,
	error_atom_sent: bool,
}

impl VectorSource {
	/// Create a new VectorSource with the given atoms.
	pub fn new(atoms: Vec<Atom>) -> Self {
		let msg             = format!("[VectorSource]: created. Item count: {}", atoms.len());
		let state           = SourceState::Ready(());
		let error_atom_sent = false;
		info!("{msg}");
		Self {atoms: atoms.into(), state, error_atom_sent}
	}
}

impl Iterator for VectorSource {
	type Item = Atom;

	fn next(&mut self) -> Option<Self::Item> {
		match &mut self.state {
			SourceState::Completed     => None,
			SourceState::Ready(_)      => {
				let rv = self.atoms.pop_front();
				if rv.is_none() {
					self.state = SourceState::Completed
				}
				rv
			},
			SourceState::Broken(err)   => {
				if self.error_atom_sent { None }
				else {
					self.error_atom_sent = true;
					let x                = err.clone();
					let a                = ErrorAtom(x);
					Some(a)
				}
			}
		}
	}
}

impl Source for VectorSource {

	// fn from_config(_cfg: &dyn SourceConfig) -> Result<Box<Self>, Error> {
	// 	Err(Error::InvalidConfig("[VectorSource]: do not use config to create this source. Call VectorSource::new(atoms) directly".to_owned()))
   //  }
	// 
	
	// Only return Ok(true) if iteration has completed
	//
	fn close(&mut self) -> Result<bool, Error> {
		println!("finish() called");
		match &self.state {
			SourceState::Completed => Ok(true),
			SourceState::Broken(x) => Err(x.to_owned()),
			_                      => Ok(false),
		}
	}
}
