use std::fmt::{self, Debug, Display};
use crate::component::relay::RelayConfig;

/// A configuration value that contains **no keys at all**.
/// Every lookup returns `None`.
#[derive(Clone, Copy, Default)]
pub struct EmptyRelayConfig;

impl Debug for EmptyRelayConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EmptyConfig")
    }
}

impl Display for EmptyRelayConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<empty-config>")
    }
}

impl RelayConfig for EmptyRelayConfig {
    fn string_value (&self, _name: &str) -> Option<String> { None }
    fn integer_value(&self, _name: &str) -> Option<i32>    { None }
    fn float_value  (&self, _name: &str) -> Option<f32>    { None }
    fn bool_value   (&self, _name: &str) -> Option<bool>   { None }
}