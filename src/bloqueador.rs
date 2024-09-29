use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom};
use crate::config::CAMINHO_HOSTS;
pub struct Bloqueador;

impl Bloqueador {
    pub fn new() -> Self {
        Self
    }

    pub fn bloquear_hosts(&self, url: &str) -> String {
        match reqwest::blocking::get(url) {
            Ok(resposta) => {
                match resposta.text() {
                    Ok(conteudo) => {
                        match self.escrever_hosts(&conteudo) {
                            Ok(_) => format!("Hosts bloqueados com sucesso de {}", url),
                            Err(e) => format!("Erro ao escrever no arquivo hosts: {}", e),
                        }
                    },
                    Err(e) => format!("Erro ao ler o conteúdo da URL: {}", e),
                }
            },
            Err(e) => format!("Erro ao acessar a URL: {}", e),
        }
    }

    fn escrever_hosts(&self, conteudo: &str) -> std::io::Result<()> {
        let mut arquivo = OpenOptions::new()
            .append(true)
            .open(CAMINHO_HOSTS)?;

        writeln!(arquivo, "\n# Bloqueios adicionados pelo programa")?;
        for linha in conteudo.lines() {
            if linha.starts_with("0.0.0.0") {
                writeln!(arquivo, "{}", linha)?;
            }
        }
        Ok(())
    }
    pub fn limpar_hosts(&self) -> std::io::Result<()> {
        let mut arquivo = File::options()
            .read(true)
            .write(true)
            .open(CAMINHO_HOSTS)?;

        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo)?;

        let linhas: Vec<&str> = conteudo.lines().collect();
        let mut novo_conteudo = String::new();
        let mut dentro_do_bloco = false;

        for linha in linhas {
            if linha.contains("# Bloqueios adicionados pelo programa") {
                dentro_do_bloco = true;
                continue;
            }

            if !dentro_do_bloco {
                novo_conteudo.push_str(linha);
                novo_conteudo.push('\n');
            }

            if dentro_do_bloco && linha.trim().is_empty() {
                dentro_do_bloco = false;
            }
        }

        arquivo.set_len(0)?;
        arquivo.seek(SeekFrom::Start(0))?;
        arquivo.write_all(novo_conteudo.as_bytes())?;

        Ok(())
    }
}