
use apex::AppState;
use crate::ui::regions::ApplicationStatus;
use crate::ui::UiState;

pub fn choose_file_with_native_dialog(state: &mut AppState, ui: &mut UiState) {
	if let Some(path) = rfd::FileDialog::new()
		.add_filter("Data files", &["csv", "json"])
		.pick_file() {
			state.set_source_path(path);
			if state.can_parse() {
				ui.set_application_status(ApplicationStatus::Idle);
			} else {
				ui.set_application_status(ApplicationStatus::NotConfigured);
			}
		}
}
