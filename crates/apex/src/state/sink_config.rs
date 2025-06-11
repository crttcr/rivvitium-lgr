use std::path::PathBuf;
use riv::component::sink::SinkKind;

/// One strongly typed configuration value covering every supported sink.
/// Variants with no additional settings (Capture, Console, DevNull)
/// are bare, while those that need parameters carry them inline.
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum SinkConfig {
    /// Retains the data in memory (buffer is internal to the sink).
    Capture,

    /// Prints every record to stdout (for debugging).
    Console,

    /// Writes a CSV file.
    Csv {
        file_path: PathBuf,
        delimiter: char,
    },

    /// Discards all data.
    DevNull,

    /// Emits a JSON file (optionally pretty-printed).
    Json {
        file_path: PathBuf,
        pretty: bool,
    },

    /// Stream records to a Kafka topic on a given server:port.
    Kafka {
        server: String,
        port:   u16,
    },
    
    /// Persists to SQLite table.
    Sqlite {
        db_path: PathBuf,
        table:   String,
    },
}

/* ───────────────── convenience ctors & helpers ──────────────────── */

impl SinkConfig {
    pub fn capture()    -> Self { Self::Capture }
    pub fn console()    -> Self { Self::Console }
    pub fn dev_null()   -> Self { Self::DevNull }

    pub fn csv<P: Into<PathBuf>>(path: P, delimiter: char) -> Self {
        Self::Csv {
            file_path: path.into(),
            delimiter,
        }
    }

    pub fn json<P: Into<PathBuf>>(path: P, pretty: bool) -> Self {
        Self::Json {
            file_path: path.into(),
            pretty,
        }
    }
    
    pub fn kafka(server: impl Into<String>, port: u16) -> Self {
        Self::Kafka {
            server: server.into(),
            port,
        }
    }
    
    pub fn sqlite<P: Into<PathBuf>>(path: P, table: impl Into<String>) -> Self {
        Self::Sqlite {
            db_path: path.into(),
            table:   table.into(),
        }
    }

    /// Handy accessor when you only need to know the *kind*.
    pub fn kind(&self) -> SinkKind {
        match self {
            Self::Capture         => SinkKind::Capture,
            Self::Console         => SinkKind::Console,
            Self::Csv     { .. }  => SinkKind::Csv,
            Self::DevNull         => SinkKind::DevNull,
            Self::Json    { .. }  => SinkKind::Json,
            Self::Kafka   { .. }  => SinkKind::Kafka,
            Self::Sqlite  { .. }  => SinkKind::Sqlite,
        }
    }

    /// Optional helper: where will this sink write its bytes?
    /// (Returns `None` for Capture / Console / DevNull.)
    pub fn dest_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Csv    { file_path, .. } => Some(file_path),
            Self::Json   { file_path, .. } => Some(file_path),
            Self::Sqlite { db_path,  .. }  => Some(db_path),
            _ => None,
        }
    }
}

impl Default for SinkConfig {
    fn default() -> Self {
        SinkConfig::DevNull
    }
}
