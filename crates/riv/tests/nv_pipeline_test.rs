
mod common;

use common::fixtures::TestAtoms;
use riv::component::sink::{SinkKind};
use riv::component::source::vector_source::VectorSource;
use riv::component::source::Source;
use riv::Error;
use crate::common::fixtures::TestComponents;

#[test]
pub fn nv_pipeline_test() -> Result<(), Error> {
	let atoms            = TestAtoms::nv_pairs();
	let mut  src         = VectorSource::new(atoms);
	let (t_cfg, mut dst) = TestComponents::capture_config_and_sink(401);
	let target_msg       = dst.initialize(&t_cfg)?;
	assert_eq!(target_msg, ());

	for atom in &mut src {
		dst.accept(atom)?;
	}

	let source_ok = src.close()?;
	assert!(source_ok);

	dst.close();
	match dst.kind() {  
		SinkKind::Capture => {
		},
		_ => panic!("sink is not CaptureSink"),
	}
	let collected = dst.drain_atoms();
	assert!(collected.len() == TestAtoms::nv_pairs().len());
	Ok(())
}
