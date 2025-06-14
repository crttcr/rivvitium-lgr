use std::fmt::{self, Debug, Display};
use std::path::PathBuf;
use crate::component::sink::SinkConfig;

/// A configuration value that contains **no keys at all**.
/// Every lookup returns `None`.
#[derive(Clone, Copy, Default)]
pub struct EmptySinkConfig;

impl Debug for EmptySinkConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EmptyConfig")
    }
}

impl Display for EmptySinkConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<empty-config>")
    }
}

impl SinkConfig for EmptySinkConfig {
    fn path_buf(&self)                   -> Option<PathBuf> { None } 
    fn string_value (&self, _name: &str) -> Option<String>  { None }
    fn integer_value(&self, _name: &str) -> Option<i32>     { None }
    fn float_value  (&self, _name: &str) -> Option<f32>     { None }
    fn bool_value   (&self, _name: &str) -> Option<bool>    { None }
}
