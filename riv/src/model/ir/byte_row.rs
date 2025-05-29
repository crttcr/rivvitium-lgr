use std::ops::Range;
use std::fmt;

pub struct ByteRow {
	values:     Box<[u8]>,
	bounds:     ByteRowBounds,
	length:     u32,
}

impl ByteRow {
	pub fn new(data: &[u8], bounds: &[usize]) -> Self {
		let values = data.to_vec().into_boxed_slice();
		let bounds = ByteRowBounds::new(bounds);
		let length = bounds.count as u32;
		ByteRow{values, bounds, length}
	}
	pub fn length(&self)   -> u32  { self.length      }
	pub fn is_empty(&self) -> bool { self.length == 0 }
	
	pub fn get(&self, index: usize) -> Option<&[u8]> {
		let range = self.bounds.get(index)?;
		Some(&self.values[range.start..range.end])
	}
}


impl fmt::Debug for ByteRow {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ByteRow")
			.field("len", &self.length)
			.finish()
	}
}


/// The bounds of fields in a single record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ByteRowBounds {
	/// The starting and ending positions of each field
	indices: Vec<usize>,
	/// Number of fields
	count:  usize,
}

impl ByteRowBounds {
	pub fn new (values: &[usize]) -> ByteRowBounds {
		let indices = values.to_vec();
		let count  = indices.len() / 2;
		ByteRowBounds{indices, count}
	}

	/// Returns the bounds of field `i`.
	#[inline]
	pub fn get(&self, i: usize) -> Option<Range<usize>> {
		if i >= self.count {
			return None;
		}
		let a     = i * 2;
		let b     = a + 1;
		unsafe {
			let s = *self.indices.get_unchecked(a);
			let e = *self.indices.get_unchecked(b);
			Some(s..e)
		}
	}

	/// Return the last position of the last field.
	///
	/// If there are no fields, this returns `0`.
	#[inline]
	pub fn end(&self) -> usize {
		self.indices.last().map(|&i| i).unwrap_or(0)
	}

	/// Returns the number of fields in these bounds.
	#[inline]
	pub fn count(&self) -> usize { self.count }
}
