use egui::Ui;
use egui::{Color32, Frame, Margin, RichText};
use egui_extras::{Column, TableBuilder};
use apex::state::parse_detail_dto::ParseDetailDTO;           // the model type
use crate::ui::helpers;
use crate::ui::visuals::banners::caption_banner;

pub fn draw_source_detail_view(ui: &mut Ui, dto: &ParseDetailDTO) {
    let status_text = helpers::status_as_rich_text(dto.parse_status());
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
                        row.col(|ui| helpers::right_label(ui, dto.file_name()));
                    });
                helpers::row_u64("Bytes parsed",  dto.bytes_parsed(),       &mut body);
                helpers::row_u32("Duration (ms)", dto.parse_duration_ms(),  &mut body);
                helpers::row_u64("Data rows",     dto.data_rows(),          &mut body);
                helpers::row_u64("Errors",        dto.errors_encountered(), &mut body);
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("Status");});
                        row.col(|ui| {ui.label(status_text);});
                    });
                });
        });
    });
	});
}
