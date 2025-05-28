use crate::component::source::csv_adapter::CsvState;
use crate::component::source::{Source, SourceState};
use crate::model::ir::atom::Atom;
use crate::model::ir::atom::Atom::ValueSequence;
use crate::model::ir::data_record::DataRecord;
use crate::Error;
use std::fmt::{Debug, Display};
use tracing::instrument;

type CsvSourceState<'a> = SourceState<CsvState<'a>>;

#[derive(Debug)]
pub struct CsvSource<'a> {
	file_path:  String,
	state:      CsvSourceState<'a>,
}

impl<'a> Source<'a> for CsvSource<'a> {
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

impl<'a> CsvSource<'a> {
	pub fn new(file_path: String) -> Self {
		let state = SourceState::Uninitialized;
		CsvSource{file_path, state}
	}

	fn handle_read_error(x: csv::Error) -> CsvSourceState<'a> {
		let msg = format!("Error reading CSV file: {}", x);
		let err = Error::Parse(msg);
		SourceState::Broken(err)
	}
}

impl<'a> Iterator for CsvSource<'a> {
	type Item = Atom<'a>;
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
								let data = DataRecord::new(&rec);
								let atom = ValueSequence(data);
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