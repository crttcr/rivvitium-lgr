use egui::{Frame, Ui};
use egui_extras::{Column, TableBuilder};
use zero::component::telemetry::component_metrics::ComponentMetrics;
use crate::ui::helpers;
use crate::ui::visuals::banners::caption_banner;


pub fn draw_sink_detail_view(ui: &mut Ui, dto: ComponentMetrics) {
    let status_text = helpers::status_as_rich_text(dto.status);
    let message_rate = 234;  // dto.messages_per_second()
	ui.vertical(|ui| {
   	caption_banner(ui, "Sink information");
 		ui.add_space(6.0);
	
    Frame::group(ui.style()).show(ui, |ui| {
        ui.push_id("sink_view", |ui| {
            /* ------------- detail table -------------------------------- */
            TableBuilder::new(ui)
                .column(Column::auto())        // label column
                .column(Column::remainder())   // value column
                .body(|mut body| {
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("Output …");});
                        row.col(|ui| helpers::right_label(ui, "FIX"));
                    });
                	  helpers::row_u32("Duration (ms)", dto.duration.subsec_millis(), &mut body);
                    helpers::row_u64("Anything else", message_rate,                 &mut body);
                    helpers::row_u64("Data rows",     dto.record_count,             &mut body);
                    helpers::row_u64("Errors",        dto.error_count,              &mut body);
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("Status");});
                        row.col(|ui| {ui.label(status_text);});
                    });
                });
        });
    });
	});
}
