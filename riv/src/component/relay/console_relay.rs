use std::fmt::Display;
use crate::Error;
use crate::model::ir::atom::Atom;
use crate::component::relay::Relay;

pub struct ConsoleRelay;

impl ConsoleRelay {
	pub fn new() -> Self {
		ConsoleRelay
	}
}

impl Relay<()> for ConsoleRelay {
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error> {
		let msg = format!("[ConsoleRelay ]: Initializing {}. TODO: Actually use configuration", cfg);
		println!("{msg}");
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Option<Atom> {
		println!("{atom:?}");
		Some(atom) // pass the atom along unmodified
	}

	fn finish(&mut self) -> bool {
		println!("--- ConsoleRelay finished ---");
		true
	}

	fn result(&mut self) -> &() { &() }
}
