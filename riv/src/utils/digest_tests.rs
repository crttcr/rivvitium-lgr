use crate::utils::digest::sha256_digest_file;
use crate::utils::digest::sha256_digest_string;
use crate::utils::digest::SHA256_EMPTY_STRING;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Error as IoError, ErrorKind as IoErrorKind, Write}; // Added ErrorKind

// This expected hash can be verified using a command-line tool like:
// echo -n "Hello, world!" | sha256sum
// 315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3
//
#[test]
fn test_hello_world_sha256() {
	let input = "Hello, world!";
	let expected_hash = "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3";
	let actual_hash   = sha256_digest_string(input);
	assert_eq!(actual_hash, expected_hash, "SHA256 hash for 'Hello, world!' did not match");
}

// This expected hash can be verified using a command-line tool like:
// echo -n "" | sha256sum
// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
//
#[test]
fn test_empty_string_sha256() {
	let input         = "";
	let expected_hash = SHA256_EMPTY_STRING;
	let actual_hash   = sha256_digest_string(input);
	assert_eq!(actual_hash, expected_hash, "SHA256 hash for an empty string did not match");
}

// Helper function to create a temporary file with specific content
fn create_temp_file(path: &str, content: &[u8]) -> Result<(), std::io::Error> {
	let mut file = File::create(path)?;
	file.write_all(content)?;
	Ok(())
}

// Pre-calculated SHA256 hash for "Hello, Rust file hashing!"
// You can verify this with: echo -n "Hello, Rust file hashing!" | sha256sum
// let expected_hash = "ea79576de559188a0b6b00799e8512577870a906b81919a900baef10e10590be";
// Oddly, this was the hash I got from the command line:
// let expected_hash = "5067ce6ca0e142e80c95fc59e13acbba38ca13857553f7d0b03570d438ab50f5";
//
#[test]
fn test_existing_file_correct_hash() {
	let test_file_path = "test_sample_file.txt";
	let file_content   = b"Hello, Rust file hashing!"; // Content as bytes
	let expected_hash  = "5067ce6ca0e142e80c95fc59e13acbba38ca13857553f7d0b03570d438ab50f5";

	// Create the temporary file
	if let Err(e) = create_temp_file(test_file_path, file_content) {
		panic!("Failed to create temp file for test: {}", e);
	}

	match sha256_digest_file(test_file_path) {
		Ok(actual_hash) => {
			assert_eq!(actual_hash, expected_hash, "SHA256 hash did not match the expected value.");
		}
		Err(e) => {
			panic!("Function returned an error when a valid hash was expected: {}", e);
		}
	}

	// Clean up the temporary file
	if let Err(e) = fs::remove_file(test_file_path) {
		// Non-critical if removal fails in some CI, but good to note.
		eprintln!("Warning: Failed to remove temporary test file '{}': {}", test_file_path, e);
	}
}
