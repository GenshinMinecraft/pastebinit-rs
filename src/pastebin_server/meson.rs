use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use regex::Regex;
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use std::time::Duration;

pub struct Meson;
impl ProviderTrait for Meson {
    fn upload_paste(
        content: String,
        _title: String,
        _private: bool,
        raw: bool,
    ) -> Result<PasteBinUrl, ErrorMessage> {
        let client = ClientBuilder::new()
            .user_agent("PastebinIt-rs")
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        let payload = format!(
            "expires=315360000&content={}",
            utf8_percent_encode(&content, NON_ALPHANUMERIC).collect::<String>()
        );

        let mut headers = header::HeaderMap::new();
        headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
        headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert(
            "content-type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert("origin", "https://pb.meson.cc".parse().unwrap());
        headers.insert("referer", "https://pb.meson.cc/paste/".parse().unwrap());
        headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".parse().unwrap());

        let response = client
            .post("https://pb.meson.cc/upload")
            .headers(headers)
            .body(payload)
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

        let response_text = response
            .text()
            .map_err(|e| format!("Failed to read response text: {e}"))?;

        let re = Regex::new(r"Pastebin - (\w+)\.txt").unwrap();
        let id = if let Some(caps) = re.captures(&response_text) {
            caps.get(1).map(|m| m.as_str()).unwrap_or("")
        } else {
            return Err("Failed to parse paste ID from response".to_string());
        };

        if raw {
            Ok(format!("https://pb.meson.cc/s/{id}.txt"))
        } else {
            Ok(format!("https://pb.meson.cc/{id}.txt"))
        }
    }
}
