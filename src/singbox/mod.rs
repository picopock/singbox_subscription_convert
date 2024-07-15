pub mod common;
pub mod dns;
pub mod experimental;
pub mod inbound;
pub mod log;
pub mod outbound;
pub mod route;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log: log::Log,
    pub dns: dns::DNS,
    pub inbounds: Vec<inbound::Inbound>,
    pub outbounds: Vec<outbound::Outbound>,
    pub route: route::Route,
    pub experimental: experimental::Experimental,
}
