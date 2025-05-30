use crate::component::source::csv_adapter::CsvState;
use crate::component::source::{Source, SourceState};
use crate::model::ir::atom::Atom;
use crate::Error;
use csv_core;
use csv_core::ReadRecordResult;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use csv::ReaderBuilder;
use tracing::{info, instrument, warn};
use crate::error::IoErrorWrapper;
use crate::model::ir::atom::Atom::{ByteRowAtom, ErrorAtom};
use crate::model::ir::byte_row::ByteRow;

const MAX_RECORD_SIZE:       usize = 1024 * 16;
const MAX_FIELDS_PER_RECORD: usize =      1024;
const CHUNK_SIZE:            usize = 1024 * 8;


type CsvByteSourceState = SourceState<ByteReaderState>;


#[derive(Debug)]
pub struct ByteReaderState {
	pub(crate)  file_path:      String,
	pub(crate)  start:          usize,
	pub(crate)  end:            usize,
	pub(crate)  input_offset:   usize,
	pub(crate)  total_bytes:    usize,
	pub(crate)  chunk_count:    usize,
	pub(crate)  chunk_buffer:   [u8; CHUNK_SIZE],
	pub(crate)  output_record:  [u8; MAX_RECORD_SIZE],
	pub(crate)  field_indices:  [usize; MAX_FIELDS_PER_RECORD * 2],
	pub(crate)  buf_reader:     BufReader<File>,
	pub(crate)  parser:         csv_core::Reader,
}

impl ByteReaderState {
	fn new(buf_reader: BufReader<File>, file_path: String, parser: csv_core::Reader) -> Self {
		let start         = 0;
		let end           = 0;
		let input_offset  = 0;
		let total_bytes   = 0;
		let chunk_count   = 0;
		let chunk_buffer  = [0; CHUNK_SIZE];
		let output_record = [0; MAX_RECORD_SIZE];
		let field_indices = [0; MAX_FIELDS_PER_RECORD * 2];
		ByteReaderState{file_path, start, end, input_offset, total_bytes, chunk_count, chunk_buffer, output_record, field_indices, buf_reader, parser}
	}

	// Have we parsed everything that has been read from the latest file read?
	//
	fn needs_fill(&self) -> bool {self.start == self.end}

	// Read the next chunk from the file
	//
	fn fill_buffer(&mut self) -> Result<bool, io::Error> {
		info!("\n--- Reading '{}' in {} byte chunks ---", self.file_path, CHUNK_SIZE);
		let buffer = &mut self.chunk_buffer;
		let n      = self.buf_reader.read(buffer)?;
		if n == 0 {
			let msg = "Empty CSV file chunk. We are done.".to_string();
			info!("{}", msg);
			Ok(false)
		} else {
			self.chunk_count += 1;
			self.total_bytes += n;
			self.start        = 0;
			self.end	         = n;
			info!("Read {} bytes for chunk {}", n, self.chunk_count);
			Ok(true)
		}
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
	file_path:          String,
	pub(crate) state:   CsvByteSourceState,
}

impl CsvByteSource {
	pub fn new(file_path: String) -> Self {
		let state        = SourceState::Uninitialized;
		CsvByteSource{file_path, state}
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
			Err(e) => {
				warn!("{}", e);
				let source = IoErrorWrapper::from(e);
				let err    = Error::from(source);
				self.state = SourceState::Broken(err.clone());
				Err(err)
			}
			Ok(file) => {
				info!("\n--- File open success: '{}'", self.file_path);
				let parser    = csv_core::ReaderBuilder::new().delimiter(b';').build();
				let reader    = BufReader::new(file);
				let file_path = self.file_path.clone();
				let state     = ByteReaderState::new(reader, file_path, parser);
				self.state    = SourceState::Ready(state);
				Ok(())
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
		let handle_completed = || { warn!("Next called on completed source"); None };
		let handle_broken    = || { warn!("Next called on broken source");    None };

		match self.state {
			SourceState::Broken(_)     => handle_broken(),
			SourceState::Completed     => handle_completed(),
			SourceState::Uninitialized => {
				let msg    = format!("Next called on uninitialized source");
				let err    = Error::General(msg);
				self.state = SourceState::Broken(err);
				None
			}
			SourceState::Ready(ref mut state) => {
				if state.needs_fill() {
					match state.fill_buffer() {
						Err(e) => {
							warn!("{}", e);
							let source = IoErrorWrapper::from(e);
							let err    = Error::from(source);
							self.state = SourceState::Broken(err);
							return None
						},
						Ok(false) => {
							self.state = SourceState::Completed;
							return None
							},
						Ok(true)  => {}  // Fall through and parse the next record ...
					}
				}
				
				let input = &state.chunk_buffer[state.start..state.end];
				let (result, bytes_read, bytes_written, field_count) = state.parser.read_record(input, &mut state.output_record, &mut state.field_indices);
				state.start += bytes_read;                                         // Slide forward in the chunk buffer
				match result {
					ReadRecordResult::InputEmpty       => {                         // Need more input: loop back to refill if possible
						self.state = SourceState::Completed;
						let error  = Error::Parse("Input empty despite latest fill".to_string());
						let atom   = Atom::ErrorAtom(error);
						Some(atom)
					}              
					ReadRecordResult::End              => {                         // No more records (trapped EOF in the middle of parser)
						self.state = SourceState::Completed;
						let error  = Error::Parse("End of data".to_string());
						let atom   = Atom::ErrorAtom(error);
						Some(atom)
					}                          
					ReadRecordResult::OutputFull       => { 
						let msg    = format!("Record too large for output buffer ({} bytes)", MAX_RECORD_SIZE);
						let err    = Error::General(msg);
						self.state = SourceState::Broken(err);
						None
					}
					ReadRecordResult::OutputEndsFull   => { 
						let msg    = format!("Too many fields. Limit ({})", MAX_FIELDS_PER_RECORD);
						let err    = Error::General(msg);
						self.state = SourceState::Broken(err);
						None
					}
					ReadRecordResult::Record   => { 
						let data   = &state.output_record[..bytes_written];
						let bounds = &state.field_indices[..field_count  ];
						let row    = ByteRow::new(data, bounds);
						let atom   = Atom::ByteRowAtom(row);
						Some(atom)
						}
				}
			}
		}
	}
}
