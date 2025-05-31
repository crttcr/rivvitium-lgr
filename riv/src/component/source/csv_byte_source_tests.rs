use tempfile::NamedTempFile;
use crate::component::source::csv_byte_source::CsvByteSource;
use crate::component::source::{Source, SourceState};
use crate::Error;
use crate::utils::test_file::TestFile;

#[test]
fn new_sets_uninitialized() {
	let src = CsvByteSource::new("ignored.csv".to_string());
	assert!(matches!(src.state, SourceState::Uninitialized));
}

#[test]
fn initialize_nonexistent_file_sets_broken_io() {
	let mut src = CsvByteSource::new("no_such_file.csv".to_string());
	let err     = src.initialize(&"cfg".to_string()).unwrap_err();
	println!("Error: {:?}", err);

	// Assert that state is broken with state an Io error
	assert!(matches!(src.state, SourceState::Broken(_)));
	if let SourceState::Broken(ref e) = src.state {
		match e {
			Error::Io { source, .. } => {
				// underlying io::ErrorKind should be NotFound
				assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
			}
			_ => panic!("Expected Io variant, got {:?}", e),
		}
	}
}

#[test]
fn initialize_empty_file_sets_broken_parse() {
	let tmp     = NamedTempFile::new().unwrap();
	let path    = tmp.path().to_string_lossy().to_string();
	let mut src = CsvByteSource::new(path.clone());
	let _       = src.initialize(&"cfg".to_string()).unwrap();
	if let SourceState::Ready(state) = src.state {
		assert_eq!(state.file_path, path);
	} else {
		panic!("Expected Broken(Parse), got {:?}", src.state);
	}
}

#[test]
fn initialize_nonempty_file_sets_ready_state() {
	let tf      = TestFile::with_content("h1,h1\nv1,v2").unwrap();
	let path    = tf.path().to_string_lossy().to_string();
	let mut src = CsvByteSource::new(path);
	let res     = src.initialize(&"cfg".to_string());
	assert!(res.is_ok(), "initialize should succeed on nonempty file");
	// state should be Ready with correct ByteReaderState
	if let SourceState::Ready(ref state) = src.state {
		assert_eq!(state.total_bytes, 0);
		assert_eq!(state.chunk_count, 0);
	} else {
		panic!("Expected Ready state, found {:?}", src.state);
	}
}

#[test]
fn finish_always_returns_true() {
	let mut src = CsvByteSource::new("irrelevant.csv".to_string());
	let r = src.finish().expect("finish should not error");
	assert!(r);
}
