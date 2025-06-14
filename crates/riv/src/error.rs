pub mod io_error_wrapper;

use std::fmt;
use thiserror::Error;
pub use crate::error::io_error_wrapper::IoErrorWrapper;
use rusqlite::{Error as RusqliteError, ErrorCode}; // Alias for clarity
use rusqlite::ErrorCode as RusqliteErrorCode;
use rusqlite::ffi;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error
{
	#[error("Invalid Configuration: {0}")]
	InvalidConfig(String),

	#[error("Invalid input: {0}")]
	Parse(String),

	#[error("Invalid input: {0}")]
	InvalidInput(String),

	// Use the new IoErrorWrapper.
	// The #[from] here allows IoErrorWrapper to be converted into Error::Io
	// The `#[error("IO error: {source}")]` uses the Display impl of IoErrorWrapper
	//
	#[error("IO error: {source}")]
	Io { #[from] source: IoErrorWrapper },

	#[error("NotFound: {0}")]
	NotFound(String),

	#[error("General error: {0}")]
	General(String),
}

/// Convert a `csv::Error` into our `Error` enum.
///
use std::{error::Error as StdError, io};
impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        // Try to see if the csv::Error was caused by an underlying io::Error
        if let Some(source) = err.source() {
            // If the source is an io::Error, wrap it
            if let Some(io_err) = source.downcast_ref::<io::Error>() {
                // Create a fresh io::Error with the same kind and message
                let cloned = io::Error::new(io_err.kind(), io_err.to_string());
                let wrapper = IoErrorWrapper::from(cloned);
                return Error::Io { source: wrapper };
            }
        }
        // Otherwise, treat it as a parse error
        Error::Parse(err.to_string())
    }
}

fn is_error_code_an_io_error(code: &ErrorCode) -> bool {
	match code { 
		RusqliteErrorCode::CannotOpen                  => true,
		RusqliteErrorCode::DatabaseBusy                => true,
		RusqliteErrorCode::DiskFull                    => true,
		RusqliteErrorCode::FileLockingProtocolFailed   => true,
		RusqliteErrorCode::NotADatabase                => true,
		RusqliteErrorCode::PermissionDenied            => true,
		RusqliteErrorCode::ReadOnly                    => true,
		RusqliteErrorCode::SystemIoFailure             => true,
		_ => return false,
	}
	
}

// Corrected `From<rusqlite::error::Error>` implementation
impl From<RusqliteError> for Error {
    fn from(err: RusqliteError) -> Self {
        match err {
            // Case 1: `RusqliteError::SqliteFailure` - this is the most common way
            // SQLite reports underlying issues, including I/O errors.
            RusqliteError::SqliteFailure(ffi_error, message) => {
                // Check if the underlying SQLite FFI error code indicates an I/O error
                // FIXME: This is a hack. We have other error codes that could be I/O errors
                if is_error_code_an_io_error(&ffi_error.code) {
                    // Create a synthetic io::Error or directly use IoErrorWrapper
                    // as we don't get a direct std::io::Error object from ffi_error
                    let io_msg = format!("SQLite IO error (code {:?}): {}", ffi_error.code, message.unwrap_or_else(|| "Unknown IO error".to_string()));
                    let synthetic_io_err = io::Error::new(io::ErrorKind::Other, io_msg); // Use Other kind, as specific kind might be ambiguous
                    Error::Io { source: IoErrorWrapper::from(synthetic_io_err) }
                } else {
                    // Otherwise, it's a general SQLite error
                    let msg = message.unwrap_or_else(|| ffi_error.to_string());
                    Error::General(format!("SQLite error (code {:?}): {}", ffi_error.code, msg))
                }
            }
            // Case 2: `RusqliteError::FromSqlConversionFailure` - This variant
            // contains a boxed `dyn StdError` which *could* be an `io::Error`.
            RusqliteError::FromSqlConversionFailure(idx, ty, source) => {
                if let Some(io_err) = source.downcast_ref::<io::Error>() {
                    // If the underlying cause of conversion failure is an IO error
                    let cloned_io = io::Error::new(io_err.kind(), io_err.to_string());
                    Error::Io { source: IoErrorWrapper::from(cloned_io) }
                } else {
                    // Otherwise, it's a parsing/conversion issue
                    Error::Parse(format!("Failed to convert SQL value from column {} (type {:?}): {}", idx, ty, source))
                }
            }
            // Other rusqlite errors can be mapped as before:
            RusqliteError::QueryReturnedNoRows => {
                Error::NotFound("No matching record found.".to_string())
            }
            RusqliteError::InvalidParameterName(name) => {
                Error::InvalidInput(format!("Invalid SQL parameter name: {}", name))
            }
            RusqliteError::InvalidColumnType(idx, expected, found) => {
                Error::Parse(format!("Invalid column type at index {}: expected {:?}, found {:?}", idx, expected, found))
            }
            RusqliteError::InvalidColumnIndex(idx) => {
                Error::InvalidInput(format!("Invalid column index: {}", idx))
            }
            RusqliteError::InvalidColumnName(name) => {
                Error::InvalidInput(format!("Invalid column name: {}", name))
            }
            RusqliteError::IntegralValueOutOfRange(idx, is_positive) => {
                Error::Parse(format!("Integral value out of range at column {}: is_positive={}", idx, is_positive))
            }
            RusqliteError::Utf8Error(e) => {
                Error::Parse(format!("UTF-8 decoding error: {}", e))
            }
            RusqliteError::NulError(e) => {
                Error::Parse(format!("Nul byte error: {}", e))
            }
            // Catch-all for any future or unhandled rusqlite errors (due to #[non_exhaustive])
            _ => Error::General(format!("An unexpected database error occurred: {}", err)),
        }
    }
}


