

use apex::AppState;
use crate::ui::regions::ActiveAction;
use crate::ui::UiState;

pub fn choose_file_with_native_dialog(state: &mut AppState, ui: &mut UiState) {
	if let Some(path) = rfd::FileDialog::new()
		.add_filter("Data files", &["csv", "json"])
		.pick_file() {
			state.set_source_path(path);
			ui.set_active_panel(ActiveAction::DataFileOnly);
		}
}
