pub mod capture_sink;
pub mod console_sink;
pub mod csv_sink;

use std::fmt::{Debug, Display};
use crate::model::ir::atom::Atom;
use crate::error::Error;

pub trait Sink<R> 
{
	fn initialize<C: Display + Debug>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)                      -> Result<(), Error>;
	fn finish(&mut self)                                  -> Result<R,  Error>;
}
