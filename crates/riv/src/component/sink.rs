pub mod capture_sink;
pub mod console_sink;
pub mod csv_sink;
pub mod dev_null_sink;
pub mod json_sink;
pub mod kafka_sink;
pub mod pubsub_sink;
pub mod sink_settings;
pub mod sqlite_sink;
pub mod sql_server_sink;

use std::fmt::{self, Debug, Display};
use std::path::{Path, PathBuf};
use crate::component::sink::sink_settings::SinkSettings;
use crate::model::ir::atom::Atom;
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SinkKind {
    Capture,    // Retains the data
    Console,    // Prints to console
    Csv,        // Creates a CSV file
    DevNull,    // Black hole
    Json,       // Creates a JSON file
    Kafka,      // Publishes Kafka messages
    PubSub,     // Sends PubSub messages
    Sqlite,     // Creates a Sqlite database
    SqlServer,  // Writes to a SqlServer database
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
impl Display for SinkKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SinkKind::Capture    => "Capture (in-memory)",
            SinkKind::Console    => "Console",
            SinkKind::Csv        => "CSV file",
            SinkKind::DevNull    => "Null sink",
            SinkKind::Json       => "JSON file",
            SinkKind::Kafka      => "Kafka producer",
            SinkKind::PubSub     => "PubSub producer",
            SinkKind::Sqlite     => "Sqlite database",
            SinkKind::SqlServer  => "SQL Server database",
        };
        f.write_str(label)
    }
}

pub trait Sink
{
	fn kind(&self)                               -> SinkKind;
	fn initialize(&mut self, cfg: &SinkSettings) -> Result<(), Error>;
	fn accept(&mut self, atom: Atom)             -> Result<(), Error>;
	fn close(&mut self);
}
