
pub mod csv_adapter;
pub mod csv_byte_source;
pub mod csv_string_source;
pub mod vector_source;

#[cfg(test)]
mod vector_source_tests;
#[cfg(test)]
mod csv_byte_source_tests;

use std::fmt::{Debug, Display};
use crate::model::ir::atom::Atom;
use crate::error::Error;

/// Tracks where a producer is in its lifecycle.
///
#[derive(Debug)]
pub enum SourceState<S> {
	Ready(S),
	Broken(Error),
	Completed,
}


pub trait Source: Iterator<Item = Atom> {
	/// Called once after production is complete.
	///
	fn finish(&mut self) -> Result<bool, Error>;
}
