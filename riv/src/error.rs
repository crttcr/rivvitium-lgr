
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error 
{
	#[error("Invalid input: {0}")]
	Parse(String),

	#[error("Invalid input: {0}")]
	InvalidInput(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("NotFound: {0}")]
	NotFound(String),
	
	#[error("General error: {0}")]
	General(String),
}
