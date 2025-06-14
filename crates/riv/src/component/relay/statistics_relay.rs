use std::fmt::Display;
use crate::Error;
use crate::model::ir::atom::Atom;
use crate::component::relay::{Relay, RelayConfig};


//
// FIXME: Implement this class to capture statistics
//

pub struct StatisticsRelay{
	total: u64,
}

impl StatisticsRelay {
	pub fn new() -> Self {
		StatisticsRelay{ total: 0 }
	}
}

impl Relay for StatisticsRelay {
	fn initialize(&mut self, cfg: &dyn RelayConfig) -> Result<(), Error> {
		println!("--- StatisticsRelay initialized --- {}", cfg);
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Option<Atom> {
		self.total += 1;
		Some(atom) // pass the atom along unmodified
	}

	fn finish(&mut self) -> bool {
		println!("--- StatisticsRelay finished ---");
		true
	}
	
}
