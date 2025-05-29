use crate::component::source::csv_adapter::CsvState;
use crate::component::source::{Source, SourceState};
use crate::model::ir::atom::Atom;
use crate::Error;
use std::fmt::{Debug, Display};
use tracing::instrument;
use crate::model::ir::atom::Atom::StringRowAtom;
use crate::model::ir::string_row::StringRow;

type CsvSourceState = SourceState<CsvState>;

#[derive(Debug)]
pub struct CsvStringSource {
	file_path:  String,
	state:      CsvSourceState,
}

impl Source for CsvStringSource {
	#[instrument]
	fn initialize<CFG: Display + Debug>(&mut self, _cfg: &CFG) -> Result<(), Error> {
		let csv_state = CsvState::new(&self.file_path)?;
		self.state    = SourceState::Ready(csv_state);
		Ok(())
	}

	#[instrument]
	fn finish(&mut self) -> Result<bool, Error> {
		Ok(true)
	}
}

impl CsvStringSource {
	pub fn new(file_path: String) -> Self {
		let state = SourceState::Uninitialized;
		CsvStringSource {file_path, state}
	}

	fn handle_read_error(x: csv::Error) -> CsvSourceState {
		let msg = format!("Error reading CSV file: {}", x);
		let err = Error::Parse(msg);
		SourceState::Broken(err)
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
			SourceState::Uninitialized => {
				let msg = String::from("Uninitialized CSV");
				eprintln!("{msg}");
				let err = Error::General(msg);
				self.state = SourceState::Broken(err);
				None
			}
			SourceState::Broken(err) => handle_broken(err),
			SourceState::Completed => handle_completed(),
			SourceState::Ready(s) => {
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
