use crate::models::{CreateProviderAccount, OpenRouterModel, ProviderAccount, UpdateProviderAccount};
use crate::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
}

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
) -> Result<ProviderAccount, String> {
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

#[tauri::command]
pub async fn test_provider_account(
    state: State<'_, AppState>,
    id: String,
) -> Result<ConnectionTestResult, String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    let api_key = mgr
        .get_key_by_id(&id)
        .map_err(|e| e.to_string())?;

    let api_key = match api_key {
        Some(k) => k,
        None => return Ok(ConnectionTestResult { success: false, message: "Account not found".to_string() }),
    };

    let base_url: Option<String> = state.db.query_row(
        "SELECT base_url FROM provider_accounts WHERE id = ?1",
        &[&id as &dyn rusqlite::types::ToSql],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let base_url = base_url.unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string());

    let provider = crate::providers::openrouter::OpenRouterProvider::with_base_url(base_url.clone());

    match provider.test_connection(&api_key).await {
        Ok(true) => Ok(ConnectionTestResult { success: true, message: "Connected successfully".to_string() }),
        Ok(false) => Ok(ConnectionTestResult { success: false, message: "Connection failed - check your API key".to_string() }),
        Err(e) => Ok(ConnectionTestResult { success: false, message: format!("Connection error: {}", e) }),
    }
}

#[tauri::command]
pub async fn fetch_provider_models(
    state: State<'_, AppState>,
    id: String,
) -> Result<Vec<OpenRouterModel>, String> {
    let mgr = crate::models::ProviderManager::new(state.db.clone());
    let api_key = mgr
        .get_key_by_id(&id)
        .map_err(|e| e.to_string())?;

    let api_key = match api_key {
        Some(k) => k,
        None => return Err("Account not found".to_string()),
    };

    let base_url: Option<String> = state.db.query_row(
        "SELECT base_url FROM provider_accounts WHERE id = ?1",
        &[&id as &dyn rusqlite::types::ToSql],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let base_url = base_url.unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string());
    let provider = crate::providers::openrouter::OpenRouterProvider::with_base_url(base_url);

    provider.fetch_models(&api_key).await.map_err(|e| e.to_string())
}
