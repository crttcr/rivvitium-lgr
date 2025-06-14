use std::io;
use std::io::Write;
use std::fs::File;
use std::path::{Path, PathBuf};


/// Returns `Ok(())` if the file exists *and* the current process can open it
/// for reading; otherwise returns the original `io::Error`.
///
pub fn assert_readable(path: &Path) -> io::Result<()> {
	// `File::open` simultaneously answers both questions:
	//  * `Err(e) if e.kind() == NotFound`         → it doesn’t exist
	//  * `Err(e) if e.kind() == PermissionDenied` → not readable
	//  * other kinds for I/O errors (e.g. broken symlink, bad device, etc.)
	File::open(path).map(|_| ())          // drop handle immediately
}

/// Creates the named file in a temporary directory
/// 
/// If the file happens to exist, it is truncated
/// 
pub fn make_temp_file_named(name: &str) -> PathBuf {
	let mut path = std::env::temp_dir();
	path.push(name);
	File::create(&path).expect("create temp file");
	path
}

/// Creates the named file in a temporary directory, and 
/// writes the provided content into the file
/// 
/// If the file happens to exist, it is truncated then written
/// 
pub fn make_temp_file_with_content(name: &str, content: &str) -> PathBuf {
	let mut path = std::env::temp_dir();
	path.push(name);
	let mut f = File::create(&path).expect("create temp file");
	write!(f, "{}", content).expect("write to temp file");
	f.flush().expect("flush temp file");
	path
}


/// Path that (almost certainly) does *not* exist.
/// 
pub fn bogus_path() -> PathBuf {
	let mut path = std::env::temp_dir();
	path.push("obviously__does__not__exist.xyz");
	path
}

