use crate::error::{Error, IoErrorWrapper};
use crate::model::ir::atom::Atom;
use crate::component::sink::Sink;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tracing::{info, instrument};
use crate::model::ir::atom_type::AtomType;
use csv::Writer;

const OUTPUT_PATH: &str = "/tmp";

#[derive(Debug)]
pub struct CsvSink {
	output_file_name: String,
	writer:           Option<Writer<File>>,
}

impl CsvSink {
	pub fn new(output_file_name: String) -> Self {
		Self {output_file_name, writer: None}
	}
}

impl Sink<()> for CsvSink {
	#[instrument]
	fn initialize<C: Display + Debug>(&mut self, _cfg: &C) -> Result<(), Error> {
		let full_path = Path::new(OUTPUT_PATH).join(&self.output_file_name);             // 1) Build the full path: "/tmp/<output_file_name>"
		let file      = File::create(&full_path)                                         // 2) Attempt to create (or truncate) the file for writing
            .map_err(|e| {
                let wrapper = IoErrorWrapper::from(e);               // Wrap the std::io::Error into your Error::Io
                Error::from(wrapper)
            })?;
		let writer = Writer::from_writer(file);
		self.writer = Some(writer);
		Ok(())
	}

	fn accept(&mut self, atom: Atom) -> Result<(), Error> {
		if atom.atom_type() == AtomType::Control {return Ok(())}
		let mut writer = self.writer.take().unwrap();
		let e_map      = |e: csv::Error| { Error::from(e) };
		
		match atom {
			Atom::ByteRowAtom(byte_row) => {
				let vec  = byte_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			Atom::StringRowAtom(string_row) => {
				let vec  = string_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			Atom::HeaderRow(header_row) => {
				let vec  = header_row.into_iter().collect::<Vec<_>>();
				let data = vec.into_boxed_slice();
				writer.write_record(data).map_err(e_map)?;
			},
			_ => {},
		}
		self.writer = Some(writer);
		Ok(())
	}

	#[instrument (skip(self),fields(self = "CsvSink", output_file_name=%self.output_file_name))]
	fn finish(&mut self) -> Result<(), Error> {
		if let Some(buf) = self.writer.take() {
			let mut inner = buf.into_inner();
			match inner {
				Err(error)       => { Err(Error::General(error.to_string()))},
				Ok(ref mut file) => { let _ = file.flush(); Ok(()) },
			}
		} else {
			let msg = "Finish called but struct contains no writer.".to_owned();
			Err(Error::General(msg))
		}
	}
}
