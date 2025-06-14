use std::fmt::{Debug, Display};
use tracing::{info, instrument};
use crate::component::sink::{Sink, SinkConfig, SinkKind};
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

impl Sink for ConsoleSink
{
	fn kind(&self) -> SinkKind { SinkKind::Console }
	
	#[instrument(level = "debug", skip_all)]
	fn initialize(&mut self, cfg: &dyn SinkConfig) -> Result<(), Error> {
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

	#[instrument(level = "debug", skip_all)]
	fn close(&mut self) {}
}