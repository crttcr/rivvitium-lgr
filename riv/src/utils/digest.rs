use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};

pub const SHA256_EMPTY_STRING: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub fn sha256_digest_file(file_path: &str) -> Result<String, std::io::Error> {
	let input      = File::open(file_path)?;
	let mut reader = BufReader::new(input);
	let mut hasher = Sha256::new();
	let mut buffer = [0; 1024];
	loop {
		let count = reader.read(&mut buffer)?;
		if count == 0 {
			break;
		}
		hasher.update(&buffer[..count]);
	}
	let result  = hasher.finalize();
	let encoded = hex::encode(result);
	Ok(encoded)
}

// Assuming this is your function:
pub fn sha256_digest_string(input: &str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(input.as_bytes());
	let result  = hasher.finalize();
	let encoded = hex::encode(result);
	encoded
}