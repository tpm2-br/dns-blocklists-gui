pub const CAMINHO_HOSTS: &str = r"C:\Windows\System32\drivers\etc\hosts";

pub const URLS: [(&str, &str); 4] = [
    ("Microsoft (Windows, Office, MSN)", "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.winoffice.txt"),
    ("Amazon (Devices, Shopping, Video)", "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.amazon.txt"),
    ("TikTok (Fingerprinting)", "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.tiktok.txt"),
    ("TikTok (Fingerprinting) Aggressive", "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/native.tiktok.extended.txt"),
];