use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    pub name: String,
    pub r#type: String,
    pub server: String,
    pub port: u16,
    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: usize,
    pub cipher: String,
    pub udp: bool,
}
