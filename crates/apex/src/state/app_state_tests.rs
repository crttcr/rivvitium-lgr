
use crate::state::app_state::AppState;

use std::{fs::File, io::Write, path::PathBuf};

// ---------- helpers ----------------------------------------------------

 /// Create a small, definitely-readable temp file and give back its `PathBuf`.
 fn make_temp_file() -> PathBuf {
	  let mut path = std::env::temp_dir();
	  path.push("source_state_test.tmp");
	  // If the file happens to exist from a previous run, truncate it:
	  let mut f = File::create(&path).expect("create temp file");
	  writeln!(f, "dummy").unwrap();
	  path
 }

 /// Path that (almost certainly) does *not* exist.
 fn bogus_path() -> PathBuf {
	  let mut path = std::env::temp_dir();
	  path.push("obviously__does__not__exist.xyz");
	  path
 }

 // ---------- tests -------------------------------------------------------

 #[test]
 fn default_state_cannot_run_any_action() {
	  let (sender, _) = std::sync::mpsc::channel();
	  let s = AppState::new(sender);
	  assert!(!s.can_parse());
	  assert!(!s.can_analyze());
	  assert!(!s.can_blueprint());
	  assert!(!s.can_publish());
 }
