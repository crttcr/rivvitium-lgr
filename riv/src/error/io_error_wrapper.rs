use std::io;
use thiserror::Error; // Make sure thiserror is in your Cargo.toml

// 1. Define a cloneable wrapper for std::io::Error information
#[derive(Clone, Debug, PartialEq)]
pub struct IoErrorWrapper
{
	kind:     io::ErrorKind,
	message:  String,
}

impl IoErrorWrapper {
	pub fn new(kind: io::ErrorKind, message: impl Into<String>) -> Self {
		let message = message.into();
		Self{kind, message}
	}

	pub fn into_io_error(self) -> io::Error {
		io::Error::new(self.kind, self.message)
	}

	pub fn kind(&self) -> io::ErrorKind {self.kind}
}

impl std::fmt::Display for IoErrorWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}: {}", self.kind, self.message)
	}
}

impl std::error::Error for IoErrorWrapper {}

impl From<io::Error> for IoErrorWrapper {
	fn from(e: io::Error) -> Self {
		let kind    = e.kind();
		let message = e.to_string();
		Self::new(kind, message)
	}
}
