//! An egui modal dialog that constructs a `SinkSettings` instance.

use egui::{Context, TextEdit, Ui};
use rfd::FileDialog;
use riv::component::sink::sink_settings::SinkSettings;
use riv::component::sink::SinkKind;

/* ───────────────────────── dialog state ─────────────────────────── */
pub struct SinkDialog {
    pub open:   bool,                // show / hide flag
    kind:       SinkKind,            // radio-button selection

    // per-kind editable controls
    file_path:  String,
    delimiter:  String,
    pretty:     bool,
    server:     String,
    port:       String,
    table:      String,
    topic:      String,
    user_name:  String,
    password:   String,
    db_name:    String,
}

impl Default for SinkDialog {
    fn default() -> Self {
    	let kind      = SinkKind::Csv;
    	let delimiter = ",".to_string();
    	let file_path = "/tmp/foo.csv".to_string();
        Self {
            open:      false,
            kind,
            file_path,
            delimiter,
            pretty:     true,
            server:     "localhost".into(),
            port:       "9092".into(),       
            table:      "data".into(),
            topic:      "my_topic".into(),
            user_name:  "user".into(),
            password:   "".into(),
            db_name:    "my_database".into(),       
        }
    }
}

/* ───────────────────────── public API ───────────────────────────── */

impl SinkDialog {
    /// Show the dialog if `self.open == true`.
    /// Returns `Some(SinkConfig)` only when the user hits **OK**.
    pub fn show(&mut self, ctx: &Context) -> Option<SinkSettings> {
        let mut result = None;

        if self.open {
        	let mut open_flag = true;
            egui::Window::new("Configure a data destination")
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .open(&mut open_flag)
                .show(ctx, |ui| {
                    self.body(ui, &mut result);
                });
			//  write any change (user clicked × or Cancel) back to the struct field             
			self.open = open_flag;                
        }

        result
    }
}

/* ───────────────────────── rendering ────────────────────────────── */

impl SinkDialog {
    fn body(&mut self, ui: &mut Ui, out: &mut Option<SinkSettings>) {
        ui.vertical(|ui| {
            /* --- sink kind ------------------------------------------------ */

            ui.label("Choose the type of destination:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.kind, SinkKind::Capture, "Capture");
                ui.radio_value(&mut self.kind, SinkKind::Console, "Console");
                ui.radio_value(&mut self.kind, SinkKind::Csv,     "CSV");
                ui.radio_value(&mut self.kind, SinkKind::Json,    "JSON");
                ui.radio_value(&mut self.kind, SinkKind::Kafka,   "Kafka");
                ui.radio_value(&mut self.kind, SinkKind::Sqlite,  "SQLite");
                ui.radio_value(&mut self.kind, SinkKind::DevNull, "DevNull");
            });
            ui.add_space(8.0);

            /* --- per-kind configuration ----------------------------------- */

            match self.kind {
					SinkKind::Kafka => {
					 ui.horizontal(|ui| {
					  ui.label("Server:");
					  ui.text_edit_singleline(&mut self.server);
					 });
					 ui.horizontal(|ui| {
					  ui.label("Port:");
					  ui.add(
					  TextEdit::singleline(&mut self.port)
						.char_limit(5)
		            .desired_width(60.0),
					  )
    				});
					},
                SinkKind::Csv => {
                    ui.label("CSV file path:");
                    self.file_path = "/tmp/foo.csv".to_string();
                    self.path_edit_row(ui);
                }

                SinkKind::Json => {
                    self.file_path = "/tmp/foo.json".to_string();
                    ui.label("JSON file path:");
                    self.path_edit_row(ui);
                }

                SinkKind::Sqlite => {
                    self.file_path = "/tmp/foo.db".to_string();
                    ui.label("Sqlite database path:");
                    self.path_edit_row(ui);
                }
                _ => {}
            }

if matches!(self.kind, SinkKind::Csv) {
    ui.horizontal(|ui| {
        ui.label("Delimiter:");
        // delimiter lives as a one-char String field on self
        ui.add(
            TextEdit::singleline(&mut self.delimiter)
                .char_limit(1)
                .desired_width(20.0),
        );
    });
}


            if matches!(self.kind, SinkKind::Json) {
                ui.checkbox(&mut self.pretty, "Pretty-print");
            }

            if matches!(self.kind, SinkKind::Sqlite) {
                ui.horizontal(|ui| {
                    ui.label("Table:");
                    ui.text_edit_singleline(&mut self.table);
                });
            }

            ui.add_space(12.0);
            ui.separator();

            /* --- action buttons ------------------------------------------- */

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        self.open = false;
                    }

                    let ok_enabled = self.ok_button_enabled();
                    if ui.add_enabled(ok_enabled, egui::Button::new("OK")).clicked() {
                        self.open = false;
                        *out = Some(self.build_config());
                    }
                });
            });
        });
    }

    /* --- helper: browse -------------------------------------------------- */

    fn path_edit_row(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.file_path);

            if ui.button("Browse…").clicked() {
                if let Some(path) = FileDialog::new().save_file() {
                    self.file_path = path.display().to_string();
                }
            }
        });
    }

    /* --- helper: can we enable OK? -------------------------------------- */
    fn ok_button_enabled(&self) -> bool {
        match self.kind {
            SinkKind::Csv | SinkKind::Json | SinkKind::Sqlite => !self.file_path.trim().is_empty(),
            SinkKind::Kafka => !self.server.trim().is_empty() && self.port.parse::<u16>().is_ok(),           
            SinkKind::Capture | SinkKind::Console | SinkKind::DevNull => true,
            SinkKind::SqlServer => false,
            _ => false,       
        }
    }

    /* --- helper: build the SinkSettings ----------------------------------- */
    fn build_config(&self) -> SinkSettings {
        match self.kind {
            SinkKind::Capture   => SinkSettings::capture(),
            SinkKind::Console   => SinkSettings::console(),
            SinkKind::DevNull   => SinkSettings::dev_null(),
            SinkKind::Csv       => {
					let delim_char = self.delimiter.chars().next().unwrap_or(',');
					SinkSettings::csv(self.file_path.clone(), delim_char)            	
            },
            SinkKind::Json      => SinkSettings::json(
                self.file_path.clone(),
                self.pretty,
            ),
	        SinkKind::Kafka => SinkSettings::kafka(
	            self.server.clone(),
	            self.port.parse::<u16>().unwrap_or(9092),
	            self.topic.clone(),
	        ),            
	        SinkKind::PubSub => SinkSettings::pubsub(
	            self.server.clone(),
	            self.port.parse::<u16>().unwrap_or(9092),
	            self.topic.clone(),
	        ),              
	        SinkKind::Sqlite    => SinkSettings::sqlite(
                self.file_path.clone(),
                self.table.clone(),
            ),
            SinkKind::SqlServer    => SinkSettings::sqlserver(
                self.server.clone(),
                self.port.parse::<u16>().unwrap_or(1433),           
                self.user_name.clone(),
                self.password.clone(),
                self.db_name.clone(),           
            ),
        }
    }
}