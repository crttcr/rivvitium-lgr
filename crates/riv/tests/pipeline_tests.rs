mod common;

use common::fixtures::TestAtoms;
use riv::component::source::vector_source::VectorSource;
use riv::component::source::Source;
use riv::model::ir::atom::Atom;
use riv::Error;
use crate::common::fixtures::TestComponents;

#[test]
pub fn test_capture_of_start_and_end() -> Result<(), Error> {
	let atoms            = TestAtoms::start_end_vec();             // Create pipeline components
	let mut  src         = VectorSource::new(atoms);
	let (t_cfg, mut dst) = TestComponents::capture_config_and_sink(401);
	let target_msg       = dst.initialize(&t_cfg)?;
	assert_eq!(target_msg, ());
	for atom in &mut src {
		println!("{:?}", atom);
		dst.accept(atom)?;
	}

	let source_ok = src.close()?;
	assert!(source_ok);

	dst.close();
	let collected: Vec<Atom> = dst.drain_atoms();
	println!("{:?}", collected);
	assert_eq!(collected.len(), TestAtoms::start_end_vec().len());
	Ok(())
}
