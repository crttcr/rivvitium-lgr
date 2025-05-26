use crate::component::source::vector_source::VectorSource;
use crate::model::ir::atom::Atom;
use crate::model::ir::external_metadata::{TaskVariant, FileMetadata};
use crate::utils::test_file::TestFile;

#[test]
fn file_metadata_has_a_valid_ulid() {
	let tf = TestFile::with_content("hello, world!").expect("Could not create test file.");
	let p = tf.path_string();
	let a = FileMetadata::for_file(p, None).unwrap();
	let s = a.ulid.to_string();
	println!("{}", s);
	println!("{a:?}");
	println!("{:?}", a.ulid.to_string());
	}
