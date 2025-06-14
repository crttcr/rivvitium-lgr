use eframe::epaint::StrokeKind;
use egui::{Color32, RichText, Sense, Stroke, Vec2, Painter, Rect, Pos2, vec2};

/// Temporary stand-in for the real data table / chart.
/// Call with:
///
/// ```rust
/// data_view(ui);   // inside an `allocate_ui` block
/// ```
///
/// The widget has no state: each frame it just draws the placeholder.
pub fn data_view(ui: &mut egui::Ui) {
    let remaining = ui.available_size();          
    let desired   = vec2(remaining.x, remaining.y - 150.0);
    let (rect, _resp) = ui.allocate_exact_size(desired, Sense::hover());

    let painter: Painter = ui.painter_at(rect);

    // 2️⃣ draw a subtle checkerboard background
    let bg1 = Color32::from_gray(240);
    let bg2 = Color32::from_gray(225);
    let cell = 16.0;

    let cols = ((rect.width()  / cell).ceil() as usize).max(1);
    let rows = ((rect.height() / cell).ceil() as usize).max(1);

    for r in 0..rows {
        for c in 0..cols {
            let x0 = rect.left() + c as f32 * cell;
            let y0 = rect.top()  + r as f32 * cell;
            let x1 = (x0 + cell).min(rect.right());
            let y1 = (y0 + cell).min(rect.bottom());

            let fill = if (r + c) % 2 == 0 { bg1 } else { bg2 };
            painter.rect_filled(Rect::from_min_max(Pos2::new(x0, y0), Pos2::new(x1, y1)), 0.0, fill);
        }
    }

    // 3️⃣ draw an outline
    painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::DARK_GRAY), StrokeKind::Middle);

    // 4️⃣ centered label
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Data view (WIP)",
        egui::FontId::proportional(16.0),
        Color32::DARK_GRAY,
    );
}