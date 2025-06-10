
pub fn draw_footer(ui: &mut egui::Ui) {
	ui.separator();
	ui.horizontal(|ui| {
//		ui.label("© 2025 Rivvitium Labs");
		ui.label("© 2025 Rivvit Inc");
		ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
			ui.label("Status: idle");
			});
		});
}
