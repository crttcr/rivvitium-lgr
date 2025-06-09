
use tracing::{info, instrument, warn};
use zed::util::file_utils::assert_readable;
use std::path::PathBuf;

pub struct AppState {
	click_count:   u32,
	selected_file: Option<PathBuf>,   // ① store the path
}

impl AppState {
	// Functional mutation
	//
	pub fn with_source(self, selected_file: PathBuf) -> Self {
		match assert_readable(&selected_file) { 
			Ok(_)  => Self {selected_file : Some(selected_file),..self},
			Err(e) => {
				  warn!("⚠ Bad file. Not updating source: {}: {}", selected_file.display(), e);
				  self
			}
		 }
	}
	
	pub fn set_source_path(&mut self, selected_file: PathBuf) {
		match assert_readable(&selected_file) { 
			Ok(_)  => {
				info!("Updating source: {}", selected_file.display());
				self.selected_file = Some(selected_file)
			},
			Err(e) => {
				  warn!("⚠ Bad file. Not updating source: {}: {}", selected_file.display(), e);
			}
		 }
	}
	
	#[instrument(skip(self))]	
	pub fn teardown(&mut self) {
		self.selected_file = None;
	}
	
	pub fn close_file(&mut self) {
		match &self.selected_file {
		Some(path) => {
				info!("Closing file: {}", path.display());
				self.selected_file = None;
			},
			None => {
				warn!("No file to close");
			}
		}
	}
	
	pub fn click_count(&self) -> u32 { self.click_count }
	pub fn reset_clicks(&mut self) {
			self.click_count = 0
	}
	
	pub fn capture_click(& mut self) {
			self.click_count += 1;
	}
	
	pub fn set_click_count(&mut self, count: u32) {
			self.click_count += count;
	}
	
	
	// Predicates
	//
	pub fn has_selected_file(&self) -> bool { self.selected_file.is_some() }
	pub fn can_run_pipeline(&self)  -> bool { self.has_selected_file()     }
}

impl Default for AppState {
	fn default() -> Self {
		let click_count   = 0;
		let selected_file = None;
		Self{click_count, selected_file}
	}
}

/*
fn pick_file(&mut self, ctx: &egui::Context) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Choose input")
        .pick_file()
    {
        self.selected_file = Some(path);
    }
}

fn process(&mut self) -> anyhow::Result<()> {
    let path = self.selected_file.as_ref().ok_or_else(|| anyhow!("no file"))?;
    let mut reader = std::fs::File::open(path)?;
    // … read / parse …
    Ok(())
}
*/