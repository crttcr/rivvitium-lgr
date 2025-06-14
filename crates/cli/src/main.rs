use anyhow::Result;
use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::empty_relay_config::EmptyRelayConfig;
use riv::component::relay::Relay;
use riv::component::sink::capture_sink::CaptureSink;
use riv::component::sink::empty_sink_config::EmptySinkConfig;
use riv::component::sink::Sink;
use riv::error::Error;
use riv::model::ir::atom::Atom;

fn main() {
	let hello = String::from("Hello. Running [tbd]");
	println!("{}", hello);
	println!("-----------------------");
	match run() {
		Ok(_) => {}
		Err(e) => println!("{}", e),
	}
}

fn run() -> Result<(), Error> {
	let mut relay = ConsoleRelay::new();
	let mut sink = CaptureSink::new();
	let relay_cfg = EmptyRelayConfig::default();
	let target_cfg = EmptySinkConfig::default();
	relay.initialize(&relay_cfg)?;
	sink.initialize(&target_cfg)?;

	// Assume you have some Atoms
	let atoms = vec![];
	for atom in atoms {
		match relay.accept(atom) {
			None => {}
			Some(a) => sink.accept(a)?,
		}
	}

	let relay_ok = relay.finish();
	if relay_ok {
		sink.close();
		let collected: Vec<Atom> = sink.into_atoms();
		let count = collected.len();
		println!("Processed {count} atoms");
		Ok(())
	} else {
		let msg = "Relay failed. Please try again.";
		let err = Error::General(msg.to_owned());
		Err(err)
	}
}

#[test]
fn run_returns_result() {
	run().unwrap();
}
