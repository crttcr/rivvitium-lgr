use crate::component::source::{Source, SourceState};
use crate::model::ir::atom::Atom;
use crate::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufReader, Read};
use tracing::instrument;
use crate::component::source::json_adapter::JsonArrayState;
use crate::model::ir::atom::Atom::{StringNVAtom, StringRowAtom};
use crate::model::ir::string_row::StringRow;
use serde_json::{Deserializer, StreamDeserializer, Value};
use serde_json::de::IoRead;
use crate::model::ir::nv_strings::NVStrings;

type JsonArraySourceState<'de, R> = SourceState<JsonArrayState<'de, R>>;

#[derive(Debug)]
pub struct JsonArraySource<'de, R: Read> {
	file_path:  String,
	state:      JsonArraySourceState<'de, R>,
}

impl<'de> Source for JsonArraySource<'de, BufReader<File>> {
	fn initialize<CFG: Display + Debug>(&mut self, _cfg: &CFG) -> Result<(), Error> {
		let file      = File::open(self.file_path.clone()).expect("Failed to open json array file");
		let reader    = BufReader::new(file);
		let state     = JsonArrayState::new(reader);
		self.state    = SourceState::Ready(state);
		Ok(())
	}

	fn finish(&mut self) -> Result<bool, Error> {
		Ok(true)
	}
}

impl<'de, R: Read> JsonArraySource<'de, R> {
	pub fn new(file_path: String) -> Self {
		let state = SourceState::Uninitialized;
		JsonArraySource{file_path, state}
	}
}

impl<'de, BufReader<File>> Iterator for JsonArraySource<'de, BufReader> {
	type Item = Atom;
	fn next(&mut self) -> Option<Self::Item> {

		let handle_broken = |x| -> Option<Self::Item> {
			eprintln!("JsonArraySource: Next called on broken producer: {}", x);
			None
		};

		// TODO: Instrument this so that we're capturing next on a completed source
		let handle_completed = || -> Option<Self::Item> {
			eprintln!("JsonArraySource: Next called on completed producer");
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
			SourceState::Completed   => handle_completed(),
			SourceState::Ready(s) => {
				match s.next() {
					Some(v) => {
						let nv = NVStrings::new(Vec::new());
						let atom = StringNVAtom(nv);
						Some(atom)
					}
					None => {
						None
					}
				}
			}
		}
	}
}
