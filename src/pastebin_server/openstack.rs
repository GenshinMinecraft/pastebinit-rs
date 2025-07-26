use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::ClientBuilder;
use std::time::Duration;

pub struct OpenStack;
impl ProviderTrait for OpenStack {
    fn upload_paste(
        content: String,
        _title: String,
        private: bool,
        raw: bool,
    ) -> Result<PasteBinUrl, ErrorMessage> {
        let client = ClientBuilder::new()
            .user_agent("PastebinIt-rs")
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        let payload = if private {
            format!(
                "code={}&language=text&private=on",
                utf8_percent_encode(&content, NON_ALPHANUMERIC).collect::<String>()
            )
        } else {
            format!(
                "code={}&language=text",
                utf8_percent_encode(&content, NON_ALPHANUMERIC).collect::<String>()
            )
        };

        let response = client
            .post("https://paste.openstack.org/")
            .body(payload)
            .header("Content-Type", "application/x-www-form-urlencoded")
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

        let re = regex::Regex::new(r#"<title>Paste #([A-Za-z0-9]+) \| LodgeIt!</title>"#).unwrap();
        let id = if let Some(caps) = re.captures(&response_text) {
            caps.get(1).map(|m| m.as_str()).unwrap_or("")
        } else {
            return Err("Failed to parse paste ID from response".to_string());
        };

        if raw {
            Ok(format!("https://paste.openstack.org/raw/{}/", id))
        } else {
            Ok(format!("https://paste.openstack.org/show/{}/", id))
        }
    }
}
