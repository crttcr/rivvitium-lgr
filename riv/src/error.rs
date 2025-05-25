mod io_error_wrapper;

use std::fmt;
use thiserror::Error;
use crate::error::io_error_wrapper::IoErrorWrapper;

#[derive(Clone, Debug, Error)]
pub enum Error 
{
	#[error("Invalid input: {0}")]
	Parse(String),

	#[error("Invalid input: {0}")]
	InvalidInput(String),

//	#[error("IO error: {0}")]
//	Io(#[from] std::io::Error),
	
	// Use the new IoErrorWrapper.
	// The #[from] here allows IoErrorWrapper to be converted into Error::Io
	// The `#[error("IO error: {source}")]` uses the Display impl of IoErrorWrapper
	#[error("IO error: {source}")]
	Io { #[from] source: IoErrorWrapper },

	#[error("NotFound: {0}")]
	NotFound(String),
	
	#[error("General error: {0}")]
	General(String),
}
