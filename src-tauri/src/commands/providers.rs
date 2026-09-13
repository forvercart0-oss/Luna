use crate::models::{CreateProviderAccount, ProviderAccount, UpdateProviderAccount};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn add_provider_account(
    state: State<'_, AppState>,
    name: String,
    api_key: String,
    base_url: Option<String>,
) -> Result<ProviderAccount, String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    mgr.create(CreateProviderAccount {
        provider: "openrouter".to_string(),
        name,
        api_key,
        base_url,
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_provider_accounts(
    state: State<'_, AppState>,
) -> Result<Vec<ProviderAccount>, String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_provider_account(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    mgr.delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_provider_account(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    api_key: Option<String>,
    base_url: Option<String>,
    is_active: Option<bool>,
) -> Result<(), String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    mgr.update(
        &id,
        UpdateProviderAccount {
            name,
            api_key,
            base_url,
            is_active,
        },
    )
    .map_err(|e| e.to_string())
}
