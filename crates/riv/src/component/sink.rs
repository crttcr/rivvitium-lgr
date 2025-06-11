pub mod capture_sink;
pub mod console_sink;
pub mod csv_sink;
pub mod sqlite_sink;

use std::fmt::{self, Debug, Display};
use crate::model::ir::atom::Atom;
use crate::error::Error;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SinkKind {
    Capture,  // Retains the data
    Console,  // Prints to console
    Csv,      // Creates a CSV file
    DevNull,  // Black hole
    Json,     // Creates a JSON file
    Kafka,    // Publishes Kafka messages
    Sqlite,   // Creates a Sqlite database
}


impl SinkKind {
    pub fn all() -> Vec<SinkKind> {
    	vec![SinkKind::Capture, SinkKind::Console, SinkKind::Csv, SinkKind::DevNull, SinkKind::Json, SinkKind::Sqlite]
    }
}

/// Human-friendly name for each sink variant.
///
/// Display is especially handy for UI lists, logging, or CLI flags:
///
/// ```rust
/// println!("Selected sink: {kind}");
/// // → “CSV file”
/// ```
impl fmt::Display for SinkKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SinkKind::Capture => "Capture (in-memory)",
            SinkKind::Console => "Console",
            SinkKind::Csv     => "CSV file",
            SinkKind::DevNull => "Null sink",
            SinkKind::Json    => "JSON file",
            SinkKind::Kafka   => "Kafka message",
            SinkKind::Sqlite  => "SQLite database",
        };
        f.write_str(label)
    }
}

pub trait Sink<R> 
{
	fn kind(&self)                                        -> SinkKind;
	fn initialize<C: Display + Debug>(&mut self, cfg: &C) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)                      -> Result<(), Error>;
	fn finish(&mut self)                                  -> Result<R,  Error>;
}
