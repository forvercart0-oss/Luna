use crate::models::{Conversation, Message};
use crate::providers::openrouter::OpenRouterProvider;
use crate::providers::{ChatCompletionRequest, ChatMessage};
use crate::AppState;
use tauri::State;
use uuid::Uuid;

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
    conversation_id: String,
    content: String,
    model_profile_id: Option<String>,
) -> Result<Message, String> {
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
        stream: false,
    };

    let response = provider
        .chat_completion(request, &api_key)
        .await
        .map_err(|e| e.to_string())?;

    let assistant_content = response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    let assistant_msg = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id,
        role: "assistant".to_string(),
        content: assistant_content,
        model_profile_id: Some(profile.id),
        status: "complete".to_string(),
        metadata: response.usage.map(|u| serde_json::to_string(&u).ok()).flatten(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    {
        let mgr = state.conversation_manager.read().await;
        mgr.add_message(&assistant_msg).map_err(|e| e.to_string())?;
    }

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
    Err("Cancel generation not yet implemented for non-streaming mode".to_string())
}
