//! src/main.rs

mod ui;
mod app;

use eframe::{egui, NativeOptions, Renderer};
use crate::app::rivvitium_app::RivvitiumApp;

fn main() -> eframe::Result<()> {
    let size     = egui::vec2(800.0, 640.0);
    let viewport = egui::ViewportBuilder::default()
        .with_app_id("Rivvitium")
        .with_inner_size(size);
    let renderer = Renderer::Wgpu;
    let options  = NativeOptions {
        viewport,
        renderer,
        ..Default::default()
    };
    eframe::run_native(
        "Rivvitium",
        options,
        Box::new(|_cc| Ok(Box::new(RivvitiumApp::default()))),
    )
}
