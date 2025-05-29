use csv::ByteRecord;
use std::str;

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
}

impl<'a> IntoIterator for &'a StringRow {
	type Item     = &'a String;
	type IntoIter = std::slice::Iter<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.values.iter()
	}
}

impl IntoIterator for StringRow {
	type Item     = String;
	type IntoIter = std::vec::IntoIter<String>;

	fn into_iter(self) -> Self::IntoIter {
		// Box<[T]> has .into_vec() method to recover Vec<T>
		self.values.into_vec().into_iter()
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
