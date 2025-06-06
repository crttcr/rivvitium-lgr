#![allow(dead_code,unused_imports)]

pub mod error;
pub mod model;
pub mod component;
pub mod utils;

pub use crate::error::Error; // Re-export the Error type

pub type Result<T> = std::result::Result<T, Error>; // Common type alias

pub const AUXBOX: &str = "../../auxbox";
