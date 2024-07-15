use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    disabled: Option<bool>,
    #[serde(default)]
    level: LogLevel,
    output: Option<String>,
    #[serde(default = "default_timestamp")]
    timestamp: bool,
}

fn default_timestamp() -> bool {
    true
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
    Panic,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}
