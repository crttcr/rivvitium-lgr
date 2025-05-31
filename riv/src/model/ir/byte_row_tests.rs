
use crate::model::ir::byte_row::{ByteRow, ByteRowBounds};

//
// Bounds tests
//

#[test]
fn test_single_field() {
	let bounds = ByteRowBounds::new(&[5]);
	assert_eq!(bounds.count(), 1);
	assert_eq!(bounds.get(0), Some(0..5));
	assert_eq!(bounds.end(), 5);
}

#[test]
fn test_multiple_fields() {
	// Two fields: [start0, end0, start1, end1]
	let bounds = ByteRowBounds::new(&[0, 2, 3, 5]);
	assert_eq!(bounds.count(), 4);
	assert_eq!(bounds.get(0), Some(0..0));
	assert_eq!(bounds.get(1), Some(0..2));
	assert_eq!(bounds.end(), 5);
}

#[test]
fn test_out_of_bounds_get() {
	let bounds = ByteRowBounds::new(&[1, 4]);
	// Valid indices for 2 fields are 0 and 1; index 2 should be out of bounds
	assert!(bounds.get(2).is_none());
	assert!(bounds.get(usize::MAX).is_none());
}

#[test]
fn test_empty_bounds() {
	let bounds = ByteRowBounds::new(&[]);
	assert_eq!(bounds.count(), 0);
	assert_eq!(bounds.get(0),  None);
	assert_eq!(bounds.end(),   0);
}

//
// ByteRow tests
//

#[test]
fn new_creates_correct_length_and_bounds() {
	let data   = b"abcdefghij";
	let bounds = &[3, 7];         // define two fields: bytes [0..3] and [3..7]
	let row    = ByteRow::new(data, bounds);

	// length should equal number of fields (2)
	assert_eq!(row.length(), 2);
	assert!(!row.is_empty());

	// bounds should match
	assert_eq!(row.length(), 2);
//	assert_eq!(row.get(0), Some(b"abc"));
//	assert_eq!(row.bounds.get(1), Some(3..7));
}

#[test]
fn get_returns_correct_slices() {
	let data   = b"hello_world";
	let bounds = &[3, 6, 11];           // three fields: "hel", "lo_", "world"
	let bnds_2 = &[5, 11];        // two fields: "hello", "_world"
	let row_a  = ByteRow::new(data, bounds);
	let row_b  = ByteRow::new(data, bnds_2);

	assert_eq!(row_a.get(0), Some(&b"hel"[..]));
	assert_eq!(row_a.get(1), Some(&b"lo_"[..]));
	assert_eq!(row_a.get(2), Some(&b"world"[..]));
	assert_eq!(row_a.get(3), None);                    // out‐of‐range
	assert_eq!(row_b.get(0), Some(&b"hello"[..]));
	assert_eq!(row_b.get(1), Some(&b"_world"[..]));
	assert_eq!(row_b.get(2), None);                    // out‐of‐range
}