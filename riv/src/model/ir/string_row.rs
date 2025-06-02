use csv::ByteRecord;
use std::str;


/// A row of owned `String` values (e.g. converted from a `ByteRecord`).
#[derive(Debug)]
pub struct StringRow {
	values: Box<[String]>,
}

impl StringRow {
	pub fn new(r: &ByteRecord) -> Self {
		let values = extract_strings(r);
		let values = values.into_boxed_slice();
		StringRow{values}
	}
	
	pub fn count(&self)    -> u32  { self.values.len() as u32}
	pub fn is_empty(&self) -> bool { self.values.is_empty()  }
	
    /// Returns an iterator over `&str` for each field.
    pub fn iter_str(&self) -> StringRowStrIter<'_> {
        StringRowStrIter {
            inner: self.values.iter(),
        }
    }

    /// Returns an iterator over `&[u8]` for each field’s UTF‐8 bytes.
    pub fn iter_bytes(&self) -> StringRowBytesIter<'_> {
        StringRowBytesIter {
            inner: self.values.iter(),
        }
    }	
}
/// When you do `for s in &string_row`, this yields `&String`.
impl<'a> IntoIterator for &'a StringRow {
	type Item     = &'a String;
	type IntoIter = std::slice::Iter<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.values.iter()
	}
}

/// When you consume the `StringRow`, you get `String` values.
impl IntoIterator for StringRow {
	type Item     = String;
	type IntoIter = std::vec::IntoIter<String>;

	fn into_iter(self) -> Self::IntoIter {
		// Box<[T]> has .into_vec() method to recover Vec<T>
		self.values.into_vec().into_iter()
	}
}

/// Iterator over `&str` for each field in a `StringRow`.
pub struct StringRowStrIter<'a> {
    inner: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for StringRowStrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| s.as_str())
    }
}

/// Iterator over `&[u8]` (UTF‐8 bytes) for each field in a `StringRow`.
pub struct StringRowBytesIter<'a> {
    inner: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for StringRowBytesIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| s.as_bytes())
    }
}

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
