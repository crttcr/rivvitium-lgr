use egui::{Color32, RichText, Ui};
use egui_extras::Column;
use crate::ui::helpers;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseStatus {
	NotParsed,
	InProgress,
	Finished,
	Error,
}



#[derive(Debug, PartialEq, Clone)]
pub struct ParseDetailDTO {
	file_name:          String,
	parse_status:       ParseStatus,
	bytes_parsed:       u64,
	parse_duration_ms:  u32,
	data_rows:          u64,
	errors_encountered: u64,
}

impl ParseDetailDTO {
	pub fn new(file_name: &str) -> Self {
	ParseDetailDTO {
			file_name:          file_name.to_string(),
			parse_status:       ParseStatus::NotParsed,
			bytes_parsed:       40395,
			parse_duration_ms:  4029,
			data_rows:          20000,
			errors_encountered: 0,
		}
	}
	
    /* ───────────── getters ───────────── */
    /// Original file name (borrowed).
    #[inline]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Current parse status enum.
    #[inline]
    pub fn parse_status(&self) -> ParseStatus {
        self.parse_status
    }

    /// Total bytes that have been processed so far.
    #[inline]
    pub fn bytes_parsed(&self) -> u64 {
        self.bytes_parsed
    }

    /// Time spent parsing (milliseconds).
    #[inline]
    pub fn parse_duration_ms(&self) -> u32 {
        self.parse_duration_ms
    }

    /// Number of data rows successfully read.
    #[inline]
    pub fn data_rows(&self) -> u64 {
        self.data_rows
    }

    /// Number of non-fatal errors encountered while parsing.
    #[inline]
    pub fn errors_encountered(&self) -> u64 {
        self.errors_encountered
    }    
    
    /// Replace all numeric counters in one call and return the updated value.
    ///
    /// Useful for code that collects all metrics first and then does a single
    /// immutable update:
    ///
    /// ```rust
    /// dto = dto.with_metrics(total_bytes, elapsed_ms, rows, errs);
    /// ```
    /// ```rust
    /// dto = dto
    ///     .with_metrics(b, t, rows, errs)
    ///     .finished()
    /// ```
    #[inline]
    pub fn with_metrics(
        self,
        bytes_parsed:       u64,
        parse_duration_ms:  u32,
        data_rows:          u64,
        errors_encountered: u64,
    ) -> Self {
        Self {
            bytes_parsed,
            parse_duration_ms,
            data_rows,
            errors_encountered,
            // keep the other fields the same
            ..self
        }
    }    

    /// Return a **new** value with `parse_status` replaced.
    #[inline]
    pub fn with_parse_status(self, status: ParseStatus) -> Self {
        Self {
            parse_status: status,
            ..self
        }
    }

    /// Convenience wrapper that marks the record **finished**.
    /// 
    #[inline]
    pub fn finished(self) -> Self {
        self.with_parse_status(ParseStatus::Finished)
    }    
}


impl ParseDetailDTO {
    /// Draw this DTO as a read-only table inside the given `Ui`.
    pub fn show(&self, ui: &mut Ui) {
    	
		let status_text = helpers::status_as_rich_text(self.parse_status());
		let half = ui.available_width() * 0.5;          // 50 % of the free width
	
        ui.group(|ui| {
        	ui.set_width(half);
            ui.heading("Parse details");

            // Two-column grid: label on the left, value on the right.
            egui_extras::TableBuilder::new(ui)
                .column(Column::auto())       // label column
                .column(Column::remainder())  // value column
                .body(|mut body| {
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("File name");});
                        row.col(|ui| {ui.monospace(self.file_name());});
                    });
                    helpers::row_u64("Bytes parsed",  self.bytes_parsed(),       &mut body);
                    helpers::row_u32("Duration (ms)", self.parse_duration_ms(),  &mut body);
                    helpers::row_u64("Data rows",     self.data_rows(),          &mut body);
                    helpers::row_u64("Errors",        self.errors_encountered(), &mut body);
                    body.row(helpers::ROW_HEIGHT, |mut row| {
                        row.col(|ui| {ui.label("Status");});
                        row.col(|ui| {ui.label(status_text);});
                    });
                });
        });
    }
}
