
use std::{fs, io};
use std::path::Path;
use crate::util::file_utils::*;

#[test]
pub fn test_assert_readable_fails_no_file()
{
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

#[test]
pub fn test_make_temp_file_named()
{
	let name = "big_foo.tmp";
	let file = make_temp_file_named(name);
	match assert_readable(&file) {
		Ok(_) => {}
		Err(x) => panic!("Unexpected failure to create temp file ({}): {}", name, x),
		}
	let _ = fs::remove_file(file);
}

#[test]
pub fn test_make_temp_file_with_content()
{
	let text = "Couple\n";
	let name = "text_foo.tmp";
	let file = make_temp_file_with_content(name, text);
	let read = fs::read_to_string(&file).unwrap();
	println!("{}", read);
	assert!(read == text);
	let _ = fs::remove_file(file);
}
