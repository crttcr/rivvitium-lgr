#![allow(dead_code,unused_imports)]

pub mod error;
pub mod model;
pub mod component;
mod utils;

pub use crate::error::Error; // Re-export the Error type

pub type Result<T> = std::result::Result<T, Error>; // Common type alias


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
