pub mod rule;
pub mod rule_set;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    rules: Vec<rule::Rule>,
    rule_set: Vec<rule_set::RuleSet>,
    r#final: String,
    auto_detect_interface: Option<bool>,
    override_android_vpn: Option<bool>,
    default_interface: Option<String>,
    default_mark: Option<usize>,
}
