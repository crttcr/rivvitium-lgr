use std::{cmp, ops};
use std::ops::Range;
use csv::ByteRecord;
use crate::component::source::csv_adapter::{extract_strings};

/// DataRecord is intended to be the minimal, shared representation 
/// of positional data values extracted from a source. In the ideal
/// case, we use the raw bytes provided by the source and skip the
/// conversion into Rust's UTF-8 strings in the IR.
///
#[derive(Debug)]
pub struct DataRecord<'a> {
	bytes:   &'a [u8],
	ends:    Endpoints,
}

impl<'a> DataRecord<'a> {
	pub fn new(r: &ByteRecord) -> Self {
		let bytes  = r.as_slice().iter().cloned().collect::<Vec<_>>();
		let ends   = Endpoints::with_capacity(r.len());
		DataRecord{bytes, ends}
	}
	
//	pub fn new(bytes: &[u8], indices: Vec<usize>) -> Self {
//		let bytes = bytes.to_vec();
//		DataRecord {bytes, indices}
//	}
	
	pub fn len(&self) -> u32 {
		self.ends.len() as u32
	}
	
	pub fn is_empty(&self) -> bool {
		self.ends.len() == 0
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Endpoints {
	/// The ending index of each field.
	ends: Vec<usize>,
	
	/// The number of fields in this record.
	///
	len: usize,
}

impl Default for Endpoints {
	#[inline]
	fn default() -> Endpoints {
		Endpoints::with_capacity(0)
	}
}

impl Endpoints {
	/// Create a new set of bounds with the given capacity for storing the
	/// ends of fields.
	#[inline]
	fn with_capacity(capacity: usize) -> Endpoints {
		Endpoints { ends: vec![0; capacity], len: 0 }
	}

	/// Returns the bounds of field `i`.
	#[inline]
	fn get(&self, i: usize) -> Option<Range<usize>> {
		if i >= self.len {
			return None;
		}
		let end = match self.ends.get(i) {
			None => return None,
			Some(&end) => end,
		};
		let start = match i.checked_sub(1).and_then(|i| self.ends.get(i)) {
			None => 0,
			Some(&start) => start,
		};
		Some(ops::Range { start, end })
	}

	/// Returns a slice of ending positions of all fields.
	#[inline]
	fn ends(&self) -> &[usize] {
		&self.ends[..self.len]
	}

	/// Return the last position of the last field.
	///
	/// If there are no fields, this returns `0`.
	#[inline]
	fn end(&self) -> usize {
		self.ends().last().map(|&i| i).unwrap_or(0)
	}

	/// Returns the number of fields in these bounds.
	#[inline]
	fn len(&self) -> usize {
		self.len
	}

	/// Expand the capacity for storing field ending positions.
	#[inline]
	fn expand(&mut self) {
		let new_len = self.ends.len().checked_mul(2).unwrap();
		self.ends.resize(cmp::max(4, new_len), 0);
	}

	/// Add a new field with the given ending position.
	#[inline]
	fn add(&mut self, pos: usize) {
		if self.len >= self.ends.len() {
			self.expand();
		}
		self.ends[self.len] = pos;
		self.len += 1;
	}
}
