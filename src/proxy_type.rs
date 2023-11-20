use serde::Serialize;

#[derive(Serialize, Default, Clone, Debug)]
pub enum ProxyType {
    #[default]
    Http,
    Https,
    Socks4,
    Socks5
}

impl From<ProxyType> for String {
    fn from(proxy_type: ProxyType) -> String {
        match proxy_type {
            ProxyType::Http => "http".to_string(),
            ProxyType::Https => "https".to_string(),
            ProxyType::Socks4 => "socks4".to_string(),
            ProxyType::Socks5 => "socks5".to_string(),
        }
    }
}