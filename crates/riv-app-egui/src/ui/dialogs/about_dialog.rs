
use crate::app::rivvitium_app::RivvitiumApp;

pub fn open_about_dialog(app: &mut RivvitiumApp, ctx: &egui::Context) {
	let mut open_flag = app.ui_state.about_dialog_visible;
	egui::Window::new("About Rivvitium")
		 .open(&mut open_flag)
		 //.open(&mut app.ui_state.about_dialog_visible) // adds the little **×** close button
		 .resizable(true)
		 .collapsible(false)
		 .show(ctx, |ui| {
			  if let Some(tex) = &app.ui_state.about_dialog_texture() {
					let size  = tex.size_vec2();
					let small = size * 0.25; // scale to a reasonable size
					ui.image((tex.id(), small));
			  } else {
					ui.label("Loading image …");
			  }
		 });
	app.ui_state.set_about_dialog_visible(open_flag);
	if !open_flag {app.ui_state.clear_about_dialog_texture()}
}	
