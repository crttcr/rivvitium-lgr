
pub mod csv_source;
pub mod vector_source;

#[cfg(test)]
mod vector_source_tests;

use std::fmt::Display;
use crate::model::ir::atom::Atom;
use crate::error::Error;

/// Tracks where a producer is in its lifecycle.
/// 
#[derive(Debug)]
pub enum SourceState<S> {
	Uninitialized,
	Broken(Error),
	Ready(S),
	Completed,
}


pub trait Source: Iterator<Item = Atom> {
	/// Receive the configuration and move the source from Uninitialized
	/// to either Ready(S) or Broken(Error) depending on the success of
	/// initialization
	///
	fn initialize<CFG: Display>(&mut self, cfg: &CFG) -> Result<(), Error>;

	/// Produces the next atom, or `None` if finished.
	/// Even when we have an error, we convert it into an Atom
	/// and send it along.
	/// 
//	fn next(&mut self) -> Option<Atom>;

	/// Called once after production is complete.
	///
	fn finish(&mut self) -> Result<bool, Error>;
}