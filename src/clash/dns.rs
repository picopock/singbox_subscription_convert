use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DNS {
    pub enable: bool,
    pub ipv6: bool,
    pub default_nameserver: Vec<String>,
    pub enhanced_mode: String,
    pub fake_ip_range: String,
    pub use_hosts: bool,
    pub nameserver: Vec<String>,
    pub fallback: Vec<String>,
    pub fallback_filter: FallbackFilter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FallbackFilter {
    pub geoip: bool,
    #[serde(rename = "ipcidr")]
    pub ip_cidr: Vec<String>,
}
