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
use crate::model::ir::atom::Atom::ByteRowAtom;
use crate::model::ir::byte_row::ByteRow;

const MAX_RECORD_SIZE:       usize = 1024 * 16;
const MAX_FIELDS_PER_RECORD: usize =      1024;
const CHUNK_SIZE:            usize = 1024 * 8;


type CsvByteSourceState = SourceState<ByteReaderState>;


#[derive(Debug)]
pub struct ByteReaderState {
	pub(crate)  file_path:      String,
	pub(crate)  needs_fill:     bool,
	pub(crate)  input_offset:   usize,
	pub(crate)  total_bytes:    usize,
	pub(crate)  chunk_count:    usize,
	pub(crate)  chunk_buffer:   [u8; MAX_RECORD_SIZE],
	pub(crate)  field_indices:  [usize; MAX_FIELDS_PER_RECORD * 2],
	pub(crate)  buf_reader:     BufReader<File>,
	pub(crate)  parser:         csv_core::Reader,
}

impl ByteReaderState {
	fn new(buf_reader: BufReader<File>, file_path: String, parser: csv_core::Reader) -> Self {
		let needs_fill    = true;
		let input_offset  = 0;
		let total_bytes   = 0;
		let chunk_count   = 0;
		let chunk_buffer  = [0; MAX_RECORD_SIZE];
		let field_indices = [0; MAX_FIELDS_PER_RECORD * 2];
		ByteReaderState{file_path, needs_fill, input_offset, total_bytes, chunk_count, chunk_buffer, field_indices, buf_reader, parser}
	}

	fn fill_buffer(&mut self) -> Result<(), Error> {
		info!("\n--- Reading '{}' in {} byte chunks ---", self.file_path, CHUNK_SIZE);
		let buffer              = &mut self.chunk_buffer;
		let bytes_read_in_chunk = self.buf_reader.read(buffer).map_err(|e| Error::from(IoErrorWrapper::from(e)))?;
		if bytes_read_in_chunk == 0 {
			let msg = "Empty CSV file chunk".to_string();
			let err = Error::Parse(msg);
			Err(err)
		} else {
			info!("Read {} bytes in {} byte chunks", bytes_read_in_chunk, self.chunk_count);
			self.chunk_count += 1;
			self.total_bytes += bytes_read_in_chunk;
			Ok(())
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
		let handle_completed     = || { warn!("Next called on completed source"); None };
		let handle_broken        = || { warn!("Next called on broken source");    None };

		match self.state {
			SourceState::Broken(_)     => handle_broken(),
			SourceState::Completed     => handle_completed(),
			SourceState::Uninitialized => {
				let msg    = format!("Next called on uninitialized source");
				let err    = Error::General(msg);
				self.state = SourceState::Broken(err);
				None
			}
			SourceState::Ready(ref mut state) =>
				// if state.needs_fill { }
			
				match state.fill_buffer() {
					Ok(_) => {
						/*
						// Feed the current chunk of input bytes to the reader.
						// `read_record` returns:
						// (result, bytes_read, bytes_written, fields_found)
						let (result, bytes_read, bytes_written, fields_found) =
							state.reader.read_record(
								&state.chunk_buffer,            // The slice of input bytes to process
								&mut state.output_buffer,       // Buffer for decoded record bytes
								&mut state.field_indices,       // Buffer for field start/end indices
							);
						state.input_offset += bytes_read;       // Advance offset by # of bytes consumed in this call.
						
						match result {
							ReadRecordResult::Record => {
								// A complete record was successfully parsed!
								println!("\nRecord Found (Input Consumed: {} bytes)", bytes_read);
								println!("  Raw Record Data in Output Buffer ({} bytes): {:?}", bytes_written, &output_buffer[..bytes_written]);

								// Extract and process each field using the `field_indices`
								if fields_found > MAX_FIELDS_PER_RECORD {
									eprintln!("Warning: Record has more fields ({}) than MAX_FIELDS_PER_RECORD ({}), some fields might be truncated or ignored.",
												 fields_found, MAX_FIELDS_PER_RECORD);
								}

								for i in 0..fields_found {
									let start = state.field_indices[i * 2];
									let end   = state.field_indices[i * 2 + 1];
									if end > bytes_written {
										eprintln!("Error: Field index out of bounds for output buffer. This indicates a logic error or buffer size issue.");
										break;
									}

									let field_bytes = &output_buffer[start..end];
									// Convert bytes to string (assuming UTF-8) for printing
									println!("  Field {}: {:?}", i, str::from_utf8(field_bytes).unwrap_or("[INVALID UTF-8]"));
								}
							}
							ReadRecordResult::InputEmpty => {
								// The reader consumed all the input provided in the last `read_record` call,
								// but hasn't necessarily finished a record.
								// In a streaming scenario (reading from file/network), you would load more
								// data into `input_bytes` (or a buffer) here and update `input_offset`.
								if input_offset == input_len {
									// We've fed all available input bytes. Break the loop.
									println!("\n--- End of all input data ---");
									break;
								} else {
									// More data is available but wasn't processed in this call.
									// This typically means the `read_record` call completed its current processing
									// without consuming all of `input_bytes[input_offset..]`. The loop will continue.
								}
							}
							ReadRecordResult::OutputFull => {
								// The `output_buffer` is not large enough to hold the current record.
								// You would typically handle this by resizing the buffer and retrying,
								// or returning an error if partial records are not allowed.
								eprintln!("\nError: Output buffer is full. Need a larger `MAX_RECORD_SIZE`.");
								break;
							}
							ReadRecordResult::EndOfFile => {
								// The reader successfully reached the end of the CSV data.
								info!("\n--- Successfully reached End of File ---");
								None
							}
							ReadRecordResult::Err(e) => {
								// A parsing error occurred.
								eprintln!("\nParsing Error: {:?}", e);
								break;
							}
						}							
						 */

						None // Fixme
					}
					Err(e) => {
						warn!("{}", e);
						self.state = SourceState::Broken(e);
						None
					}
			}
		}
	}
}
