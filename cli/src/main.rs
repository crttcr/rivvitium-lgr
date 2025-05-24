use anyhow::Result;
use riv::error::Error;
use riv::sink::console_sink::ConsoleSink;
use riv::sink::Sink;

fn main() 
{
    let hello = String::from("Hello. Running [tbd]");
    println!("{}", hello);
    println!("-----------------------");
    println!("Something goes here");
    match run() {
        Ok(_)  => {},
        Err(e) => println!("{}", e),
    }
    
}

fn run() -> Result<(), Error> 
{
    let mut sink = ConsoleSink::new();
    let config   = "config.toml".to_owned();
    sink.initialze(&config)?;

    // Assume you have some Atoms
    let atoms = vec![];
    for atom in &atoms {
        sink.accept(atom)?;
    }

    let count = sink.finalize()?;
    println!("Processed {count} atoms");
    Ok(())
}

#[test]
fn run_returns_result() {
    run().unwrap();
}