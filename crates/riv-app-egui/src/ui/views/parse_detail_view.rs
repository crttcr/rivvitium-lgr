use egui::Ui;
use egui_extras::{Column, TableBuilder};
use apex::state::parse_detail_dto::ParseDetailDTO;           // the model type
use crate::ui::helpers;

const HORIZONTAL_PERCENT: f32 = 0.55;

/// Draw `ParseDetailDTO` in the given `Ui`.
pub fn show_parse_detail(ui: &mut Ui, dto: &ParseDetailDTO) {
    let status_text = helpers::status_as_rich_text(dto.parse_status());
    let half        = ui.available_width() * HORIZONTAL_PERCENT;

    ui.group(|ui| {
        ui.set_width(half);
        ui.heading("Parse details");

        TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                body.row(helpers::ROW_HEIGHT, |mut row| {
                    row.col(|ui| {ui.label("File name");});
//                    row.col(|ui| {ui.monospace(dto.file_name());});
                    row.col(|ui| {helpers::right_label(ui, dto.file_name());});
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
}
