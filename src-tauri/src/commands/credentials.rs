use crate::api_connector::{ApiCredential, ApiCredentialTest, ApiConnector};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_credential(
    state: State<'_, AppState>,
    provider_id: String,
    credential_type: String,
    credential_key: String,
    credential_value: String,
    label: Option<String>,
) -> Result<ApiCredential, String> {
    let mgr = state.api_credentials.read().await;
    mgr.create(
        &provider_id,
        &credential_type,
        &credential_key,
        &credential_value,
        label.as_deref(),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_credentials(
    state: State<'_, AppState>,
) -> Result<Vec<ApiCredential>, String> {
    let mgr = state.api_credentials.read().await;
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_credentials_for_provider(
    state: State<'_, AppState>,
    provider_id: String,
) -> Result<Vec<ApiCredential>, String> {
    let mgr = state.api_credentials.read().await;
    mgr.get_for_provider(&provider_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_credential(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.api_credentials.read().await;
    mgr.delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_active_credential(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.api_credentials.read().await;
    mgr.set_active(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_credential(
    state: State<'_, AppState>,
    id: String,
    test_url: Option<String>,
) -> Result<ApiCredentialTest, String> {
    let mgr = state.api_credentials.read().await;
    let value = mgr.get_decrypted_value(&id).map_err(|e| e.to_string())?;

    if let Some(url) = test_url {
        let connector = ApiConnector::new();
        match connector.execute_request(
            "GET",
            &url,
            Some(&[("Authorization", &format!("Bearer {}", value))]),
            None,
        ).await {
            Ok(resp) => Ok(ApiCredentialTest {
                success: resp.status >= 200 && resp.status < 300,
                message: format!("HTTP {} - {}ms", resp.status, resp.response_time_ms),
                response_time_ms: Some(resp.response_time_ms),
            }),
            Err(e) => Ok(ApiCredentialTest {
                success: false,
                message: format!("Connection failed: {}", e),
                response_time_ms: None,
            }),
        }
    } else {
        Ok(ApiCredentialTest {
            success: !value.is_empty(),
            message: if value.is_empty() { "No credential value".to_string() } else { "Credential stored".to_string() },
            response_time_ms: None,
        })
    }
}

#[tauri::command]
pub async fn update_credential(
    state: State<'_, AppState>,
    id: String,
    label: Option<String>,
    credential_value: Option<String>,
) -> Result<(), String> {
    let mgr = state.api_credentials.read().await;
    mgr.update(&id, label.as_deref(), credential_value.as_deref())
        .map_err(|e| e.to_string())
}
