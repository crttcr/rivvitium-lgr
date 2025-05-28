use std::fmt::Debug;
use std::fs::File;
use std::marker::PhantomData;
use csv::{ByteRecord, ReaderBuilder};
use crate::model::coordinate::coordinate::Coordinate;
use crate::model::coordinate::coordinate::Coordinate::Position;
use crate::model::coordinate::text_location::TextLocation;
use crate::model::ir::atom::Atom;
use crate::model::ir::atom::Atom::HeaderRow;
use crate::model::ir::data_record::DataRecord;
use std::str;
use crate::component::source::csv_source::CsvSource;
use crate::Error;
use crate::error::IoErrorWrapper;
use std::fmt;

//
// I tried using the Type State pattern for CsvState to get compile-time
// enforcement of the correct order of operations. However, that was complicated
// because this is embedded with the CsvSource which already has a state enum
// and this object is part of the Ready() variant. So it was getting complicated
// and I relented, for now.
//

pub struct CsvState<'a> {
	pub header_atom:      Option<Atom<'a>>,
	pub iterator:         csv::ByteRecordsIntoIter<File>,
}

impl<'a> CsvState<'a> {
	pub fn new(file_path: &String) -> Result<Self, Error> {
		let file        = File::open(file_path).map_err(IoErrorWrapper::from)?;
		let mut reader  = ReaderBuilder::new().delimiter(b';').from_reader(file);
		let headers     = reader.byte_headers().map_err(|e| Error::Parse(e.to_string()))?;
		let header_atom = compute_headers(headers);
		let header_atom = Some(header_atom);
		let iterator    = reader.into_byte_records();
		let csv_state   = CsvState {header_atom, iterator};
		Ok(csv_state)
	}
}

impl<'a> Debug for CsvState<'a> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("CsvState")
		.field("header_atom", &self.header_atom)
		.finish()
	}
}

pub fn compute_coordinate(r: &ByteRecord) -> Coordinate {
	match r.position() {
		None    => Coordinate::Undefined,
		Some(p) => {
			let line     = p.line();
			let byte     = p.byte();
			let column   = 0;
			let location = TextLocation{line, column, byte};
			Position(location)
		}
	}
}

// TODO: This should only be used to get the header values.
// Right now we don't have a better approach so this is also being applied
// to data rows which means a bunch of allocs and string conversions for
// every record we read in ...
//
// TODO: Error reporting when we fail to convert
//
pub fn extract_strings(r: &ByteRecord) -> Vec<String> {
	let mut rv = Vec::new();
	for field in r.iter() {
		match str::from_utf8(field) {
			Ok(s)  => rv.push(s.to_owned()),
			Err(_) => rv.push("".to_owned())
		}
	}
	rv
}

pub fn compute_headers(r: &ByteRecord) -> Atom {
	let data_record = DataRecord::new(r);
	HeaderRow(data_record)
}
