use anyhow::Result;
use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::empty_relay_config::EmptyRelayConfig;
use riv::component::relay::Relay;
use riv::component::sink::sink_settings::SinkSettings;
use riv::error::Error;
use riv::model::ir::atom::Atom;
use zero::component::identity::id_generator::global_id_gen;

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
	let sink_cfg  = SinkSettings::capture();
	let comp_id   = global_id_gen().next_id();
	let (tx, _)   = std::sync::mpsc::channel();
	let mut sink  = sink_cfg.build_sink(comp_id, tx)?;
	let relay_cfg = EmptyRelayConfig::default();
	relay.initialize(&relay_cfg)?;
	sink.initialize(&sink_cfg)?;

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
		let collected: Vec<Atom> = sink.drain_atoms();
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
