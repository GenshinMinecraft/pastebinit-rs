use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use regex::Regex;
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use std::time::Duration;

pub struct Debian;
impl ProviderTrait for Debian {
    fn upload_paste(
        content: String,
        title: String,
        private: bool,
        raw: bool,
    ) -> Result<PasteBinUrl, ErrorMessage> {
        if content.lines().collect::<Vec<&str>>().len() <= 2 {
            return Err(String::from("Debian pastebin need more than 2 lines"));
        }

        let client = ClientBuilder::new()
            .user_agent("PastebinIt-rs")
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        let form = reqwest::blocking::multipart::Form::new()
            .text("poster", title)
            .text("lang", "-1")
            .text("expire", "-1")
            .text("private", if private { "1" } else { "0" })
            .text("code", content);

        let response = client
            .post("https://paste.debian.net/")
            .headers(header::HeaderMap::new())
            .multipart(form)
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
            .map_err(|e| format!("Failed to read response text: {e}"))?;

        let re = Regex::new(r"//paste\.debian\.net/plainh?/(\w+)").unwrap();
        let id = if let Some(caps) = re.captures(&response_text) {
            caps.get(1).map(|m| m.as_str()).unwrap_or("")
        } else {
            return Err("Failed to parse paste ID from response".to_string());
        };

        if raw {
            if private {
                Ok(format!("https://paste.debian.net/plainh/{id}"))
            } else {
                Ok(format!("https://paste.debian.net/plain/{id}"))
            }
        } else if private {
            Ok(format!("https://paste.debian.net/?hidden={id}"))
        } else {
            Ok(format!("https://paste.debian.net/{id}"))
        }
    }
}
