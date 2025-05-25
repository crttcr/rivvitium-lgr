use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::source::Source;
use crate::source::vector_source::VectorSource;

#[test]
fn uninitialized_vector_source_returns_none() {
	let atoms = vec![Atom::Start, Atom::Finish];
	let src   = VectorSource::new(atoms);
	let collected: Vec<Atom> = src.collect();
	assert!(collected.is_empty());
}


#[test]
fn test_initialize_and_iterate() -> Result<(), Error> {
	let atoms    = vec![ Atom::Start, Atom::Finish, ];
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
