#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(windows)]
extern crate winapi;
mod gui;
mod bloqueador;
mod config;
use eframe::egui;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::io::Read;
use std::sync::Arc; 
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
    let mut opcoes = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_resizable(false),
        ..Default::default()
    };
        // up musica background
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = File::open("im-alone.mp3").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
        // up ícone
        let mut dados_icone = Vec::new();
        if let Ok(mut file) = File::open("icon.ico") {
            if file.read_to_end(&mut dados_icone).is_ok() {
                opcoes.viewport.icon = Some(Arc::new(egui::IconData {
                    rgba: dados_icone,
                    width: 128, 
                    height: 128,
                }));
            }
        }

    eframe::run_native(
        "DNS-Blocklists GUI",
        opcoes,
        Box::new(|cc| Ok(Box::new(AplicativoBloqueador::new(cc)))),
    )
}