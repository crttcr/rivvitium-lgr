use csv::ByteRecord;
use crate::component::source::csv_adapter::{extract_strings};

/// DataRecord is intended to be the minimal, shared representation 
/// of positional data values extracted from a source. In the ideal
/// case, we use the raw bytes provided by the source and skip the
/// conversion into Rust's UTF-8 strings in the IR.
///
#[derive(Debug)]
pub struct DataRecord {
	values:     Vec<String>, // Temporary representation to make progres
	
//	bytes:     &[u8],       // Convert to [u8] an array of bytes
//	indices:   [u32],       // Convert to a fixed size array of indices
}

impl DataRecord {
	pub fn new(r: &ByteRecord) -> Self {
		let values = extract_strings(r);
		DataRecord{values}
	}
	
//	pub fn new(bytes: &[u8], indices: Vec<usize>) -> Self {
//		let bytes = bytes.to_vec();
//		DataRecord {bytes, indices}
//	}
	
	pub fn len(&self) -> u32 {
		self.values.len() as u32
	}
	
	pub fn is_empty(&self) -> bool {
		self.values.is_empty()
	}
}
