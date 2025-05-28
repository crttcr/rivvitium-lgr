pub mod console_relay;
pub mod statistics_relay;

use std::fmt::{Debug, Display};
use crate::Error;
use crate::model::ir::atom::Atom;

pub trait Relay<T>
{
	fn initialize<C: Display + Debug>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)                      -> Option<Atom>;
	fn finish(&mut self)                                  -> bool;
	fn result(&mut self)                                  -> &T;
}

enum RelayState<S>
{
	Uninitialized,
	Broken(String),
	Ready(S),
	Completed,
}
