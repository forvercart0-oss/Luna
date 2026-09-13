use crate::models::{Conversation, Message};
use crate::providers::openrouter::OpenRouterProvider;
use crate::providers::{ChatCompletionRequest, ChatMessage};
use crate::AppState;
use futures_util::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, State};
use uuid::Uuid;

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub async fn create_conversation(
    state: State<'_, AppState>,
    title: Option<String>,
    model_profile_id: Option<String>,
) -> Result<Conversation, String> {
    let mgr = state.conversation_manager.read().await;
    mgr.create(title, model_profile_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversations(
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String> {
    let mgr = state.conversation_manager.read().await;
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages(
    state: State<'_, AppState>,
    conversation_id: String,
) -> Result<Vec<Message>, String> {
    let mgr = state.conversation_manager.read().await;
    mgr.get_messages(&conversation_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_message(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    conversation_id: String,
    content: String,
    model_profile_id: Option<String>,
) -> Result<Message, String> {
    CANCEL_FLAG.store(false, Ordering::Relaxed);

    let msg_id = Uuid::new_v4().to_string();

    let user_msg = Message {
        id: msg_id.clone(),
        conversation_id: conversation_id.clone(),
        role: "user".to_string(),
        content: content.clone(),
        model_profile_id: model_profile_id.clone(),
        status: "complete".to_string(),
        metadata: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    {
        let mgr = state.conversation_manager.read().await;
        mgr.add_message(&user_msg).map_err(|e| e.to_string())?;
    }

    // Emit user message event
    let _ = app.emit("chat:user-message", &user_msg);

    let profile = {
        let model_mgr = state.model_manager.read().await;
        if let Some(ref mp_id) = model_profile_id {
            model_mgr.get_by_id(mp_id).ok()
        } else {
            model_mgr.get_active().ok().flatten()
        }
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            let assistant_msg = Message {
                id: Uuid::new_v4().to_string(),
                conversation_id,
                role: "assistant".to_string(),
                content: "No model profile configured. Please add a model profile in Settings → Models and set one as active.".to_string(),
                model_profile_id: None,
                status: "complete".to_string(),
                metadata: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            let mgr = state.conversation_manager.read().await;
            mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
            let _ = app.emit("chat:assistant-message", &assistant_msg);
            return Ok(assistant_msg);
        }
    };

    let provider_mgr = crate::models::ProviderManager::new(state.db.clone());
    let (api_key, base_url) = match provider_mgr.get_active_key() {
        Ok(Some(k)) => k,
        Ok(None) => {
            let assistant_msg = Message {
                id: Uuid::new_v4().to_string(),
                conversation_id,
                role: "assistant".to_string(),
                content: "No API key configured. Please add your OpenRouter API key in Settings → AI Providers.".to_string(),
                model_profile_id: None,
                status: "complete".to_string(),
                metadata: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            let mgr = state.conversation_manager.read().await;
            mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
            let _ = app.emit("chat:assistant-message", &assistant_msg);
            return Ok(assistant_msg);
        }
        Err(e) => return Err(e.to_string()),
    };

    let provider = match base_url {
        Some(url) => OpenRouterProvider::with_base_url(url),
        None => OpenRouterProvider::new(),
    };

    let history = {
        let mgr = state.conversation_manager.read().await;
        mgr.get_messages(&conversation_id).map_err(|e| e.to_string())?
    };

    let mut messages = Vec::new();
    if let Some(ref sp) = profile.system_prompt {
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: sp.clone(),
        });
    }

    let max_ctx = {
        let settings = state.settings.read().await;
        settings.get().ai.max_context_messages
    };

    let start = history.len().saturating_sub(max_ctx);
    for msg in &history[start..] {
        if msg.role == "user" || msg.role == "assistant" {
            messages.push(ChatMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
        }
    }

    let request = ChatCompletionRequest {
        model: profile.model_id.clone(),
        messages,
        temperature: Some(profile.temperature),
        max_tokens: Some(profile.max_tokens),
        stream: true,
    };

    // Emit thinking event
    let _ = app.emit("luna:state-change", "thinking");

    match provider.stream_chat_completion(request, &api_key).await {
        Ok(mut stream) => {
            let mut full_content = String::new();
            let msg_id = Uuid::new_v4().to_string();
            let conv_id = conversation_id.clone();
            let profile_id = profile.id.clone();

            while let Some(chunk) = stream.next().await {
                if CANCEL_FLAG.load(Ordering::Relaxed) {
                    let _ = app.emit("luna:state-change", "idle");
                    break;
                }

                match chunk {
                    Ok(text) => {
                        full_content.push_str(&text);
                        let _ = app.emit("chat:stream-chunk", serde_json::json!({
                            "conversation_id": conv_id,
                            "content": full_content,
                            "delta": text,
                        }));
                    }
                    Err(e) => {
                        log::error!("Stream error: {}", e);
                        let _ = app.emit("luna:state-change", "error");
                        break;
                    }
                }
            }

            if !full_content.is_empty() {
                let assistant_msg = Message {
                    id: msg_id,
                    conversation_id: conv_id,
                    role: "assistant".to_string(),
                    content: full_content,
                    model_profile_id: Some(profile_id),
                    status: "complete".to_string(),
                    metadata: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                };

                {
                    let mgr = state.conversation_manager.read().await;
                    mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
                }

                let _ = app.emit("chat:assistant-message", &assistant_msg);
                let _ = app.emit("luna:state-change", "idle");

                let audit = state.audit_log.read().await;
                let _ = audit.log(
                    "chat.send",
                    Some("openrouter"),
                    Some(&format!("model={}", profile.model_id)),
                    Some(&format!("{} chars", assistant_msg.content.len())),
                    true,
                    Some("allowed"),
                );

                Ok(assistant_msg)
            } else {
                let assistant_msg = Message {
                    id: msg_id,
                    conversation_id: conv_id,
                    role: "assistant".to_string(),
                    content: "No response received from the model.".to_string(),
                    model_profile_id: Some(profile_id),
                    status: "error".to_string(),
                    metadata: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                };

                {
                    let mgr = state.conversation_manager.read().await;
                    mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
                }

                let _ = app.emit("chat:assistant-message", &assistant_msg);
                let _ = app.emit("luna:state-change", "idle");

                Ok(assistant_msg)
            }
        }
        Err(e) => {
            let _ = app.emit("luna:state-change", "error");
            let assistant_msg = Message {
                id: Uuid::new_v4().to_string(),
                conversation_id,
                role: "assistant".to_string(),
                content: format!("OpenRouter error: {}", e),
                model_profile_id: Some(profile.id),
                status: "error".to_string(),
                metadata: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            let mgr = state.conversation_manager.read().await;
            mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
            let _ = app.emit("chat:assistant-message", &assistant_msg);
            Ok(assistant_msg)
        }
    }
}

#[tauri::command]
pub async fn delete_conversation(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.conversation_manager.read().await;
    mgr.delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_conversation(
    state: State<'_, AppState>,
    id: String,
    title: String,
) -> Result<(), String> {
    let mgr = state.conversation_manager.read().await;
    mgr.rename(&id, &title).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_generation(
    _state: State<'_, AppState>,
    _conversation_id: String,
) -> Result<(), String> {
    CANCEL_FLAG.store(true, Ordering::Relaxed);
    Ok(())
}
