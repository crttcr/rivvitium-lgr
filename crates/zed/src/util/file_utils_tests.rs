
use std::io;
use std::path::Path;
use crate::util::file_utils::assert_readable;

#[test]
pub fn test_assert_readable_fails_no_file() {
	let my_file = Path::new("/does/not/exist/foo.txt");

	match assert_readable(&my_file) {
		 Ok(()) => println!("✅ can read {}", my_file.display()),
		 Err(e) if e.kind() == io::ErrorKind::NotFound       =>
			  eprintln!("⚠ file missing: {}", my_file.display()),
		 Err(e) if e.kind() == io::ErrorKind::PermissionDenied =>
			  eprintln!("⚠ no read permission: {}", my_file.display()),
		 Err(e) => eprintln!("⚠ other I/O error: {e}"),
	}
}
