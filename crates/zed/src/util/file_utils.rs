use std::io;
use std::fs::File;
use std::path::Path;

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
