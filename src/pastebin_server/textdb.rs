use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use rand::Rng;
use rand::distributions::Alphanumeric;
use reqwest::blocking::ClientBuilder;
use std::time::Duration;

pub struct TextDb;
impl ProviderTrait for TextDb {
    fn upload_paste(
        content: String,
        title: String,
        _private: bool,
        raw: bool,
    ) -> Result<PasteBinUrl, ErrorMessage> {
        let client = ClientBuilder::new()
            .user_agent("PastebinIt-rs")
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        let title = if title.is_empty() {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .map(char::from)
                .collect()
        } else {
            title
        };

        let payload = format!(
            "key={}&value={}",
            utf8_percent_encode(&title, NON_ALPHANUMERIC).collect::<String>(),
            utf8_percent_encode(&content, NON_ALPHANUMERIC).collect::<String>()
        );

        let response = client
            .post(format!("https://textdb.online/update/?{payload}"))
            .send()
            .map_err(|e| format!("Failed to send HTTP request: {e}"))?;

        if response.status().as_u16() == 413 {
            return Err("Payload Too Large".to_string());
        }

        if response.status() != 200 {
            return Err(format!(
                "Failed to upload paste: HTTP {}",
                response.status()
            ));
        }

        Ok(format!("https://textdb.online/{title}"))
    }
}
