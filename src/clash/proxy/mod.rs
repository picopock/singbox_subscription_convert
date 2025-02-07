mod http;
mod shadowsocks;
mod shadowsocks_r;
mod snell;
mod socks5;
mod trojan;
mod vmess;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Proxy {
    Ss(shadowsocks::Shadowsocks),
    Ssr(shadowsocks_r::ShadowsocksR),
    Vmess(vmess::Vmess),
    Socks5(socks5::Socks5),
    Http(http::Http),
    Snell(snell::Snell),
    Trojan(trojan::Trojan),
}
