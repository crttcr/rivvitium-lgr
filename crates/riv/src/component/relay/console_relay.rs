use std::fmt::{Debug, Display};
use tracing::{info, instrument};
use crate::Error;
use crate::model::ir::atom::Atom;
use crate::component::relay::{Relay, RelayConfig};

#[derive(Debug)]
pub struct ConsoleRelay;

impl ConsoleRelay {
	pub fn new() -> Self {
		ConsoleRelay
	}
}

impl Relay for ConsoleRelay {
	#[instrument]
	fn initialize(&mut self, _cfg: &dyn RelayConfig) -> Result<(), Error> {
		Ok(())
	}

	// ConsoleRelay simply displays an Atom and then passes it along.
	// Other relays might filter, change or explode an atom.
	//
	fn accept(&mut self, atom: Atom) -> Option<Atom> {
		println!("{atom:?}");
		Some(atom)
	}

	#[instrument]
	fn finish(&mut self) -> bool {
		info!("[ConsoleRelay Finish -- Only here to make instrumentation work]");
		println!("--- ConsoleRelay finished ---");
		true
	}
}
