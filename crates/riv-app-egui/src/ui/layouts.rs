
use std::time::{Duration, Instant};
use apex::state::parse_detail_dto::ParseDetailDTO;
use crate::app::rivvitium_app::RivvitiumApp;
use crate::ui::regions::button_bar::draw_button_bar;
use crate::ui::views::activity_view::{activity_view, ActivityEvent};
use crate::ui::views::data_view::data_view;
use crate::ui::views::sink_detail_view::draw_sink_detail_view;
use crate::ui::views::source_detail_view::show_source_detail_view;

pub fn draw_main_screen(app: &mut RivvitiumApp, ctx: &egui::Context) {

	// FIXME: plumb this with real data ...
	let dto: ParseDetailDTO = ParseDetailDTO::new("boo");
	let dummy = vec![
       ActivityEvent { time: Instant::now() - Duration::from_secs(3605), label: "Rivvitium startup".into() },
       ActivityEvent { time: Instant::now() - Duration::from_secs(  45), label: "Selected data file ".into() },
       ActivityEvent { time: Instant::now() - Duration::from_secs(   3), label: "Built pipeline".into() },
   ];

	
    //           ────────── whole client area ──────────
    egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
        /* --------------- row 0 --------------- */
            draw_button_bar(app, ui);
        /* --------------- row 1 --------------- */
        ui.horizontal(|ui| {
            let height = ui.available_height();
            let width  = ui.available_width() * 0.5;          // equal-width split for source & sink
            let size   = egui::vec2(width, height);
            ui.allocate_ui(size, |ui| {
                show_source_detail_view(ui, &dto);
            });
            ui.allocate_ui(size, |ui| {
                draw_sink_detail_view(ui, &dto);
            });
        });
        ui.separator();                         // thin line between the rows
			data_view(ui);
        ui.separator();                         // thin line between the rows
        activity_view(ui, &dummy);
    });
}