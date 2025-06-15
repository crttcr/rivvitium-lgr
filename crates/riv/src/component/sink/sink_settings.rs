use std::path::PathBuf;
use std::sync::mpsc::Sender;
use zero::component::telemetry::component_metrics::ComponentMetrics;
use crate::component::sink::capture_sink::CaptureSink;
use crate::component::sink::{Sink, SinkKind};
use crate::component::sink::console_sink::ConsoleSink;
use crate::component::sink::csv_sink::CsvSink;
use crate::component::sink::dev_null_sink::DevNullSink;
use crate::component::sink::json_sink::JsonSink;
use crate::component::sink::kafka_sink::KafkaSink;
use crate::component::sink::sql_server_sink::SqlServerSink;
use crate::component::sink::sqlite_sink::SqliteSink;
use crate::Error;

/// One strongly typed configuration value covering every supported sink.
/// Variants with no additional settings (Capture, Console, DevNull)
/// are bare, while those that need parameters carry them inline.
/// 
#[derive(Debug, Default, Clone, PartialEq)]
pub enum SinkSettings {
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
    #[default]
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
        topic:  String,
    },

    /// Stream records to a PubSub topic on a given server:port.
    PubSub {
        server: String,
        port:   u16,
        topic:  String,
    },    
    
    /// Persists to SQLite table.
    Sqlite {
        db_path: PathBuf,
        table:   String,
    },
    
    /// Persists to Sql Server database.
    SqlServer {
        server:    String,
        port:      u16,
        user_name: String,
        password:  String,
        db_name:   String,
    },
}

/* ───────────────── convenience constructors ──────────────────── */
impl SinkSettings {
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
    
    pub fn kafka(server: impl Into<String>, port: u16, topic: impl Into<String>) -> Self {
        Self::Kafka {
            server: server.into(),
            port,
            topic:   topic.into(),
        }
    }
    
    pub fn pubsub(server: impl Into<String>, port: u16, topic: impl Into<String>) -> Self {
        Self::PubSub {
            server: server.into(),
            port,
            topic:   topic.into(),
        }
    }
    
    pub fn sqlite<P: Into<PathBuf>>(db_path: P, table: impl Into<String>) -> Self {
        Self::Sqlite {
            db_path: db_path.into(),
            table:   table.into(),
        }
    }

    pub fn sqlserver(
    	server:      impl Into<String>,
    	port:        u16,
    	user_name:   impl Into<String>,
    	password:    impl Into<String>,
    	db_name:     impl Into<String>,
    	) -> Self {
        Self::SqlServer {
            server:     server.into(),
            port:       port,
            user_name:  user_name.into(),
            password:   password.into(),
            db_name:    db_name.into(),       
        }
    }
}

/* ---------------------------------------------------------------

        server:    String,
        port:      u16,
        user_name: String,
        password:  String,
        db_name:   String,
    },
   Build a boxed sink **from config + sender**
---------------------------------------------------------------- */
impl SinkSettings {
    /// Convert the enum + a telemetry channel into a boxed `Sink`.
    pub fn build_sink(
        &self,
        component_id: u32,
        tx:           Sender<ComponentMetrics>,
    ) -> Result<Box<dyn Sink>, Error> {
        match self {
            SinkSettings::Capture => Ok(Box::new(CaptureSink::new(component_id, tx))),
            SinkSettings::Console => Ok(Box::new(ConsoleSink::new(component_id, tx))),
            SinkSettings::DevNull => Ok(Box::new(DevNullSink::new(component_id, tx))),
            SinkSettings::Csv{file_path, delimiter} => {
            	let file_path = file_path.clone();
            	let sink      = CsvSink::new(component_id, file_path, *delimiter, tx);
            	let sink      = Box::new(sink);
					Ok(sink)
            }

			SinkSettings::Json{file_path, pretty}    => {
            	let file_path = file_path.clone();
            	let sink      = JsonSink::new(component_id, file_path, *pretty, tx);
            	let sink      = Box::new(sink);
					Ok(sink)
            },

			SinkSettings::Kafka { server, port, topic } => {
            	let server = server.clone();
            	let topic  = topic.clone();
            	let sink   = KafkaSink::new(component_id, server, *port, topic, tx);
            	let sink   = Box::new(sink);
					Ok(sink)
            }
            
			SinkSettings::PubSub { server, port, topic } => {
            	let server = server.clone();
            	let topic  = topic.clone();
            	let sink   = KafkaSink::new(component_id, server, *port, topic, tx);
            	let sink   = Box::new(sink);
					Ok(sink)
            }

            SinkSettings::Sqlite {db_path, table} => {
	            let file_path = db_path.clone();
	            let table     = table.clone();
            	let sink   = SqliteSink::new(component_id, file_path, table, tx);
            	let sink   = Box::new(sink);
					Ok(sink)
            }
            
            SinkSettings::SqlServer {server, port, user_name, password, db_name} => {
	            let server    = server.clone();
	            let user_name = user_name.clone();
	            let password  = password.clone();
	            let db_name   = db_name.clone();
            	let sink      = SqlServerSink::new(component_id, server, *port, 
            		user_name, password, db_name, tx);
            	let sink      = Box::new(sink);
					Ok(sink)
            }
        }
    }
}

/* ───────────────── helpers ──────────────────── */
impl SinkSettings {
    /// Handy accessor when you only need to know the *kind*.
    pub fn kind(&self) -> SinkKind {
        match self {
            Self::Capture           => SinkKind::Capture,
            Self::Console           => SinkKind::Console,
            Self::Csv       { .. }  => SinkKind::Csv,
            Self::DevNull           => SinkKind::DevNull,
            Self::Json      { .. }  => SinkKind::Json,
            Self::Kafka     { .. }  => SinkKind::Kafka,
            Self::PubSub    { .. }  => SinkKind::Kafka,
            Self::Sqlite    { .. }  => SinkKind::Sqlite,
            Self::SqlServer { .. }  => SinkKind::Kafka,
        }
    }

    /// Handy accessor when you only need to know the *kind*.
    pub fn can_publish(&self) -> bool {
        match self {
            Self::Capture           => false,
            Self::Console           => false,
            Self::DevNull           => false,
            Self::Json      { .. }  => false,     // false until implemented
            Self::Kafka     { .. }  => false,     // false until implemented
            Self::PubSub    { .. }  => false,     // false until implemented
            Self::SqlServer { .. }  => false,     // false until implemented
            _                       => true,       
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
