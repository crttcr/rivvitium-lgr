pub mod console_relay;
pub mod statistics_relay;
pub mod empty_relay_config;

use std::fmt::{Debug, Display};
use crate::Error;
use crate::model::ir::atom::Atom;

pub trait Relay
{
	fn initialize(&mut self, cfg: &dyn RelayConfig) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)                -> Option<Atom>;
	fn finish(&mut self)                            -> bool;
}

enum RelayState<S>
{
	Uninitialized,
	Broken(String),
	Ready(S),
	Completed,
}

pub trait RelayConfig: Debug + Display {
    fn string_value (&self, name: &str) -> Option<String>;
    fn integer_value(&self, name: &str) -> Option<i32>;
    fn float_value  (&self, name: &str) -> Option<f32>;
    fn bool_value   (&self, name: &str) -> Option<bool>;
}