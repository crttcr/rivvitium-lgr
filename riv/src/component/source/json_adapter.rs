use std::fmt::Debug;
use std::marker::PhantomData;
use csv::{ByteRecord, ReaderBuilder};
use crate::model::coordinate::coordinate::Coordinate;
use crate::model::coordinate::coordinate::Coordinate::Position;
use crate::model::coordinate::text_location::TextLocation;
use crate::model::ir::atom::Atom;
use crate::model::ir::atom::Atom::HeaderRow;
use std::str;
use crate::Error;
use crate::error::IoErrorWrapper;
use std::fmt;
use std::io::Read;
use serde_json::{Deserializer, StreamDeserializer, Value};
use serde_json::de::IoRead;
use crate::model::ir::string_row::StringRow;

pub struct JsonArrayState<'de, R: Read> {
	pub stream:  StreamDeserializer<'de, IoRead<R>, Value>,
}

impl<'de, R:Read> JsonArrayState<'de, R> {
	pub fn new<'a>(reader: R) -> Self {
	   let deserializer = Deserializer::from_reader(reader);
		let stream       = deserializer.into_iter::<Value>(); // 2) Turn it into a streaming iterator of `Value`
		JsonArrayState {stream}
	}
}

impl<'de, R: Read> Debug for JsonArrayState<'de, R> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("JsonArrayState")
		.finish()
	}
}

pub fn compute_coordinate() -> Coordinate {
		Coordinate::Undefined
}

