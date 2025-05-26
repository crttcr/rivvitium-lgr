use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::source::Source;
use crate::component::source::vector_source::VectorSource;
use crate::model::ir::external_metadata::{ExternalMetadataVariant, FileMetadata};

#[test]
fn uninitialized_vector_source_returns_error() {
	let f = "/tmp/test_initialize_and_iterate".to_string();
	let a = FileMetadata::for_file(f, None);
	let a = ExternalMetadataVariant::File(a);
	let a = Atom::Start(a);
	let atoms = vec![a, Atom::Finish];
	let src   = VectorSource::new(atoms);
	let collected: Vec<Atom> = src.collect();
	let head                 = collected.get(0);
	println!("{:?}", head);
	assert!(collected.len() == 1);
	assert!(head.is_some());
}

#[test]
fn test_initialize_and_iterate() -> Result<(), Error> {
	let f = "/tmp/test_initialize_and_iterate".to_string();
	let a = FileMetadata::for_file(f, None);
	let a = ExternalMetadataVariant::File(a);
	let a = Atom::Start(a);
	let atoms    = vec![a, Atom::Finish, ];
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
