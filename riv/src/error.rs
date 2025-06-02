pub mod io_error_wrapper;

use std::fmt;
use thiserror::Error;
pub use crate::error::io_error_wrapper::IoErrorWrapper;

#[derive(Clone, Debug, Error)]
pub enum Error 
{
	#[error("Invalid input: {0}")]
	Parse(String),

	#[error("Invalid input: {0}")]
	InvalidInput(String),

	// Use the new IoErrorWrapper.
	// The #[from] here allows IoErrorWrapper to be converted into Error::Io
	// The `#[error("IO error: {source}")]` uses the Display impl of IoErrorWrapper
	//
	#[error("IO error: {source}")]
	Io { #[from] source: IoErrorWrapper },

	#[error("NotFound: {0}")]
	NotFound(String),
	
	#[error("General error: {0}")]
	General(String),
}

/// Convert a `csv::Error` into our `Error` enum.
/// 
use std::{error::Error as StdError, io};
impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        // Try to see if the csv::Error was caused by an underlying io::Error
        if let Some(source) = err.source() {
            // If the source is an io::Error, wrap it
            if let Some(io_err) = source.downcast_ref::<io::Error>() {
                // Create a fresh io::Error with the same kind and message
                let cloned = io::Error::new(io_err.kind(), io_err.to_string());
                let wrapper = IoErrorWrapper::from(cloned);
                return Error::Io { source: wrapper };
            }
        }
        // Otherwise, treat it as a parse error
        Error::Parse(err.to_string())
    }
}
