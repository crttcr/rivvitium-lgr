use std::fmt::Display;
use std::fs::File;
use csv::ByteRecordsIntoIter;
use crate::component::source::{Source, SourceState};
use crate::Error;
use crate::model::ir::atom::Atom;

type CsvSourceState = SourceState<csv::ByteRecordsIntoIter<File>>;

pub struct CsvSource {
	file:  File,
	state: CsvSourceState,
}

impl Source for CsvSource {
	fn initialize<CFG: Display>(&mut self, cfg: &CFG) -> Result<(), Error> {
		print!("TODO: Initializing CSV source {}", cfg);
		todo!()
	}

	fn finish(&mut self) -> Result<bool, Error> {
		Ok(true)
	}
}

impl CsvSource {
	pub fn new(file: File) -> Self {
		let state = SourceState::Uninitialized;
		CsvSource{file, state}
	}
}

impl Iterator for CsvSource {
	type Item = Atom;
	fn next(&mut self) -> Option<Self::Item> {
		let read_error = |x: csv::Error| -> CsvSourceState {
			let msg    = format!("Error reading CSV file: {}", x);
			let err    = Error::General(msg.into());
			SourceState::Broken(err)
		};
		
		// TODO: Setup the CSV reader to capture headers and prep an iterator
		//
		let handle_uninitialized = || -> Option<Self::Item> {
			// let file  = File::open(&self.file.).expect("Unable to open file");
			None
		};

		// TODO: Call CSV iterator and convert the result into an Atom and emit
		// 
		let handle_ready = |_s: &mut ByteRecordsIntoIter<File>| -> Option<Self::Item> {
			eprintln!("TODO: Handle ready event");
			None
		};

		// TODO: Instrument so that we're capturing next on a broken source
		let handle_broken = |x| -> Option<Self::Item> {
			eprintln!("CsvSource: Next called on broken producer: {}", x);
			None
		};

		// TODO: Instrument this so that we're capturing next on a completed source
		let handle_completed = || -> Option<Self::Item> {
			eprintln!("CsvSource: Next called on completed producer");
			None
		};
		
		match &mut self.state {
			SourceState::Uninitialized => handle_uninitialized(),
			SourceState::Broken(err)   => handle_broken(err),
			SourceState::Completed     => handle_completed(),
			SourceState::Ready(s)      => handle_ready(s),
		}

		/*		
				match &mut self.state {
					SourceState::Uninitialized => {
						let file       = File::open(&self.file_path).expect("Unable to open file");
						let mut rdr    = Reader::from_reader(file);
						match rdr.byte_headers() {
							Err(x)      => {self.state = read_error(x); None }
							Ok(headers) => {
								let coordinate = compute_coordinate(&headers);
								let atom       = compute_headers(&headers, coordinate);
								let iter       = rdr.into_byte_records();
								self.state     = SourceState::Ready(iter);
								Some(atom)
							},
						}
					}
					SourceState::Ready(it) => {
						match  it.next() {
							None         => { self.state = SourceState::Completed; None }
							Some(result) => {
								match result {
									Err(x) => {self.state = read_error(x); None }
									Ok(r) => {
										let coordinate  = compute_coordinate(&r);
										let atom        = compute_raw_values(&r, coordinate);
										Some(atom)
									},
								}
							},
						}
					},
				}
		*/
	}
}
