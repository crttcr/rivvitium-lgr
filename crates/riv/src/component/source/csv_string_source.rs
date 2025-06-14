use crate::component::source::csv_adapter::CsvState;
use crate::component::source::{Source, SourceConfig, SourceState};
use crate::model::ir::atom::Atom;
use crate::Error;
use std::fmt::{Debug, Display};
use tracing::instrument;
use crate::error::IoErrorWrapper;
use crate::model::ir::atom::Atom::StringRowAtom;
use crate::model::ir::string_row::StringRow;

type CsvSourceState = SourceState<CsvState>;

#[derive(Debug)]
pub struct CsvStringSource {
	file_path:  String,
	state:      CsvSourceState,
}

impl CsvStringSource {
	pub fn new(file_path: String) -> Self {
		match CsvState::new(&file_path) {
			Ok(state) => {
				let state = SourceState::Ready(state);
				CsvStringSource {file_path, state}
			}
			Err(err) => {
				let error = Error::from(err);
				let state = SourceState::Broken(error);
				CsvStringSource {file_path, state}
			}	
		}
	}

	fn handle_read_error(x: csv::Error) -> CsvSourceState {
		let msg = format!("Error reading CSV file: {}", x);
		let err = Error::Parse(msg);
		SourceState::Broken(err)
	}
}

impl Source for CsvStringSource {
	// fn from_config(cfg: &dyn SourceConfig) -> Result<Box<Self>, Error> {
	// 	if let(Some(file_path)) = cfg.path_buf() {
	// 		let file_path_string = file_path.clone().into_os_string().into_string();
	// 		match file_path_string {
	// 			Ok(path) => {
	// 				let source    = CsvStringSource::new(path);
	// 				Ok(Box::new(source))
	// 			},
	// 			Err(x) => {
	// 				let msg = format!("Bad path: {:?}", x);
	// 				let err = Error::InvalidConfig(msg);
	// 				Err(err)
	// 			},
	// 		}
	// 	} else {
	// 	 	let msg = "CsvStringSource can only be called on file paths".to_string();
	// 	 Err(Error::InvalidConfig(msg))
	// 	}
	// }
	
	#[instrument]
	fn close(&mut self) -> Result<bool, Error> {
		Ok(true)
	}
}



impl Iterator for CsvStringSource {
	type Item = Atom;
	fn next(&mut self) -> Option<Self::Item> {

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
			SourceState::Broken(err) => handle_broken(err),
			SourceState::Completed   => handle_completed(),
			SourceState::Ready(s)    => {
				if s.header_atom.is_some() {
					let headers = s.header_atom.take().unwrap();
					Some(headers)
				} else {
					match s.iterator.next() {
						None => None,
						Some(r) => match r {
							Ok(rec) => {
								let data = StringRow::new(&rec);
								let atom = StringRowAtom(data);
								Some(atom)
							}
							Err(x) => {
								let msg = format!("Error reading CSV file: {}", x);
								let err = Error::Parse(msg);
								self.state = SourceState::Broken(err);
								None
							}
						}
					}
				}
			}
		}
	}
}
