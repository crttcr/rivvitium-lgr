use std::fmt::Debug;
use std::fs::File;
use std::marker::PhantomData;
use csv::{ByteRecord, ReaderBuilder};
use crate::model::coordinate::coordinate::Coordinate;
use crate::model::coordinate::coordinate::Coordinate::Position;
use crate::model::coordinate::text_location::TextLocation;
use crate::model::ir::atom::Atom;
use crate::model::ir::atom::Atom::HeaderRow;
use std::str;
use crate::component::source::csv_string_source::CsvStringSource;
use crate::Error;
use crate::error::IoErrorWrapper;
use std::fmt;
use crate::model::ir::string_row::StringRow;

//
// I tried using the Type State pattern for CsvState to enable order of operations
// enforcement at compile-time. However, that was complicated
// because this is embedded with the CsvSource which already has a state enum
// and this object is part of the Ready() variant. So it was getting complicated
// and I relented, for now.
//

pub struct CsvState {
	pub header_atom:      Option<Atom>,
	pub iterator:         csv::ByteRecordsIntoIter<File>,
}

impl CsvState {
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

impl Debug for CsvState {
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

pub fn compute_headers(r: &ByteRecord) -> Atom {
	let data_record = StringRow::new(r);
	HeaderRow(data_record)
}
