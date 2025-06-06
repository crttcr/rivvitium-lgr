use crate::model::ir::external_metadata::FileMetadata;
use crate::utils::test_file::TestFile;

#[test]
fn file_metadata_has_a_valid_ulid() {
	let f = TestFile::with_content("hello, world!").expect("Could not create test file.");
	let p = f.path_string();
	let a = FileMetadata::for_file(p, None).unwrap();
	let t = a.ulid.timestamp_ms();
	assert!(t > 1748360722700)
}
