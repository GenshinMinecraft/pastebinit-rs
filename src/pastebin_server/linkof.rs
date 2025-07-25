use crate::pastebin_server::provider_trait::{ErrorMessage, PasteBinUrl, ProviderTrait};
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use std::time::Duration;
use miniserde::{json, Serialize, Deserialize};
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    key: String,
    value: String,
}

pub struct LinkOf;
impl ProviderTrait for LinkOf {
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

        let payload = Payload {
            key: if title.is_empty() {
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect()
            } else {
                title
            },
            value: content,
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("accept", "*/*".parse().unwrap());
        headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert("content-type", "application/json;charset=UTF-8".parse().unwrap());
        headers.insert("dnt", "1".parse().unwrap());
        headers.insert("origin", "https://note.linkof.link".parse().unwrap());
        headers.insert("priority", "u=1, i".parse().unwrap());
        headers.insert("referer", "https://note.linkof.link/".parse().unwrap());
        headers.insert("sec-ch-ua", "\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\", \"Google Chrome\";v=\"138\"".parse().unwrap());
        headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
        headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
        headers.insert("sec-fetch-dest", "empty".parse().unwrap());
        headers.insert("sec-fetch-mode", "cors".parse().unwrap());
        headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
        headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".parse().unwrap());

        let response = client
            .post("https://note.linkof.link/set")
            .body(json::to_string(&payload))
            .headers(headers)
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

        if raw {
            Ok(format!("https://note.linkof.link/{}", payload.key))
        } else {
            Ok(format!("https://note.linkof.link/{}.html", payload.key))
        }
    }
}
