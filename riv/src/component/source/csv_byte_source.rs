use crate::component::source::csv_adapter::CsvState;
use crate::component::source::{Source, SourceState};
use crate::model::ir::atom::Atom;
use crate::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use csv::ReaderBuilder;
use tracing::{info, instrument, warn};
use crate::error::IoErrorWrapper;
use crate::model::ir::atom::Atom::ByteRowAtom;
use crate::model::ir::byte_row::ByteRow;

type CsvSourceState = SourceState<ByteReaderState>;

#[derive(Debug)]
pub struct ByteReaderState {
	pub(crate) total_bytes: usize,
	pub(crate) chunk_count: usize,
	pub(crate) chunk_buffer: Vec<u8>,
}

impl ByteReaderState {
	fn new(total_bytes: usize, chunk_count: usize, chunk_buffer: Vec<u8>) -> Self {
		ByteReaderState{total_bytes, chunk_count, chunk_buffer}
	}
}

/// This class uses the lower level csv_core crate to parse a CSV file.
/// This is done so we have control over the underlying byte buffer.
/// 
/// NOTE: reading is split between initialize() and the iterator. When initialize is
/// called, the file is opened and the first chunk is read, but no processing is done
/// until the Iterator::next() is called.
/// 
#[derive(Debug)]
pub struct CsvByteSource {
	file_path:     String,
	pub(crate) state:         CsvSourceState,
//	chunk_buffer:  Vec<u8>,
}

impl CsvByteSource {
	pub fn new(file_path: String) -> Self {
		let state        = SourceState::Uninitialized;
		CsvByteSource {file_path, state}  // Should ChunkBuffer go into the state?
	}

	// This is a bit goofy, but we need to return an error and
	// capture an error as part of the state, so we remember it
	// next time self.state is interrogated. So a copy is made
	//
	fn handle_io_error(&mut self, x: io::Error) -> Error {
		let msg = format!("Error reading CSV file: {}", x.to_string());
		warn!(msg);
		let copy   =  io::Error::new(x.kind(), msg);
		let source = IoErrorWrapper::from(copy);
		let other  = Error::from(source);
		self.state = SourceState::Broken(other);
		let source = IoErrorWrapper::from(x);
		Error::from(source)
	}
	
	fn capture_parse_error(&mut self, x: csv::Error) -> () {
		let msg    = format!("Error parsing CSV file: {}", x);
		warn!(msg);
		let err    = Error::Parse(msg);
		self.state = SourceState::Broken(err)
	}
}

impl Source for CsvByteSource {
	#[instrument]
	fn initialize<CFG: Display + Debug>(&mut self, _cfg: &CFG) -> Result<(), Error> {
		match File::open(self.file_path.clone()) {
			Err(e)   => {
				warn!("{}", e);
				let source = IoErrorWrapper::from(e);
				let err    = Error::from(source);
				self.state = SourceState::Broken(err.clone());
				Err(err)
			}
			Ok(file) => {
				let chunk_size       = 1024 * 8;
				let mut reader       = BufReader::new(file);
				let mut chunk_buffer = vec![0; chunk_size];
				let mut total_bytes  = 0;
				let mut chunk_count  = 0;
				info!("\n--- Reading '{}' in {} byte chunks ---", self.file_path, chunk_size);
				// `read` attempts to fill the buffer, returning the number of bytes read.
				// It does not clear the buffer; it overwrites the beginning.
				let bytes_read_in_chunk = reader.read(&mut chunk_buffer).map_err(|e| self.handle_io_error(e))?;
				if bytes_read_in_chunk == 0 {
					let msg    = "Empty CSV file chunk".to_string();
					let err    = Error::Parse(msg);
					self.state = SourceState::Broken(err.clone());
					Err(err)
				} else {
					info!("Read {} bytes in {} byte chunks", bytes_read_in_chunk, chunk_count);
					chunk_count += 1;
					total_bytes += bytes_read_in_chunk;
					let state = ByteReaderState::new(total_bytes, chunk_count, chunk_buffer);
					self.state   = SourceState::Ready(state); // FIXME: State
					Ok(())
				}
			}
		}
	}

	#[instrument]
	fn finish(&mut self) -> Result<bool, Error> {
		Ok(true)
	}
}

impl Iterator for CsvByteSource {
	type Item = Atom;
	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}
