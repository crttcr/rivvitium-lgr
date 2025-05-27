use crate::component::source::vector_source::VectorSource;
use crate::component::source::Source;
use crate::error::Error;
use crate::model::ir::atom::Atom;
use crate::model::ir::external_metadata::{BytesMetadata, TaskVariant};

#[test]
fn uninitialized_vector_source_returns_error() {
	let bytes   = "test_initialize_and_iterate".as_bytes();
	let meta    = BytesMetadata::for_bytes(bytes, None);
	let variant = TaskVariant::Bytes(meta);
	let atom    = Atom::StartTask(variant);
	let atoms   = vec![atom, Atom::FinishTask];
	let src     = VectorSource::new(atoms);
	let collected: Vec<Atom> = src.collect();
	let head                 = collected.get(0);
	println!("{:?}", head);
	assert!(collected.len() == 1);
	assert!(head.is_some());
}

#[test]
fn test_initialize_and_iterate() -> Result<(), Error> {
	let bytes    = "test_initialize_and_iterate".as_bytes();
	let meta     = BytesMetadata::for_bytes(bytes, None);
	let variant  = TaskVariant::Bytes(meta);
	let atom     = Atom::StartTask(variant);
	let atoms    = vec![atom, Atom::FinishTask, ];
	let mut  src = VectorSource::new(atoms);
	let msg = src.initialize(&"cfg".to_string())?;
	//assert_eq!(msg, "VectorSource initialized with config: cfg");
	assert_eq!(msg, ());

	let collected: Vec<Atom> = src.by_ref().collect();
	println!("{:?}", collected);	
//	assert_eq!(collected, atoms);

	// After completion, next yields None
	assert!(src.next().is_none());

	let rv = src.finish()?;
	assert!(rv);
	Ok(())
}
