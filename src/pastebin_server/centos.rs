use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use std::time::Duration;

pub struct Centos;
impl ProviderTrait for Centos {
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

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let payload = format!(
            "name=PastebinIt-rs&title={}&lang=text&code={}&expire=14400000&submit=submit",
            utf8_percent_encode(&title, NON_ALPHANUMERIC).collect::<String>(),
            utf8_percent_encode(&content, NON_ALPHANUMERIC).collect::<String>()
        );

        let response = client
            .post("https://paste.centos.org/")
            .headers(headers)
            .body(payload)
            .send()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        if response.status().as_u16() == 413 {
            return Err("Payload Too Large".to_string());
        }

        if response.status() != 200 {
            return Err(format!(
                "Failed to upload paste: HTTP {}",
                response.status()
            ));
        }

        let response_text = response
            .text()
            .map_err(|e| format!("Failed to read response: {e}"))?;
        use regex::Regex;
        let re = Regex::new(r"https://paste\.centos\.org/view/download/?([a-zA-Z0-9]+)\b").unwrap();
        let id = if let Some(caps) = re.captures(&response_text) {
            let id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            id
        } else {
            return Err("Failed to parse paste ID from response".to_string());
        };

        if raw {
            Ok(format!("https://paste.centos.org/view/raw/{}", id))
        } else {
            Ok(format!("https://paste.centos.org/view/{}", id))
        }
    }
}
