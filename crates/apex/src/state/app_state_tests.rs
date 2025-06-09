
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
 fn default_state_is_empty() {
	  let s = AppState::default();
	  assert_eq!(s.click_count(), 0);
	  assert!(!s.has_selected_file());
	  assert!(!s.can_run_pipeline());
 }

 #[test]
 fn capture_click_increments_count() {
	  let mut state = AppState::default();
		state.capture_click();
	  assert_eq!(state.click_count(), 1);
		state.capture_click();
	  assert_eq!(state.click_count(), 2);
 }

 #[test]
 fn with_source_accepts_readable_file() {
	  let file = make_temp_file();
	  let s    = AppState::default().with_source(file.clone());
	  assert!(s.has_selected_file());
 }

 #[test]
 fn with_source_rejects_unreadable_file() {
	  let bogus = bogus_path();

	  let s = AppState::default();
	  let s_after = s.with_source(bogus);

	  // Because the path is unreadable, the struct should be unchanged.
	  assert!(!s_after.has_selected_file());
	  assert_eq!(s_after.click_count(), 0);
 }