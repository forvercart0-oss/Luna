use crate::tools::ToolDefinition;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_tool_list(
    state: State<'_, AppState>,
) -> Result<Vec<ToolDefinition>, String> {
    let registry = state.tool_registry.read().await;
    Ok(registry.get_all().into_iter().cloned().collect())
}
