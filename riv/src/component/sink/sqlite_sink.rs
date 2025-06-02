use std::{fmt::Debug, path::Path};
use rusqlite::{params_from_iter, Connection, Error as RusqliteError};
use crate::error::{Error, IoErrorWrapper};
use tracing::instrument;
use crate::component::sink::Sink;
use crate::model::ir::atom::Atom;
use crate::model::ir::atom_type::AtomType;

/// A Sink that writes rows into a SQLite database.  
/// 
/// It expects to see a `HeaderRowAtom` first, which defines column names.  
/// Subsequent row atoms (`ByteRowAtom` or `StringRowAtom`) are inserted into the table.
pub struct SqliteSink {
    /// Filesystem path to the SQLite database file
    db_path: String,
    /// The open SQLite connection (populated in `initialize`)
    conn: Option<Connection>,
    /// Table name to use (fixed as "records")
    table: String,
    /// Column names, populated when a `HeaderRowAtom` is accepted
    columns: Vec<String>,
}

impl SqliteSink {
    pub fn new(db_path: String) -> Self {
        SqliteSink {
            db_path,
            conn: None,
            table: "records".to_string(),
            columns: Vec::new(),
        }
    }
}

impl Sink<()> for SqliteSink {
//    #[instrument]
    fn initialize<C: Debug + std::fmt::Display>(&mut self, _cfg: &C) -> Result<(), Error> {
        // 1) Open (or create) the SQLite database file
        let conn = Connection::open(&self.db_path).map_err(|e| {Error::from(e)})?;
           // .map_err(|e| Error::Io { source: IoErrorWrapper::from(e.into()) })?;
        // Turn on foreign keys, etc., if desired
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| Error::General(format!("Failed to enable PRAGMA: {}", e)))?;
        self.conn = Some(conn);
        Ok(())
    }

    fn accept(&mut self, atom: Atom) -> Result<(), Error> {
        // If Sink not initialized yet, that's a logic error
        let conn = self.conn.as_mut().ok_or_else(|| {
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
                conn.execute_batch(&create_sql)
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
                let mut stmt = conn.prepare(&insert_sql)
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
                let mut stmt = conn.prepare(&insert_sql)
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

    #[instrument(skip(self), fields(self = "SqliteSink", db_path=%self.db_path))]
    fn finish(&mut self) -> Result<(), Error> {
        // 7) Finalize by closing the connection (drop it)
        if let Some(conn) = self.conn.take() {
            conn.close()
                .map_err(|(_, err)| Error::General(format!("Error closing DB: {}", err)))?;
            Ok(())
        } else {
            Err(Error::General(
                "finish called but no active SQLite connection".into(),
            ))
        }
    }
}
