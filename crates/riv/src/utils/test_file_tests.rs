use crate::utils::test_file::TestFile;
use std::fs;

const TEST_FILE_CREATE_ERROR:  &str = "Failed to create testfile";
const TEST_FILE_READ_ERROR:    &str = "Failed to read testfile";
const TEST_FILE_CONVERT_ERROR: &str = "Failed to convert testfile content";

#[test]
fn test_file_exists_and_contains_expected_content() {
	let content = "hello, world!";
	let tf      = TestFile::with_content(content).expect(TEST_FILE_CREATE_ERROR);
	let path    = tf.path();
	let bytes   = fs::read(path).expect(TEST_FILE_READ_ERROR);
	let hydrate = String::from_utf8(bytes).expect(TEST_FILE_CONVERT_ERROR);
	assert_eq!(hydrate, content);
}

#[test]
fn temp_file_is_deleted_on_drop() {
	// 1) Create a block so that `tf` is dropped at the end of it.
	let path = {
		let tf = TestFile::with_content("some data").expect(TEST_FILE_CREATE_ERROR);
		let p  = tf.path().to_path_buf();
		assert!(fs::metadata(&p).is_ok(), "file should exist before drop");
		p
	}; // ← here `tf` is dropped, `NamedTempFile`'s Drop impl unlinks the file

	let r = fs::metadata(&path);
	assert!(r.is_err(), "file should not exist after TestFile is dropped" );
}
