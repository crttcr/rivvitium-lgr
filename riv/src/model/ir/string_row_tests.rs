use csv::ByteRecord;
use crate::model::ir::string_row::StringRow;

#[test]
fn test_new_count_and_iteration() {
	// Create a ByteRecord from string slices
	let rec = ByteRecord::from(vec!["a", "b", "c"]);
	let row = StringRow::new(&rec);

	// count and is_empty
	assert_eq!(row.count(), 3);
	assert!(!row.is_empty());

	// by-reference iteration yields &String
	let fields_ref: Vec<&String> = (&row).into_iter().collect();
	let a = "a".to_string();
	let b = "b".to_string();
	let c = "c".to_string();
	let expected_ref = vec![&a, &b, &c];
	assert_eq!(fields_ref, expected_ref);

	// by-value iteration consumes and yields String
	let fields_val: Vec<String> = row.into_iter().collect();
	let expected_val = vec![a, b, c];
	assert_eq!(fields_val, expected_val);
}

#[test]
fn test_empty_row() {
	let rec = ByteRecord::new();
	let row = StringRow::new(&rec);
	assert_eq!(row.count(), 0);
	assert!(row.is_empty());
	assert_eq!((&row).into_iter().count(), 0);
	assert_eq!(row.into_iter().count(), 0);
}

#[test]
fn test_invalid_utf8_fields() {
	// Push an invalid UTF-8 sequence
	let mut rec = ByteRecord::new();
	rec.push_field(&[0xFF, 0xFF, 0xFF]);
	let row = StringRow::new(&rec);

	// Should treat an invalid sequence as empty string
	assert_eq!(row.count(), 1);
	let mut iter = row.into_iter();
	assert_eq!(iter.next(), Some(String::new()));
	assert_eq!(iter.next(), None);
}


 /// Helper to create a dummy ByteRecord from an array of &str.
 fn make_byte_record(fields: &[&str]) -> ByteRecord {
	  let mut record = ByteRecord::new();
	  for &f in fields {
			record.push_field(f.as_bytes());
	  }
	  record
 }

 #[test]
 fn into_iter_by_reference() {
	  let byte_record   = make_byte_record(&["foo", "bar", "baz"]);
	  let string_row    = StringRow::new(&byte_record);
	  let mut collected = Vec::new();
	  for s in &string_row {
			collected.push(s.clone());
	  }
	  assert_eq!(collected, vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]);
 }

 #[test]
 fn iter_str_yields_str_slices() {
	  let br = make_byte_record(&["one", "two"]);
	  let row = StringRow::new(&br);
	  let mut iter = row.iter_str();
	  assert_eq!(iter.next(), Some("one"));
	  assert_eq!(iter.next(), Some("two"));
	  assert_eq!(iter.next(), None);
 }

 #[test]
 fn iter_bytes_yields_byte_slices() {
	  let br = make_byte_record(&["αβ", "γδ"]);
	  let row = StringRow::new(&br);
	  let mut iter = row.iter_bytes();
	  assert_eq!(iter.next(), Some("αβ".as_bytes()));
	  assert_eq!(iter.next(), Some("γδ".as_bytes()));
	  assert_eq!(iter.next(), None);
 }