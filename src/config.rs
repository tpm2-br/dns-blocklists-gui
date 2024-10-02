pub const CAMINHO_HOSTS: &str = r"C:\Windows\System32\drivers\etc\hosts";

pub const URLS: [(&str, &str, &str); 5] = [
    (
        "LIGHT",
        "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/light.txt",
        "A versão LIGHT bloqueia anúncios, rastreadores, métricas e sites de badware. É ideal para usuários que querem uma proteção leve e otimizada. OBS: Adicione o arquivo hosts nas exclusões do Windows Defender!"
    ),
    (
        "Microsoft (Windows, Office, MSN)",
        "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.winoffice.txt",
        "Esta lista bloqueia servidores de telemetria e rastreamento específicos da Microsoft, incluindo Windows, Office e MSN."
    ),
    (
        "Amazon (Devices, Shopping, Video)",
        "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.amazon.txt",
        "Bloqueia telemetria e rastreamento em dispositivos Amazon, incluindo compras e serviços de vídeo."
    ),
    (
        "TikTok (Fingerprinting)",
        "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.tiktok.txt",
        "Esta lista foca em bloquear os métodos de fingerprinting usados pelo TikTok, protegendo a privacidade dos usuários."
    ),
    (
        "TikTok (Fingerprinting) Aggressive",
        "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.tiktok.extended.txt",
        "Uma versão mais agressiva da lista TikTok, bloqueando ainda mais rastreamento e fingerprinting."
    ),
];
