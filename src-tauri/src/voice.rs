use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice: String,
    pub speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResponse {
    pub audio_data: Vec<u8>,
    pub format: String,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttRequest {
    pub audio_data: Vec<u8>,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResponse {
    pub text: String,
    pub confidence: Option<f64>,
}

pub trait TtsProvider: Send + Sync {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn synthesize(&self, request: TtsRequest) -> impl std::future::Future<Output = Result<TtsResponse>> + Send;
    fn is_available(&self) -> impl std::future::Future<Output = bool> + Send;
}

pub trait SttProvider: Send + Sync {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn transcribe(&self, request: SttRequest) -> impl std::future::Future<Output = Result<SttResponse>> + Send;
    fn is_available(&self) -> impl std::future::Future<Output = bool> + Send;
}

pub struct KokoroProvider {
    client: Client,
    endpoint: String,
}

impl KokoroProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            endpoint,
        }
    }
}

impl TtsProvider for KokoroProvider {
    fn name(&self) -> &str {
        "kokoro"
    }

    async fn synthesize(&self, request: TtsRequest) -> Result<TtsResponse> {
        let url = format!("{}/v1/audio/speech", self.endpoint);

        let body = serde_json::json!({
            "model": "kokoro",
            "input": request.text,
            "voice": request.voice,
            "speed": request.speed,
            "response_format": "mp3",
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Kokoro API error ({}): {}", status, text));
        }

        let content_type = response.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("audio/mpeg")
            .to_string();

        let format = if content_type.contains("wav") {
            "wav".to_string()
        } else if content_type.contains("mp3") {
            "mp3".to_string()
        } else {
            "audio".to_string()
        };

        let audio_data = response.bytes().await?.to_vec();

        Ok(TtsResponse {
            audio_data,
            format,
            duration_ms: None,
        })
    }

    async fn is_available(&self) -> bool {
        let url = format!("{}/v1/models", self.endpoint);
        self.client.get(&url).send().await.is_ok()
    }
}

pub struct WhisperProvider {
    client: Client,
    endpoint: String,
}

impl WhisperProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            endpoint,
        }
    }
}

impl SttProvider for WhisperProvider {
    fn name(&self) -> &str {
        "whisper"
    }

    async fn transcribe(&self, request: SttRequest) -> Result<SttResponse> {
        let url = format!("{}/v1/audio/transcriptions", self.endpoint);

        let body = serde_json::json!({
            "audio_data": base64::engine::general_purpose::STANDARD.encode(&request.audio_data),
            "format": request.format,
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Whisper API error ({}): {}", status, text));
        }

        let body: serde_json::Value = response.json().await?;
        let text = body["text"].as_str().unwrap_or("").to_string();

        Ok(SttResponse {
            text,
            confidence: None,
        })
    }

    async fn is_available(&self) -> bool {
        let url = format!("{}/v1/models", self.endpoint);
        self.client.get(&url).send().await.is_ok()
    }
}
