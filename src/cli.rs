use clap::Parser;
use std::path;
use url::Url;

#[derive(Parser, Debug)]
#[command(name = "SubscriptionConvert")]
#[command(author = "Glimmer")]
#[command(version = "0.1.0")]
#[command(about = "Convert clash subscription to singbox config", long_about = None)]
pub struct Args {
    /// The url you will subscribe
    pub url: Url,

    /// The singbox config template path, if not set, will use default template
    #[arg(short, long)]
    pub template: Option<path::PathBuf>,

    /// The path singbox will output
    #[arg(short, long, default_value = "config.json")]
    pub output: Option<path::PathBuf>,
}
