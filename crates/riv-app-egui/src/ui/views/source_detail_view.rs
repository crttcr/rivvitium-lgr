use egui::Ui;
use egui::{Frame};
use egui_extras::{Column, TableBuilder};
use zero::component::telemetry::component_metrics::ComponentMetrics;
use crate::ui::helpers;
use crate::ui::visuals::banners::caption_banner;

pub fn draw_source_detail_view(ui: &mut Ui, dto: ComponentMetrics) {
    let status_text = helpers::status_as_rich_text(dto.status);
	ui.vertical(|ui| {
   	caption_banner(ui, "Source information");
 		ui.add_space(6.0);
	
    Frame::group(ui.style()).show(ui, |ui| {
        ui.push_id("src_view", |ui| {

            /* ------------- detail table -------------------------------- */
            TableBuilder::new(ui)
                .column(Column::auto())        // label column
                .column(Column::remainder())   // value column
                .body(|mut body| {
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("File name");});
                        row.col(|ui| helpers::right_label(ui, "fixme.txt"));
                    });
                helpers::row_u64("Bytes parsed",  dto.byte_count,               &mut body);
                helpers::row_u32("Duration (ms)", dto.duration.subsec_millis(), &mut body);
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
