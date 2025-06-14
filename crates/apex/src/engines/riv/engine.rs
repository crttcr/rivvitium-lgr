

use riv::component::relay::Relay;
use riv::component::sink::Sink;
use riv::component::source::Source;

pub struct Engine {
	pub source:   Box<dyn Source>,
	pub relays:   Vec<Box<dyn Relay>>,
	pub sink:     Option<Box<dyn Sink>>,
}

impl Engine {
	/// Does this engine have relays?
	#[inline]
	pub fn has_relays(&self) -> bool { !self.relays.is_empty() }

	/// Does this engine have a sink?
	#[inline]
	pub fn has_sink(&self) -> bool { self.sink.is_some() }
}
