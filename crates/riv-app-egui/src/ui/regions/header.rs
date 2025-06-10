
pub fn draw_header(ui: &mut egui::Ui) {
	ui.horizontal_centered(|ui| {
		ui.heading("Rivvitium");
		ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
			if ui.button("Sync").clicked() { /* … */ }
			ui.separator();
			ui.label("v0.3.0");
	      });	
		});	
}
