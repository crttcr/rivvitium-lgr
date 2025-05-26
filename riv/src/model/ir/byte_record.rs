

#[derive(Debug)]
pub struct ByteRecord {
	bytes:     Vec<u8>,      // Convert to [u8] an array of bytes
	indices:   Vec<usize>,   // Convert to a fixed size array of indices
}