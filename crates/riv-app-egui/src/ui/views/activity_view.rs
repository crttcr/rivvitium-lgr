use std::time::{Duration, Instant};
use chrono::{DateTime, Local};
use egui::{Label};
use egui_extras::{Column, TableBuilder};

/// One log item for the activity view.
#[derive(Clone)]
pub struct ActivityEvent {
    pub time:  Instant,   // when the event happened
    pub label: String,    // short text (“Started parse”, “Wrote CSV”, …)
}

/// Call this from your main layout:
///
/// ```rust
/// activity_view(ui, &self.event_log);
/// ```
///
pub fn activity_view(ui: &mut egui::Ui, events: &[ActivityEvent]) {
    ui.group(|ui| {
        ui.heading("Activity");
        if events.is_empty() {
            ui.label("No activity has been recorded");
            return;
        }

        TableBuilder::new(ui)
            .column(Column::exact(100.0))     // “ago” column
            .column(Column::exact(200.0))     // timestamp
            .column(Column::remainder())      // label column
            .body(|mut body| {
                for ev in events.iter().rev() {
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                        	let v   = format_duration(ev.time.elapsed());
                        	let label = Label::new(v);
                           ui.add(label);
                        });
                        // ---------- col 2: timestamp -------------
                        row.col(|ui| {
                        	let v   = format_timestamp(ev.time);
                        	let label = Label::new(v);
                            ui.add(label);
                        });
                        row.col(|ui| {
                            ui.label(&ev.label);
                        });
                    });
                }
            });
    });
}

/* Helper: prettify Duration → “4 s”, “12 m”, “2 h” */
fn format_duration(d: Duration) -> String {
    let s = d.as_secs();
    match s {
        0..=59           => format!("{s}s"),
        60..=3599        => format!("{}m", s / 60),
        3600..=86_399    => format!("{}h", s / 3600),
        _                => format!("{}d", s / 86_400),
    }
}

fn format_timestamp(t: Instant) -> String {
    let sys: DateTime<Local> =
        (Local::now() - chrono::Duration::from_std(t.elapsed()).unwrap()).into();
    sys.format("%H:%M:%S").to_string() // e.g. 14:32:07
}
