// build.rs
// Compiles slint files into Rust code
//
//   let entry_file = "ui/gallery.slint";               // Path to main .slint file, relative to the project root
//
fn main()
{
	// This tells Cargo to re-run the build script if build.rs itself changes. Good practice.
   println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=ui/app_window.slint");
	println!("cargo:rerun-if-changed=ui/widgets/dialogs/about_dialog.slint");

//	slint_build::compile("ui/widgets/dialogs/about_dialog.slint").unwrap();
	slint_build::compile("ui/app_window.slint").unwrap();
}
