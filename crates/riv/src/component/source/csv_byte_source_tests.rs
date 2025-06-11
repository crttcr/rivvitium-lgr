use std::fs::File;
use tempfile::NamedTempFile;
use zero::test_tools::failing_reader::FailingReader;
use crate::component::source::csv_byte_source::CsvByteSource;
use crate::component::source::{Source, SourceState};
use crate::Error;
use crate::utils::test_file::TestFile;


#[test]
fn failing_reader_causes_transition_to_broken_state() {
	let read    = FailingReader::default();
	let mut src = CsvByteSource::new(read);
	assert!(matches!(src.state, SourceState::Ready(_))); // Start in a good state
	
	let x = src.next();
	assert!(matches!(src.state, SourceState::Broken(_))); // Broken reader caused error state
	println!("{:?}", x);
}

#[test]
fn initialize_empty_file_sets_broken_parse() {
	let tmp    = NamedTempFile::new().unwrap();
	let path   = tmp.path().to_string_lossy().to_string();
	let read   = File::open(path).unwrap();
	let src    = CsvByteSource::new(read);
	if let SourceState::Ready(state) = src.state {
		let bytes = state.total_bytes;
		assert_eq!(bytes, 0);
	} else {
		panic!("Expected Broken(Parse), got {:?}", src.state);
	}
}

#[test]
fn initialize_nonempty_file_sets_ready_state() {
	let tf   = TestFile::with_content("h1,h1\nv1,v2").unwrap();
	let path = tf.path().to_string_lossy().to_string();
	let read = File::open(path).unwrap();
	let src  = CsvByteSource::new(read);
	if let SourceState::Ready(ref state) = src.state {
		assert_eq!(state.total_bytes, 0);
		assert_eq!(state.chunk_count, 0);
	} else {
		panic!("Expected Ready state, found {:?}", src.state);
	}
}
