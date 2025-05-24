use crate::model::ir::atom::Atom;

pub trait Relay
{
	fn init(&mut self)                  -> bool;
	fn accept(&mut self, segment: Atom) -> Atom;
	fn finish(&mut self)                -> bool;
}


enum RelayState<S>
{
	Uninitialized,
	Broken(String),
	Ready(S),
	Completed,
}
