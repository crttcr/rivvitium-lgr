
mod common;

use common::fixtures::TestAtoms;
use riv::component::sink::capture_sink::CaptureSink;
use riv::component::sink::empty_sink_config::EmptySinkConfig;
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
	let target_cfg      = EmptySinkConfig::default();
	let target_msg      = dst.initialize(&target_cfg)?;
	assert_eq!(target_msg, ());

	for atom in &mut src {
		dst.accept(atom)?;
	}

	let source_ok = src.close()?;
	assert!(source_ok);

	dst.close();
	let collected = dst.into_atoms();
	println!("{:?}", collected);

	let atoms = TestAtoms::nv_pairs();
	assert!(collected.len() == atoms.len());
	Ok(())
}
