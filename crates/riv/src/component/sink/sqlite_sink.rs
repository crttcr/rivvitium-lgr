use std::{fmt::Debug, path::Path};
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Instant;
use rusqlite::{params_from_iter, Connection, Error as RusqliteError};
use crate::error::{Error, IoErrorWrapper};
use tracing::{error, info, instrument, warn};
use tracing_subscriber::fmt::format;
use zero::component::telemetry::component_metrics::ComponentMetrics;
use zero::component::telemetry::provides_metrics::ProvidesMetrics;
use crate::component::sink::{Sink, SinkKind};
use crate::component::sink::sink_settings::SinkSettings;
use crate::model::ir::atom::Atom;
use crate::model::ir::atom_type::AtomType;

/// A Sink that writes rows into a SQLite database.  
/// 
/// It expects to see a `HeaderRowAtom` first, which defines column names.  
/// Subsequent row atoms (`ByteRowAtom` or `StringRowAtom`) are inserted into the table.
pub struct SqliteSink {
	component_id:     u32,
    /// Filesystem path to the SQLite database file
	file_path:        PathBuf,
    /// Table name to use (fixed as "records")
    table: String,

    /// The open SQLite connection (populated in `initialize`)
    cx: Option<Connection>,
    /// Column names, populated when a `HeaderRowAtom` is accepted
    columns: Vec<String>,
	created_utc:      Instant,
	started_utc:      Instant,
	metrics:          ComponentMetrics,
	tx:               Sender<ComponentMetrics>
}

impl SqliteSink {
    pub fn new(component_id: u32, file_path: PathBuf, table: String, tx: Sender<ComponentMetrics>) -> Self {
		let created_utc = Instant::now();
		let started_utc = created_utc;
		let metrics     = ComponentMetrics::new(component_id);
        SqliteSink {
				component_id,
            file_path,
            table,
            cx: None,
            columns: Vec::new(),
            tx,
            created_utc,
            started_utc,
            metrics,
        }
    }
}

impl Sink for SqliteSink {
	fn kind(&self) -> SinkKind { SinkKind::Sqlite }

//    #[instrument]
	fn initialize(&mut self, _cfg: &SinkSettings) -> Result<(), Error> {
		
        // 1) Open (or create) the SQLite database file
        let cx = Connection::open(&self.file_path).map_err(|e| {Error::from(e)})?;
           // .map_err(|e| Error::Io { source: IoErrorWrapper::from(e.into()) })?;
        // Turn on foreign keys, etc., if desired
        cx.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| Error::General(format!("Failed to enable PRAGMA: {}", e)))?;
        self.cx = Some(cx);
        Ok(())
    }

    fn accept(&mut self, atom: Atom) -> Result<(), Error> {
        // If Sink not initialized yet, that's a logic error
        let cx = self.cx.as_mut().ok_or_else(|| {
            Error::General("SqliteSink.accept called before initialize".into())
        })?;

        // Skip control atoms
        if atom.atom_type() == AtomType::Control {
            return Ok(());
        }

        match atom {
            Atom::HeaderRow(header_row) => {
                // 2) Extract column names from StringRow
                //let cols: Vec<String> = header_row.into_iter().cloned().collect();
                let cols: Vec<String> = header_row.into_iter().as_slice().to_vec();
                if cols.is_empty() {
                    return Err(Error::General("Header row is empty".into()));
                }
                self.columns = cols.clone();

                // 3) Build and execute CREATE TABLE statement
                //    Columns are all TEXT type for simplicity
                let col_defs: Vec<String> = cols.iter()
                    .map(|c| format!("\"{}\" TEXT", c.replace('"', "\"\"")))
                    .collect();
                let create_sql = format!(
                    "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
                    self.table,
                    col_defs.join(", ")
                );
                cx.execute_batch(&create_sql)
                    .map_err(|e| Error::General(format!("Failed to create table: {}", e)))?;
            }

            Atom::StringRowAtom(string_row) => {
                // 4) Ensure header seen
                if self.columns.is_empty() {
                    return Err(Error::General(
                        "Received StringRowAtom before HeaderRowAtom".into(),
                    ));
                }
                let vals: Vec<String> = string_row.into_iter().collect();
                if vals.len() != self.columns.len() {
                    return Err(Error::General(format!(
                        "Row has {} columns but header has {}",
                        vals.len(),
                        self.columns.len()
                    )));
                }

                // 5) Build INSERT statement with ? placeholders
                let placeholders = vec!["?"; vals.len()].join(", ");
                let insert_sql = format!(
                    "INSERT INTO \"{}\" ({}) VALUES ({})",
                    self.table,
                    self.columns.iter()
                        .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
                        .collect::<Vec<_>>()
                        .join(", "),
                    placeholders
                );
                let mut stmt = cx.prepare(&insert_sql)
                    .map_err(|e| Error::General(format!("Failed to prepare INSERT: {}", e)))?;

                // 6) Bind and execute
                stmt.execute(params_from_iter(vals.iter()))
                    .map_err(|e| Error::General(format!("Failed to insert row: {}", e)))?;
            }

            Atom::ByteRowAtom(byte_row) => {
                // Similar to StringRow, but convert bytes to String (UTF-8 lossy)
                if self.columns.is_empty() {
                    return Err(Error::General(
                        "Received ByteRowAtom before HeaderRowAtom".into(),
                    ));
                }
                let mut vals: Vec<String> = Vec::new();
                for b in byte_row.into_iter() {
                    let s = String::from_utf8_lossy(b).into_owned();
                    vals.push(s);
                }
                if vals.len() != self.columns.len() {
                    return Err(Error::General(format!(
                        "Row has {} columns but header has {}",
                        vals.len(),
                        self.columns.len()
                    )));
                }

                // Build and execute same INSERT as above
                let placeholders = vec!["?"; vals.len()].join(", ");
                let insert_sql = format!(
                    "INSERT INTO \"{}\" ({}) VALUES ({})",
                    self.table,
                    self.columns.iter()
                        .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
                        .collect::<Vec<_>>()
                        .join(", "),
                    placeholders
                );
                let mut stmt = cx.prepare(&insert_sql)
                    .map_err(|e| Error::General(format!("Failed to prepare INSERT: {}", e)))?;
                stmt.execute(params_from_iter(vals.iter()))
                    .map_err(|e| Error::General(format!("Failed to insert row: {}", e)))?;
            }

            _ => {
                // Other atom types are ignored
            }
        }

        Ok(())
    }

    fn close(&mut self) {
        // 7) Finalize by closing the connection (drop it)
        if let Some(cx) = self.cx.take() {
            match cx.close() {
            	Ok(_)         => info!("Successfully closed the connection."),
            	Err((_, err)) => warn!("Error closing db: {}", err),
            }
        } else {
				warn!("Close called but no active SqlLite connection");
        }
    }
}

impl ProvidesMetrics for SqliteSink {
    fn metrics(&self) -> ComponentMetrics {
    	self.metrics.clone()
    }
	fn take_metrics(&mut self) -> ComponentMetrics {
		let rv = self.metrics.clone();
		self.metrics.reset();
		rv
	}
}
