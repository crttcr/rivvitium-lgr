pub mod capture_sink;
pub mod console_sink;

use std::fmt::{Debug, Display};
use crate::model::ir::atom::Atom;
use crate::error::Error;

pub trait Sink<'a, R> 
{
	fn initialize<C: Display + Debug>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom<'a>)                  -> Result<(), Error>;
	fn finish(&mut self)                                  -> Result<R,  Error>;
}
