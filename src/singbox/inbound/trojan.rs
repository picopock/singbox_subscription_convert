use super::base::User;
use crate::singbox::common::{base::Strategy, multiplex::Multiplex, tls, transport::Transport};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Trojan {
    pub tag: String,
    pub users: Vec<User>,
    pub listen: Option<String>,
    pub listen_port: Option<u16>,
    pub tcp_fast_open: Option<bool>,
    pub tcp_multi_path: Option<bool>,
    pub udp_fragment: Option<bool>,
    pub udp_timeout: Option<String>,
    pub detour: Option<String>,
    pub sniff: Option<bool>,
    pub sniff_override_destination: Option<bool>,
    pub sniff_timeout: Option<String>,
    pub domain_strategy: Option<Strategy>,
    pub udp_disable_domain_unmapping: Option<bool>,
    pub tls: Option<tls::Inbound>,
    pub fallback: Option<Fallback>,
    pub fallback_for_alpn: Option<FallbackForAlpn>,
    pub multiplex: Option<Multiplex>,
    pub transport: Option<Transport>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Fallback {
    pub server: String,
    pub server_port: u16,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FallbackForAlpn {}
