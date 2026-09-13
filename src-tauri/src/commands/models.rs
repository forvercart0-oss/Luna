use crate::models::{
    CreateModelProfile, ModelProfile, UpdateModelProfile,
};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_model_profile(
    state: State<'_, AppState>,
    name: String,
    provider: String,
    model_id: String,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<i64>,
) -> Result<ModelProfile, String> {
    let mgr = state.model_manager.read().await;
    mgr.create(CreateModelProfile {
        name,
        provider,
        model_id,
        system_prompt,
        temperature,
        max_tokens,
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_model_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<ModelProfile>, String> {
    let mgr = state.model_manager.read().await;
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_model_profile(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    provider: Option<String>,
    model_id: Option<String>,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<i64>,
    enabled: Option<bool>,
) -> Result<ModelProfile, String> {
    let mgr = state.model_manager.read().await;
    mgr.update(
        &id,
        UpdateModelProfile {
            name,
            provider,
            model_id,
            system_prompt,
            temperature,
            max_tokens,
            enabled,
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_model_profile(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.model_manager.read().await;
    mgr.delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_active_model_profile(
    state: State<'_, AppState>,
) -> Result<Option<ModelProfile>, String> {
    let mgr = state.model_manager.read().await;
    mgr.get_active().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_active_model_profile(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.model_manager.read().await;
    mgr.set_active(&id).map_err(|e| e.to_string())
}
