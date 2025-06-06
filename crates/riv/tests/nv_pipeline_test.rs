
mod common;

use common::fixtures::TestAtoms;
use riv::component::sink::capture_sink::CaptureSink;
use riv::component::sink::Sink;
use riv::component::source::vector_source::VectorSource;
use riv::component::source::Source;
use riv::model::ir::atom::Atom;
use riv::Error;


#[test]
pub fn nv_pipeline_test() -> Result<(), Error> {

	// Create pipeline components
	let atoms    = TestAtoms::nv_pairs();
	let mut  src = VectorSource::new(atoms);
	let mut  dst = CaptureSink::new();

	// Initialize pipeline components
	let cfg             = "cfg".to_owned();
	let target_msg      = dst.initialize(&cfg)?;
	assert_eq!(target_msg, ());

	for atom in &mut src {
		dst.accept(atom)?;
	}

	let source_ok = src.finish()?;
	assert!(source_ok);

	let collected: Vec<Atom> = dst.finish()?;
	println!("{:?}", collected);

	let atoms = TestAtoms::nv_pairs();
	assert!(collected.len() == atoms.len());
	Ok(())
}
