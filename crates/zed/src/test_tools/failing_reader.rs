use std::io::{self, Read};

/// A custom `Read` implementation that always fails.
///
/// This struct simulates a reader that encounters an error every time
/// its `read` method is invoked.
#[derive(Debug)]
pub struct FailingReader {
    error_message: String,
    error_kind:    io::ErrorKind,
}

impl FailingReader {
    /// Creates a default `FailingReader` instance.
    ///
    pub fn default() -> Self {
		let error_message = "Read failed".to_string();
		let error_kind    = io::ErrorKind::Other;
		FailingReader { error_message, error_kind}
    }
    
    /// Creates a new `FailingReader` instance.
    ///
    /// # Arguments
    ///
    /// * `message` - A string message to include in the `io::Error`.
    /// * `kind` - The `io::ErrorKind` to return with the error.
    pub fn new(message: impl Into<String>, kind: io::ErrorKind) -> Self {
        FailingReader {
            error_message: message.into(),
            error_kind: kind,
        }
    }
}

// Implement the `std::io::Read` trait for `FailingReader`.
impl Read for FailingReader {
    /// Attempts to read bytes into the provided buffer.
    ///
    /// This implementation always returns an `io::Error` with the
    /// configured error message and kind. It never successfully reads any bytes.
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        // Construct and return an error.
        // The `_buf` parameter is ignored because we're simulating a failure.
        Err(io::Error::new(
            self.error_kind,
            self.error_message.as_str(),
        ))
    }
}

/// Module containing test cases for `FailingReader`.
#[cfg(test)] // This attribute ensures the module is only compiled during tests.
mod tests {
    // `use super::*` brings all items from the outer scope into this test module.
    // This includes `FailingReader`, `io`, `Read`, `BufReader`.
    use super::*;

    /// Test case to verify that `FailingReader` consistently returns errors
    /// on direct `read` calls.
    #[test] // Marks this function as a test.
    fn test_failing_reader_direct_calls() {
        // Create an instance of our failing reader
        let mut reader = FailingReader::new("Simulated read failure!", io::ErrorKind::Other);

        // Create a buffer to attempt reading into
        let mut buffer = [0; 10]; // A 10-byte buffer

        // Call the `read` method in a loop to demonstrate repeated failures
        for i in 1..=3 {
            let result = reader.read(&mut buffer);

            // Assert that the result is an error
            assert!(result.is_err(), "Attempt {} should have failed", i);

            let e = result.unwrap_err(); // Unwrap the error to inspect it

            // Assert the error kind and message
            assert_eq!(e.kind(), io::ErrorKind::Other, "Attempt {} error kind mismatch", i);
            assert_eq!(e.to_string(), "Simulated read failure!", "Attempt {} error message mismatch", i);

            // Print for debugging purposes if the test fails or run with `cargo test -- --nocapture`
            eprintln!("Attempt {}: Read failed as expected: {:?}", i, e);
        }
    }

    /// Test case to verify that `FailingReader` also causes `BufReader` to fail.
    #[test]
    fn test_failing_reader_with_buf_reader() {
        // Wrap FailingReader in a BufReader. When BufReader tries to fill its
        // internal buffer, it will call FailingReader's `read` method, which will fail.
        let mut buffered_reader = io::BufReader::new(FailingReader::new("Buffered read failure!", io::ErrorKind::BrokenPipe));
        let mut another_buffer = [0; 20]; // Buffer for the BufReader to write into

        let result = buffered_reader.read(&mut another_buffer);

        // Assert that the result from BufReader is an error
        assert!(result.is_err(), "Buffered read should have failed");

        let e = result.unwrap_err(); // Unwrap the error

        // Assert the error kind and message
        assert_eq!(e.kind(), io::ErrorKind::BrokenPipe, "Buffered read error kind mismatch");
        assert_eq!(e.to_string(), "Buffered read failure!", "Buffered read error message mismatch");

        eprintln!("Buffered read failed as expected: {:?}", e);
    }
}