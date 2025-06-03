// build.rs
// Compiles slint files into Rust code
//
fn main()
{
   // This tells Cargo to re-run the build script if build.rs itself changes. Good practice.
   println!("cargo:rerun-if-changed=build.rs");

   // This is the path to your main .slint file, relative to the project root
   let entry_file = "ui/app_window.slint";
//   let entry_file = "ui/temp_test.slint";

   // Call the Slint build function
   let result     = slint_build::compile(entry_file);

   // Handle the result of the compilation
   match result {
     Ok(_)  => {
         // Compilation successful, nothing to do here as slint_build
         // handles setting environment variables for the main build.
     },
     Err(e) => {
       // If compilation fails, print the error to stderr and panic.
       // This will cause the Cargo build to fail and show the error.
       eprintln!("Slint build failed: {}", e);
       panic!("Slint build error: {}", e);
       },
   }
}
