use riv::component::source::csv_byte_source::CsvByteSource;
use riv::component::source::csv_string_source::CsvStringSource;
use riv::component::source::Source;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use tracing::info;

/// Errors that can occur while choosing the right `Source` implementor.
#[derive(thiserror::Error, Debug)]
pub enum SourceError {
    #[error("unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("cannot guess JSON kind (array vs object)")]
    UnknownJson,
}

/// Decide which concrete `Source` to use based on the file extension,
/// construct it with `new(path: String)`, and return it as a boxed trait obj.
///
/// * `.csv`   → `CsvByteSource` or `CsvStringSource`
/// * `.json`  → first non-WS byte: `[` → `JsonArraySource`, `{` → `JsonObjectSource`
///
pub fn open_source(path: &PathBuf) -> Result<Box<dyn Source>, SourceError> {
    // Convert the path to a String once (used by all `new()` calls).
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .ok_or_else(|| SourceError::UnsupportedExtension("none".into()))?;

    let path_str = path
        .to_str()
        .ok_or_else(|| SourceError::UnsupportedExtension("invalid UTF-8 path".into()))?
        .to_owned();

    match ext.as_str() {
        "json" => open_json_source(path),
        "csvs" => Ok(Box::new(CsvStringSource::new(path_str))),
        "csv"  => {
    			let file       = File::open(path)?;
            let csv_source = CsvByteSource::new(file);
            Ok(Box::new(csv_source))
        }
        other => Err(SourceError::UnsupportedExtension(other.into())),
    }
}

fn open_json_source(path: &PathBuf) -> Result<Box<dyn Source>, SourceError> {
    info!(
        "Ignoring JSON file until JSON sources are implemented: {:?}",
        path
    );
    Err(SourceError::UnknownJson)
    /*
    let path_str = path
        .to_str()
        .ok_or_else(|| SourceError::UnsupportedExtension("invalid UTF-8 path".into()))?
        .to_owned();

	let mut file   = File::open(path)?;
    let mut buf  = [0u8; 256];
    let n        = file.read(&mut buf)?;
    let first    = buf[..n]
        .iter()
        .map(|b| *b as char)
        .find(|c| !c.is_whitespace());

        match first {
            Some('[') => Ok(Box::new(JsonArraySource::new(path_str))),
            Some('{') => Ok(Box::new(JsonObjectSource::new(path_str))),
            _         => Err(SourceError::UnknownJson),
            }
    */
}
