pub mod console_relay;

use std::fmt::Display;
use crate::Error;
use crate::model::ir::atom::Atom;

pub trait Relay
{
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)              -> Option<Atom>;
	fn finish(&mut self)                          -> bool;
}

enum RelayState<S>
{
	Uninitialized,
	Broken(String),
	Ready(S),
	Completed,
}
