
mod common;

use riv::Error;
use riv::model::ir::atom::Atom;
use riv::component::sink::capture_sink::CaptureSink;
use riv::component::sink::Sink;
use riv::component::source::Source;
use riv::component::source::vector_source::VectorSource;
use common::fixtures::TestAtoms;



#[test]
pub fn test_capture_of_start_and_end() -> Result<(), Error> {
	
	// Create pipeline components
	let atoms    = TestAtoms::start_end_vec();
	let mut  src = VectorSource::new(atoms);
	let mut  dst = CaptureSink::new();
	
	// Initialize pipeline components
	let cfg             = "cfg".to_owned();
	let source_msg      = src.initialize(&cfg)?;
	let target_msg      = dst.initialize(&cfg)?;
	assert_eq!(source_msg, ());
	assert_eq!(target_msg, ());

	for atom in &mut src {
		dst.accept(atom)?;
	}

	let source_ok = src.finish()?;
	assert!(source_ok);
	
	let collected: Vec<Atom> = dst.finalize()?;
	println!("{:?}", collected);
	
	let atoms = TestAtoms::start_end_vec();
	assert!(collected.len() == atoms.len());
	
	
	// TODO: Assert something happens
	
	Ok(())
}
