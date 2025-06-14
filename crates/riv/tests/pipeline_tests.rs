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
pub fn test_capture_of_start_and_end() -> Result<(), Error> {
	let atoms    = TestAtoms::start_end_vec();             // Create pipeline components
	let mut  src = VectorSource::new(atoms);
	let mut  dst = CaptureSink::new();

	let target_cfg      = EmptySinkConfig::default();
	let target_msg      = dst.initialize(&target_cfg)?;
	assert_eq!(target_msg, ());
	for atom in &mut src {
		dst.accept(atom)?;
	}

	let source_ok = src.close()?;
	assert!(source_ok);

	dst.close();
	let collected: Vec<Atom> = dst.into_atoms();
	println!("{:?}", collected);
	assert_eq!(collected.len(), TestAtoms::start_end_vec().len());
	Ok(())
}
