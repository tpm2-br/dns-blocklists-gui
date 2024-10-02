use eframe::egui;
use crate::bloqueador::Bloqueador;
use crate::config::URLS;
pub struct InterfaceBloqueador {
    bloqueador: Bloqueador,
    status: String,
    selecionados: Vec<bool>,
}

impl InterfaceBloqueador {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            bloqueador: Bloqueador::new(),
            status: String::new(),
            selecionados: vec![false; URLS.len()],
        }
    }
}

impl eframe::App for InterfaceBloqueador {
    fn update(&mut self, ctx: &egui::Context, _quadro: &mut eframe::Frame) {
        let tamanho_desejado = egui::vec2(400.0, 600.0);
        
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(tamanho_desejado);

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("DNS-Blocklists GUI");
                ui.add_space(20.0);
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (indice, (nome, _url, explicacao)) in URLS.iter().enumerate() {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add(egui::Checkbox::new(&mut self.selecionados[indice], ""))
                            .on_hover_text(format!("Selecionar {}", nome));
                        ui.label(nome.to_string())
                        .on_hover_ui(|ui| {
                            ui.label(explicacao.to_string()); // explicar m.
                        });
                        
                    });
                }
            });

            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                if ui.add_sized(
                    [200.0, 35.0],
                    egui::Button::new("SALVAR")
                        .fill(egui::Color32::from_rgb(0, 50, 200))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                ).clicked() {
                    self.status.clear();
                    if self.selecionados.iter().all(|&x| !x) {
                        match self.bloqueador.limpar_hosts() {
                            Ok(_) => self.status.push_str("Arquivo hosts limpo com sucesso.\n"),
                            Err(e) => self.status.push_str(&format!("Erro ao limpar o arquivo hosts: {}\n", e)),
                        }
                    } else {
                        for (indice, (_nome, url, _)) in URLS.iter().enumerate() {
                            if self.selecionados[indice] {
                                let resultado = self.bloqueador.bloquear_hosts(url);
                                self.status.push_str(&format!("{}\n", resultado));
                            }
                        }
                    }
                }
            });

            ui.add_space(20.0);

            if !self.status.is_empty() {
                egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    ui.label(&self.status);
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("Powered by TPM2");
                ui.hyperlink_to("Sobre", "https://github.com/tpm2-br");
            });
        });
    }
}