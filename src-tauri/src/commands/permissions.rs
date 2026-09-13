use crate::permissions::Permission;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_permissions(
    state: State<'_, AppState>,
) -> Result<Vec<Permission>, String> {
    let mgr = state.permission_manager.read().await;
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_permission(
    state: State<'_, AppState>,
    tool_id: String,
    scope: Option<String>,
    permission: String,
) -> Result<(), String> {
    let mgr = state.permission_manager.read().await;
    mgr.update(&tool_id, scope.as_deref(), &permission)
        .map_err(|e| e.to_string())
}