// Example Usage (for demonstration, typically in a test or main function)
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};
    use std::fs;
use std::process::id;
use tempfile::NamedTempFile;

    #[test]
    fn test_rusqlite_io_error_conversion_sqlite_failure() {
        // Create a temporary file path that is likely to cause an IO error if treated as a dir
        let temp_dir = tempfile::tempdir().unwrap();
        let non_existent_file_in_dir = temp_dir.path().join("sub_dir").join("test.db"); // Path that shouldn't exist directly
        fs::create_dir(&non_existent_file_in_dir.parent().unwrap()).unwrap(); // Create the sub_dir

        // Try to open a database where the path structure is wrong (e.g., trying to write to a dir as a file)
        let result: Result<Connection, RusqliteError> = Connection::open(&temp_dir);
        println!("{:?}", result);
        let err = result.unwrap_err();

        let converted_err: Error = err.into();
        println!("{:?}", converted_err);
        assert!(matches!(converted_err, Error::Io { .. }));
        if let Error::Io { source } = converted_err {
            // The exact message might vary by OS, but should indicate an IO error
            // Check for a common IO error kind that would occur in such a scenario
            assert!(
                source.kind() == io::ErrorKind::NotFound || // On some OS, it's not found
                source.kind() == io::ErrorKind::PermissionDenied || // On others, it might be permission
                source.kind() == io::ErrorKind::Other // Or just a generic OS error
            );
            assert!(source.to_string().contains("SQLite IO error") || source.to_string().contains("unable to open database file"));
        }
    }

    #[test]
    fn test_rusqlite_query_returned_no_rows_conversion() {
        let db = Connection::open_in_memory().unwrap();
        db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)", []).unwrap();

        let result: Result<String, RusqliteError> = db.query_row(
            "SELECT name FROM users WHERE id = ?",
            params![1],
            |row| row.get(0),
        );
        let err                  = result.unwrap_err();
        let converted_err: Error = err.into();
        let expected             = Error::NotFound(format!("No matching record found."));
        assert_eq!(converted_err, expected);
    }

    #[test]
    fn test_rusqlite_invalid_column_type_conversion() {
        let db = Connection::open_in_memory().unwrap();
        db.execute("CREATE TABLE items (id INTEGER PRIMARY KEY, value TEXT)", []).unwrap();
        db.execute("INSERT INTO items (id, value) VALUES (1, 'hello')", []).unwrap();

        // Try to read a TEXT column as an INTEGER
        let result: Result<i32, RusqliteError> = db.query_row(
            "SELECT value FROM items WHERE id = 1",
            [],
            |row| row.get(0),
        );
        let err = result.unwrap_err();

        let converted_err: Error = err.into();
        assert!(matches!(converted_err, Error::Parse(_)));
        if let Error::Parse(msg) = converted_err {
            assert!(msg.contains("Invalid column type at index 0"));
            assert!(msg.contains("found Text"));
        }
    }

    #[test]
    fn test_rusqlite_sqlite_failure_conversion() {
        let db = Connection::open_in_memory().unwrap();
        db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", []).unwrap();
        db.execute("INSERT INTO test (id) VALUES (1)", []).unwrap();

        // Attempt to insert duplicate primary key, causing a constraint violation
        let result = db.execute("INSERT INTO test (id) VALUES (1)", []);
        let err = result.unwrap_err();

        let converted_err: Error = err.into();
        assert!(matches!(converted_err, Error::General(_)));
        if let Error::General(msg) = converted_err {
            assert!(msg.contains("SQLite error"));
            assert!(msg.contains("UNIQUE constraint failed"));
        }
    }

    #[test]
    fn test_rusqlite_from_sql_conversion_failure_with_io_source() {
        use rusqlite::types::Type;
        // This is a bit harder to trigger directly in rusqlite with an actual io::Error source.
        // We'll simulate it by creating the error manually for the test.
        let io_error = io::Error::new(io::ErrorKind::BrokenPipe, "Simulated underlying IO issue");
        let rusqlite_err = RusqliteError::FromSqlConversionFailure(
            0,
            Type::Text,
            Box::new(io_error)
        );

        let converted_err: Error = rusqlite_err.into();
        assert!(matches!(converted_err, Error::Io { .. }));
        if let Error::Io { source } = converted_err {
            assert_eq!(source.kind(), io::ErrorKind::BrokenPipe);
            assert_eq!(source.to_string(), "BrokenPipe: Simulated underlying IO issue");
        }
    }
}