pub mod dns;
pub mod proxy;
pub mod proxy_group;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mixed_port: u16,
    pub allow_lan: bool,
    pub bind_address: String,
    pub mode: Mode,
    pub log_level: LogLevel,
    pub external_controller: String,
    pub dns: dns::DNS,
    pub proxies: Vec<proxy::Proxy>,
    pub proxy_groups: Vec<proxy_group::ProxyGroup>,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Global,
    Rule,
    Direct,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
