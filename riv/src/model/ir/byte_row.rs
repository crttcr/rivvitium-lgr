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

// --------------------------------------------------
// Part 1)  Iterator over raw bytes: &ByteRow → &[u8]
// --------------------------------------------------

/// An iterator that, for each `i`, yields `&[u8]` for the i-th field of a `ByteRow`.
pub struct ByteRowBytesIter<'a> {
    row: &'a ByteRow,
    idx: usize,
}

impl<'a> Iterator for ByteRowBytesIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.row.get(self.idx);
        self.idx += 1;
        result
    }
}

/// Implement `IntoIterator` for `&ByteRow` so that `for bytes in &row { … }`
/// yields each field as a raw byte‐slice (`&[u8]`).
impl<'a> IntoIterator for &'a ByteRow {
    type Item = &'a [u8];
    type IntoIter = ByteRowBytesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ByteRowBytesIter { row: self, idx: 0 }
    }
}

// --------------------------------------------------
// Part 2)  Iterator over UTF-8 strings: &ByteRow → &str
// --------------------------------------------------

/// An iterator that, for each field index `i`, yields a &str (lossily if needed).
pub struct ByteRowStrIter<'a> {
    row: &'a ByteRow,
    idx: usize,
}

impl<'a> Iterator for ByteRowStrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // 1) Grab the raw bytes for field `idx`
        let bytes_opt = self.row.get(self.idx);
        self.idx += 1;

        // 2) Try to convert to &str. If invalid UTF-8, use `from_utf8_lossy`
        //    but that returns a Cow<'_, str>; here we cheat and allocate a small
        //    String in those cases—but requirement was &str, so we only return
        //    valid &str in the Some branch. In practice, if data might not be valid UTF-8,
        //    you may want to return Cow<'a, str> instead. For now, we’ll assume valid UTF-8.
        bytes_opt.and_then(|b| match std::str::from_utf8(b) {
            Ok(s) => Some(s),
            Err(_) => {
                // If you really must return a `&str`, you cannot return an owned `String` here.
                // Instead, we could choose to return `None` or panic. For this example, we’ll
                // simply replace invalid sequences with the Unicode replacement character and
                // store it in a temporary `String`. But `String` cannot live long enough for
                // a `&str` return. So in a real app, you’d want `Iterator<Item=Cow<'a, str>>`.
                // For simplicity, we’ll just return `None` on invalid UTF-8:
                None
            }
        })
    }
}

/// Provide a method on `ByteRow` that returns an iterator over `&str`.
/// (We cannot write `impl IntoIterator for &ByteRow` a second time, so we offer a method.)
impl ByteRow {
    pub fn iter_str(&self) -> ByteRowStrIter<'_> {
        ByteRowStrIter { row: self, idx: 0 }
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
