#![allow(dead_code,unused_imports)]

pub mod error;
pub mod model;
pub mod component;
pub mod utils;

use std::fs::File;
use std::path::Path;
pub use crate::error::Error; // Re-export the Error type

pub type Result<T> = std::result::Result<T, Error>; // Common type alias

pub const AUXBOX: &str = "auxbox";

pub fn data_file_path_as_str(file: &str) -> String {
	format!("../../{}/data/{}", AUXBOX, file)
}
