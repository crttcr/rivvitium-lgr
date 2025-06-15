
pub mod dialogs;
pub mod helpers;
pub mod layouts;
pub mod menu;
pub mod regions;
pub mod widgets;
pub mod views;
pub mod visuals;

use egui::TextureHandle;
use crate::ui::regions::ApplicationStatus;
use std::fmt;
use crate::ui::dialogs::sink_dialog::SinkDialog;

#[inline]
#[allow(dead_code)]
pub fn standard_button_width() -> f32 { 120.0 }


/// UI-level state that lives alongside `AppState` / model state.
//#[derive(Clone, PartialEq)]
pub struct UiState {
	status:                    ApplicationStatus,
	pub about_dialog_visible:  bool,  ///
	pub sink_dialog:           SinkDialog,  ///
	about_dialog_texture:      Option<TextureHandle>,
}

impl Default for UiState {
    fn default() -> Self {
		let status                  = ApplicationStatus::default();
		let about_dialog_visible    = false;
		let about_dialog_texture    = None;
		let sink_dialog             = SinkDialog::default();
		Self {
			status,
			about_dialog_visible, 
			sink_dialog, 
			about_dialog_texture
			}
	}
}

impl fmt::Debug for UiState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("active_panel", &self.status)
            .field("about_dialog_visible", &self.about_dialog_visible)
            // show `true / false` instead of trying to print the GPU texture
            .field("has_texture", &self.about_dialog_texture.is_some())
            .finish()
    }
}

impl UiState {
    #[inline]
    pub fn set_application_status(&mut self, panel: ApplicationStatus) {
        self.status = panel;
    }

    /* ──────── about_dialog_visible ────────── */

    #[inline]
    pub fn is_sink_dialog_visible(&self) -> bool {
        self.sink_dialog.open
    }

    #[inline]
    pub fn set_sink_dialog_invisible(&mut self) {
    	println!("set_sink_dialog_invisible");
        self.sink_dialog.open = false;
    }

    #[inline]
    pub fn set_sink_dialog_visible(&mut self) {
        self.sink_dialog.open = true;
    }

    /* ──────── about_dialog_visible ────────── */

    #[inline]
    pub fn is_about_dialog_visible(&self) -> bool {
        self.about_dialog_visible
    }

    #[inline]
    pub fn set_about_dialog_visible(&mut self, visible: bool) {
        self.about_dialog_visible = visible;
    }

    #[inline]
    pub fn show_about_dialog(&mut self) {
        self.about_dialog_visible = true;
    }

//    #[inline]
//    pub fn hide_about_dialog(&mut self) {
//        self.about_dialog_visible = false;
//    }

    /* ─────────── about_dialog_texture ─────── */

    #[inline]
    pub fn has_about_dialog_texture(&self) -> bool {
        self.about_dialog_texture.is_some()
    }

    #[inline]
    pub fn about_dialog_texture(&self) -> Option<&TextureHandle> {
        self.about_dialog_texture.as_ref()
    }

    /// Replace the cached dialog texture (e.g. after `ctx.load_texture`).
    #[inline]
    pub fn set_about_dialog_texture(&mut self, tex: TextureHandle) {
        self.about_dialog_texture = Some(tex);
    }

    /// Drop the cached texture to free GPU memory.
    #[inline]
    pub fn clear_about_dialog_texture(&mut self) {
        self.about_dialog_texture = None;
    }
}
