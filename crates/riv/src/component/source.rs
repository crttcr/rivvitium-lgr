
pub mod csv_adapter;
pub mod csv_byte_source;
pub mod csv_string_source;
pub mod path_buf_config;
pub mod vector_source;

#[cfg(test)]
mod vector_source_tests;
#[cfg(test)]
mod csv_byte_source_tests;

use std::fmt::{Debug, Display};
use std::path::PathBuf;
use crate::model::ir::atom::Atom;
use crate::error::Error;

/// Defines the type of source 
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
	Csv,
	Json,
	StaticData,
}

/// Tracks where a producer is in its lifecycle.
///
#[derive(Debug)]
pub enum SourceState<S> {
	Ready(S),
	Broken(Error),
	Completed,
}



pub trait Source: Iterator<Item = Atom> {
	fn source_type(&self) -> SourceType;
	
	/// Called once after production is complete.
	///
	fn close(&mut self) -> Result<bool, Error>;
}


pub trait SourceConfig: Debug + Display {
	fn source_type(&self) -> SourceType {SourceType::Csv}
	
    fn path_buf     (&self)             -> Option<&PathBuf>;
    fn string_value (&self, name: &str) -> Option<String>;
    fn integer_value(&self, name: &str) -> Option<i32>;
    fn float_value  (&self, name: &str) -> Option<f32>;
    fn bool_value   (&self, name: &str) -> Option<bool>;
}
