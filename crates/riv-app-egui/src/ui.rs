
pub mod dialogs;
pub mod menu;
pub mod regions;
pub mod widgets;
pub mod visuals;

use egui::TextureHandle;
use crate::ui::regions::ActiveAction;
use std::fmt;

#[inline]
#[allow(dead_code)]
pub fn standard_button_width() -> f32 { 120.0 }




/// UI-level state that lives alongside `AppState` / model state.
#[derive(Clone, PartialEq, Eq)]
pub struct UiState {
    active_panel: ActiveAction,
    pub about_dialog_visible: bool,  /// 
    about_dialog_texture: Option<TextureHandle>,
}

impl Default for UiState {
    fn default() -> Self {
		let active_panel         = ActiveAction::default();
		let about_dialog_visible = false;
		let about_dialog_texture = None;
		Self {active_panel, about_dialog_visible, about_dialog_texture}
	}
}

impl fmt::Debug for UiState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("active_panel", &self.active_panel)
            .field("about_dialog_visible", &self.about_dialog_visible)
            // show `true / false` instead of trying to print the GPU texture
            .field("has_texture", &self.about_dialog_texture.is_some())
            .finish()
    }
}

impl UiState {
    /* ───────────── active_panel ───────────── */

    #[inline]
    pub fn active_panel(&self) -> ActiveAction {
        self.active_panel
    }

    #[inline]
    pub fn set_active_panel(&mut self, panel: ActiveAction) {
        self.active_panel = panel;
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

    #[inline]
    pub fn hide_about_dialog(&mut self) {
        self.about_dialog_visible = false;
    }

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