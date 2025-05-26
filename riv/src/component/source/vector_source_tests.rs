use crate::model::ir::atom::Atom;
use crate::error::Error;
use crate::component::source::Source;
use crate::component::source::vector_source::VectorSource;
use crate::model::ir::external_metadata::{TaskVariant, FileMetadata, BytesMetadata};

#[test]
fn uninitialized_vector_source_returns_error() {

	let b = "test_initialize_and_iterate".as_bytes();
	let m = BytesMetadata::for_bytes(b, None);
	let v = TaskVariant::Bytes(m);
	let a = Atom::StartTask(v);
	let atoms = vec![a, Atom::Finish];
	let src   = VectorSource::new(atoms);
	let collected: Vec<Atom> = src.collect();
	let head                 = collected.get(0);
	println!("{:?}", head);
	assert!(collected.len() == 1);
	assert!(head.is_some());
}

/*
use std::fs;
use crate::tests::utils::temp_file::TestFile; // adjust the path

#[test]
fn my_reader_reads_back_file_contents() {
    // 1) spin up a temp file with known content
    let tf = TestFile::with_content("hello, world!").expect("couldn't make temp file");

    // 2) hand the path to whatever you’re testing
    let got = my_crate::read_file(tf.path()).expect("reader failed");

    // 3) assert as usual
    assert_eq!(got, "hello, world!");

    // when `tf` goes out of scope, its backing file is deleted
}
 */

#[test]
fn test_initialize_and_iterate() -> Result<(), Error> {
	let b = "test_initialize_and_iterate".as_bytes();
	let m = BytesMetadata::for_bytes(b, None);
	let v = TaskVariant::Bytes(m);
	let a = Atom::StartTask(v);
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
