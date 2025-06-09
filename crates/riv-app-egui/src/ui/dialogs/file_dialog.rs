

use apex::AppState;

pub fn choose_file_with_native_dialog(state: &mut AppState) {
	if let Some(path) = rfd::FileDialog::new()
		.add_filter("Data files", &["csv", "json"])
		.pick_file() {
			state.set_source_path(path);
		}
}