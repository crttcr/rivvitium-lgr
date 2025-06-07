
use crate::app::rivvitium_app::RivvitiumApp;

pub fn open_about_dialog(app: &mut RivvitiumApp, ctx: &egui::Context) {
	egui::Window::new("About Rivvitium")
		 .open(&mut app.show_dialog) // adds the little **×** close button
		 .resizable(true)
		 .collapsible(false)
		 .show(ctx, |ui| {
			  if let Some(tex) = &app.image_about {
					let size  = tex.size_vec2();
					let small = size * 0.25; // scale to reasonable size
					ui.image((tex.id(), small));
			  } else {
					ui.label("Loading image …");
			  }
		 });
}	
