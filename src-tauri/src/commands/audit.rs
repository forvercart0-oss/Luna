use crate::audit::AuditEntry;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_audit_logs(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<AuditEntry>, String> {
    let log = state.audit_log.read().await;
    log.get_logs(limit).map_err(|e| e.to_string())
}
