use anyhow::{Result, anyhow};
use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ChatCompletionRequest, ChatCompletionResponse, Choice, Usage};
use crate::models::OpenRouterModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterModelDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub context_tokens: u64,
    pub pricing_prompt: f64,
    pub pricing_completion: f64,
    pub architecture: String,
    pub category: String,
    pub provider: String,
    pub endpoints: Option<Vec<String>>,
}

pub struct OpenRouterProvider {
    client: Client,
    base_url: String,
}

impl OpenRouterProvider {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    pub fn with_base_url(url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
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
    ) -> Result<BoxStream<'static, Result<String>>> {
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

        Ok(stream.boxed())
    }

    pub async fn fetch_models(&self, api_key: &str) -> Result<Vec<OpenRouterModel>> {
        let url = format!("{}/models", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://luna-desktop.app")
            .header("X-Title", "LUNA Desktop AI Assistant")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenRouter API error ({}): {}", status, body));
        }

        let body: Value = response.json().await?;

        let mut models = Vec::new();
        let data = body.get("data").and_then(|d| d.as_array());
        let empty: Vec<serde_json::Value> = Vec::new();
        let data = data.unwrap_or(&empty);

        for m in data {
            let id = m["id"].as_str().unwrap_or("").to_string();
            let name = m["name"].as_str().unwrap_or(&id).to_string();
            let description = m.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string();
            let context_tokens = m.get("context_tokens").and_then(|c| c.as_u64()).unwrap_or(0);

            let pricing = m.get("pricing").and_then(|p| {
                Some((
                    p.get("prompt").and_then(|v| v.as_str())?.parse::<f64>().unwrap_or(0.0),
                    p.get("completion").and_then(|v| v.as_str())?.parse::<f64>().unwrap_or(0.0),
                ))
            }).unwrap_or((0.0, 0.0));

            let architecture = m.get("architecture")
                .and_then(|a| a.get("modality"))
                .and_then(|m| m.as_str())
                .unwrap_or("unknown")
                .to_string();

            let category = m.get("category")
                .and_then(|c| c.as_str())
                .unwrap_or("general")
                .to_string();

            let provider = m.get("owning_organization")
                .and_then(|o| o.as_str())
                .unwrap_or("unknown")
                .to_string();

            models.push(OpenRouterModel {
                id,
                name,
                description,
                context_tokens,
                pricing_prompt: pricing.0,
                pricing_completion: pricing.1,
                architecture,
                category,
                provider,
            });
        }

        Ok(models)
    }

    pub async fn test_connection(&self, api_key: &str) -> Result<bool> {
        let url = format!("{}/models", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://luna-desktop.app")
            .header("X-Title", "LUNA Desktop AI Assistant")
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}
