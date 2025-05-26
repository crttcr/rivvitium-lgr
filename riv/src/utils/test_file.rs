
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// A short‐lived test file: writes `content` to a temp file
/// and deletes it on drop.
/// 
// `NamedTempFile` already implements `Drop` to clean up, so
// you don't need an explicit Drop impl here.
//
pub struct TestFile {
	inner: NamedTempFile,
}

impl TestFile {
	/// Create a new temp file containing `content`.
	pub fn with_content(content: &str) -> io::Result<Self> {
		let mut inner = NamedTempFile::new()?;
		inner.write_all(content.as_bytes())?;
		inner.flush()?;
		Ok(TestFile{inner})
	}

	/// The filesystem path to the temp file
	pub fn path(&self) -> &Path {
		self.inner.path()
	}
	
	/// The filesystem path to the temp file
	pub fn path_str(&self) -> &str {
		self.path().to_str().unwrap_or_else(|| "")
	}
	
	pub fn path_string(&self) -> String {
		self.inner.path().to_string_lossy().to_string()
	}

	/// If you really need to take ownership of the path (and
	/// prevent automatic cleanup), use this method.
	pub fn into_path(self) -> PathBuf {
		self.inner.into_temp_path().to_path_buf()
	}
}
