pub mod capture_sink;
pub mod console_sink;

use std::fmt::Display;
use crate::model::ir::atom::Atom;
use crate::error::Error;

pub trait Sink<R> 
{
	fn initialize<C: Display>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)              -> Result<(), Error>;
	fn finish(&mut self) -> Result<R,  Error>;
}
