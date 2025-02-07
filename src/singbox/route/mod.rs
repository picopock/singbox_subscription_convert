pub mod rule;
pub mod rule_set;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::base::Strategy;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    default_domain_resolver: StringOrDomainResolver,
    rules: Vec<rule::Rule>,
    rule_set: Vec<rule_set::RuleSet>,
    r#final: String,
    auto_detect_interface: Option<bool>,
    override_android_vpn: Option<bool>,
    default_interface: Option<String>,
    default_mark: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StringOrDomainResolver {
    Str(String),
    DomainResolver(DomainResolver),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainResolver {
    server: String,
    strategy: Option<Strategy>,
    disable_cache: Option<bool>,
    rewrite_ttl: Option<i32>,
    client_subnet: Option<String>,
}
