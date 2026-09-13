use crate::tools::{ToolCall, ToolRegistry};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_tool_list(
    state: State<'_, AppState>,
) -> Result<Vec<crate::tools::ToolDefinition>, String> {
    let registry = state.tool_registry.read().await;
    Ok(registry.get_all().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn execute_tool(
    state: State<'_, AppState>,
    tool_id: String,
    arguments: serde_json::Value,
) -> Result<String, String> {
    let registry = state.tool_registry.read().await;
    let perm_mgr = state.permission_manager.read().await;

    let call = ToolCall {
        id: uuid::Uuid::new_v4().to_string(),
        tool_id: tool_id.clone(),
        arguments,
    };

    let result = registry.execute(&call, &perm_mgr).await.map_err(|e| e.to_string())?;

    let audit = state.audit_log.read().await;
    let _ = audit.log(
        "tool.execute",
        Some(&tool_id),
        Some(&format!("tool_id={}", call.id)),
        Some(&format!("{} chars, success={}", result.output.len(), result.success)),
        result.success,
        Some("allowed"),
    );

    Ok(serde_json::to_string(&serde_json::json!({
        "success": result.success,
        "output": result.output,
        "error": result.error,
    })).unwrap_or_default())
}
