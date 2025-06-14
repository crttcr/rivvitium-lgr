
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::path::PathBuf;
use crate::component::source::SourceConfig;

pub struct PathBufConfig {
	pub path: PathBuf,	
}

impl PathBufConfig {
	pub fn new(path: PathBuf) -> Self {Self{path}}
}

impl SourceConfig for PathBufConfig {
	fn path_buf(&self)                    -> Option<&PathBuf> { Some(&self.path)}
	fn string_value(&self,   _name: &str) -> Option<String>   { None }
	fn integer_value(&self, _name: &str)  -> Option<i32>      { None }
	fn float_value(&self,   _name: &str)  -> Option<f32>      { None }
	fn bool_value(&self,    _name: &str)  -> Option<bool>     { None }
}

/* ───────── Display & Debug ─────────────────────────────────── */

impl Display for PathBufConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `PathBuf::display()` gives a Display wrapper that prints correctly
        write!(f, "{}", self.path.display())
    }
}

impl Debug for PathBufConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathBufConfig")
            .field("path", &self.path)
            .finish()
    }
}