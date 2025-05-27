use std::fmt::Display;
use crate::component::sink::Sink;
use crate::error::Error;
use crate::model::ir::atom::Atom;

pub struct ConsoleSink {
	count: u64,
}

impl ConsoleSink {
	pub fn new() -> Self {
		Self { count: 0 }
	}
}

impl Sink<u64> for ConsoleSink
{
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error> {
		let msg = format!("[ConsoleSink ]: Initializing {}. TODO: Actually use configuration", cfg);
		println!("{msg}");
		self.count = 0;
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		println!("{atom:?}");
		self.count += 1;
		Ok(())
	}

	fn finish(&mut self) -> Result<u64, Error> {
		println!("--- ConsoleSink finalized ---");
		Ok(self.count)
	}
}