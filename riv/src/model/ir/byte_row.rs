use std::ops::Range;
use std::fmt;

pub struct ByteRow {
	values:     Box<[u8]>,
	bounds:     ByteRowBounds,
}

impl ByteRow {
	pub fn new(data: &[u8], ends: &[usize]) -> Self {
		let values = data.to_vec().into_boxed_slice();
		let bounds = ByteRowBounds::new(ends);
		ByteRow{values, bounds}
	}

	pub fn length(&self)   -> u32  { self.bounds.count()      }
	pub fn is_empty(&self) -> bool { self.bounds.count() == 0 }

	pub fn get(&self, index: usize) -> Option<&[u8]> {
		let range = self.bounds.get(index)?;
		Some(&self.values[range.start..range.end])
	}
}

/// Implement `Debug` so that it prints the field‐values as UTF-8 strings:
impl fmt::Debug for ByteRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect each field slice as a UTF-8 String (using lossless conversion).
        let fields: Vec<String> = (0..self.bounds.count())
            .filter_map(|i| {
                self.get(i as usize).map(|bytes| {
                    // Convert &[u8] to String, replacing invalid UTF-8 with �
                    String::from_utf8_lossy(bytes).into_owned()
                })
            })
            .collect();

        // Now format as:
        // ByteRow { length: N, fields: ["foo", "bar", ...] }
        f.debug_struct("ByteRow")
            .field("length", &self.length())
            .field("fields", &fields)
            .finish()
    }
}

/// The bounds of fields in a single record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ByteRowBounds {
	/// The ending positions of each field
	indices: Vec<usize>,
}

impl ByteRowBounds {
	pub fn new (values: &[usize]) -> ByteRowBounds {
		let indices = values.to_vec();
		ByteRowBounds{indices}
	}

	/// Returns the bounds of field `i` if it exists, and None if it does not.
	#[inline]
	pub fn get(&self, i: usize) -> Option<Range<usize>> {
		let count = self.count();
		if count < 1         { return None; }
		if i as u32 >= count { return None; }
		let end   = self.indices[i];
		let start = if i == 0 { 0 } else { self.indices[i - 1] };
		Some(start..end)
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
	pub fn count(&self) -> u32 {self.indices.len() as u32}
}
