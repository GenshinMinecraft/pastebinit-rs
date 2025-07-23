use crate::pastebin_server;
use crate::pastebin_server::provider_trait::ProviderTrait;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Provider {
    Debian,
    Centos,
}

pub fn upload(
    provider: Provider,
    content: String,
    title: String,
    private: bool,
    raw: bool,
) -> Result<String, String> {
    match provider {
        Provider::Debian => {
            pastebin_server::debian::Debian::upload_paste(content, title, private, raw)
        }
        Provider::Centos => {
            pastebin_server::centos::Centos::upload_paste(content, title, private, raw)
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "PastebinIt-rs", version, about)]
pub struct Config {
    /// Select the pastebin provider
    #[arg(short, long, value_enum, default_value_t = Provider::Debian)]
    pub server: Provider,

    /// Select file to upload (Ignored stdin)
    #[arg(short, long, default_value_t = String::new())]
    pub file: String,

    /// Set pastebin title
    #[arg(short, long, default_value_t = String::new())]
    pub title: String,

    /// Set pastebin visibility
    #[arg(short, long, default_value_t = false)]
    pub private: bool,

    /// Show RAW Text link
    #[arg(short, long, default_value_t = false)]
    pub raw: bool,
}
