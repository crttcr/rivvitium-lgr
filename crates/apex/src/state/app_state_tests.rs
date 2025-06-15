
use crate::state::app_state::AppState;

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
