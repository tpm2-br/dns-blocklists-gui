#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(windows)]
extern crate winapi;
mod gui;
mod bloqueador;
mod config;
use eframe::egui;
const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 600.0;

struct AplicativoBloqueador {
    interface: gui::InterfaceBloqueador,
}

impl AplicativoBloqueador {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            interface: gui::InterfaceBloqueador::new(cc),
        }
    }
}

impl eframe::App for AplicativoBloqueador {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.interface.update(ctx, _frame);
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)));
    }
}
fn main() -> Result<(), eframe::Error> {
    let opcoes = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "DNS-Blocklists GUI",
        opcoes,
        Box::new(|cc| Ok(Box::new(AplicativoBloqueador::new(cc)))),
    )
}