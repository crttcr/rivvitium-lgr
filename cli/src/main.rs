use anyhow::Result;
use riv::error::Error;
use riv::relay::console_relay::ConsoleRelay;
use riv::relay::Relay;
use riv::sink::capture_sink::CaptureSink;
use riv::sink::Sink;

fn main() 
{
    let hello = String::from("Hello. Running [tbd]");
    println!("{}", hello);
    println!("-----------------------");
    match run() {
        Ok(_)  => {},
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<(), Error> 
{
    let mut relay = ConsoleRelay::new();
    let mut sink  = CaptureSink::new();
    let config    = "config.toml".to_owned();
    relay.initialize(&config)?;
    sink.initialize(&config)?;

    // Assume you have some Atoms
    let atoms = vec![];
    for atom in atoms {
        match relay.accept(atom) {
            None     =>   {},
            Some(a)  => sink.accept(a)?,
        }
    }

    let relay_ok = relay.finish();
    if relay_ok {
        let count = sink.finalize()?;
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