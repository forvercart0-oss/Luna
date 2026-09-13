use crate::voice::{KokoroProvider, TtsProvider, TtsRequest, WhisperProvider, SttProvider, SttRequest};
use crate::AppState;
use base64::Engine;
use tauri::State;

#[tauri::command]
pub async fn tts_speak(
    state: State<'_, AppState>,
    text: String,
    voice: Option<String>,
    speed: Option<f64>,
) -> Result<String, String> {
    let settings = state.settings.read().await;
    let voice_settings = &settings.get().voice;

    if !voice_settings.tts_enabled {
        return Err("TTS is not enabled in settings".to_string());
    }

    let endpoint = std::env::var("KOKORO_TTS_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8880".to_string());

    let provider = KokoroProvider::new(endpoint);

    let available = provider.is_available().await;
    if !available {
        return Err("Kokoro TTS server is not reachable".to_string());
    }

    let request = TtsRequest {
        text,
        voice: voice.unwrap_or_else(|| voice_settings.voice.clone()),
        speed: speed.unwrap_or(voice_settings.speed),
    };

    let response = provider
        .synthesize(request)
        .await
        .map_err(|e| e.to_string())?;

    let audio_b64 = base64::engine::general_purpose::STANDARD.encode(&response.audio_data);

    Ok(serde_json::to_string(&serde_json::json!({
        "audio": audio_b64,
        "format": response.format,
        "duration_ms": response.duration_ms,
    })).unwrap_or_default())
}

#[tauri::command]
pub async fn stt_transcribe(
    _state: State<'_, AppState>,
    audio_data_b64: String,
    format: Option<String>,
) -> Result<String, String> {
    let endpoint = std::env::var("WHISPER_STT_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let provider = WhisperProvider::new(endpoint);

    let available = provider.is_available().await;
    if !available {
        return Err("Whisper STT server is not reachable".to_string());
    }

    let audio_data = base64::engine::general_purpose::STANDARD
        .decode(&audio_data_b64)
        .map_err(|e| e.to_string())?;

    let request = SttRequest {
        audio_data,
        format: format.unwrap_or_else(|| "mp3".to_string()),
    };

    let response = provider
        .transcribe(request)
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_string(&serde_json::json!({
        "text": response.text,
        "confidence": response.confidence,
    })).unwrap_or_default())
}

#[tauri::command]
pub async fn tts_check_availability() -> Result<bool, String> {
    let endpoint = std::env::var("KOKORO_TTS_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8880".to_string());

    let provider = KokoroProvider::new(endpoint);
    Ok(provider.is_available().await)
}

#[tauri::command]
pub async fn stt_check_availability() -> Result<bool, String> {
    let endpoint = std::env::var("WHISPER_STT_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let provider = WhisperProvider::new(endpoint);
    Ok(provider.is_available().await)
}
