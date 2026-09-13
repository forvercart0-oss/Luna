use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::Value;

use super::{ChatCompletionRequest, ChatCompletionResponse, Choice, Usage};

pub struct OpenRouterProvider {
    client: Client,
    base_url: String,
}

impl OpenRouterProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    pub fn with_base_url(url: String) -> Self {
        Self {
            client: Client::new(),
            base_url: url,
        }
    }

    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
        api_key: &str,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://luna-desktop.app")
            .header("X-Title", "LUNA Desktop AI Assistant")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenRouter API error ({}): {}", status, body));
        }

        let body: Value = response.json().await?;

        let id = body["id"].as_str().unwrap_or("").to_string();

        let choices = body["choices"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|c| Choice {
                        index: c["index"].as_i64().unwrap_or(0),
                        message: super::ChatMessage {
                            role: c["message"]["role"]
                                .as_str()
                                .unwrap_or("assistant")
                                .to_string(),
                            content: c["message"]["content"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                        },
                        finish_reason: c["finish_reason"]
                            .as_str()
                            .map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let usage = body.get("usage").map(|u| Usage {
            prompt_tokens: u.get("prompt_tokens").and_then(|v| v.as_i64()).unwrap_or(0),
            completion_tokens: u.get("completion_tokens").and_then(|v| v.as_i64()).unwrap_or(0),
            total_tokens: u.get("total_tokens").and_then(|v| v.as_i64()).unwrap_or(0),
        });

        Ok(ChatCompletionResponse {
            id,
            choices,
            usage,
        })
    }

    pub async fn stream_chat_completion(
        &self,
        request: ChatCompletionRequest,
        api_key: &str,
    ) -> Result<impl futures_util::Stream<Item = Result<String>>> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://luna-desktop.app")
            .header("X-Title", "LUNA Desktop AI Assistant")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenRouter API error ({}): {}", status, body));
        }

        let stream = response.bytes_stream().filter_map(|chunk| async move {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes).to_string();
                    let mut results = Vec::new();
                    for line in text.lines() {
                        let line = line.trim();
                        if line.is_empty() || !line.starts_with("data: ") {
                            continue;
                        }
                        let data = &line[6..];
                        if data == "[DONE]" {
                            continue;
                        }
                        if let Ok(v) = serde_json::from_str::<Value>(data) {
                            if let Some(content) = v["choices"][0]["delta"]["content"].as_str() {
                                results.push(Ok(content.to_string()));
                            }
                        }
                    }
                    if results.is_empty() {
                        None
                    } else if results.len() == 1 {
                        Some(results.into_iter().next().unwrap())
                    } else {
                        Some(Ok(results.into_iter().map(|r| r.unwrap_or_default()).collect::<String>()))
                    }
                }
                Err(e) => Some(Err(anyhow!("Stream error: {}", e))),
            }
        });

        Ok(stream)
    }
}
