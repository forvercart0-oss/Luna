use crate::api_catalog::{ApiCatalogEntry, CatalogSyncResult, CatalogStats, ApiToolDefinition};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn sync_catalog(
    state: State<'_, AppState>,
    readme_content: String,
) -> Result<CatalogSyncResult, String> {
    let catalog = state.api_catalog.read().await;
    catalog.sync_from_readme(&readme_content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_catalog(
    state: State<'_, AppState>,
    query: String,
    category: Option<String>,
    auth_filter: Option<String>,
) -> Result<Vec<ApiCatalogEntry>, String> {
    let catalog = state.api_catalog.read().await;
    catalog.search(&query, category.as_deref(), auth_filter.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_catalog_stats(
    state: State<'_, AppState>,
) -> Result<CatalogStats, String> {
    let catalog = state.api_catalog.read().await;
    catalog.get_stats().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_catalog_entry(
    state: State<'_, AppState>,
    id: String,
) -> Result<ApiCatalogEntry, String> {
    let catalog = state.api_catalog.read().await;
    catalog.get_by_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_catalog_entry(
    state: State<'_, AppState>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let catalog = state.api_catalog.read().await;
    catalog.toggle_enabled(&id, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let catalog = state.api_catalog.read().await;
    catalog.get_categories().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_api_tool(
    state: State<'_, AppState>,
    catalog_id: String,
    tool_name: String,
    tool_description: String,
    endpoint: String,
    method: String,
    parameters: Option<String>,
    auth_required: bool,
    category: String,
) -> Result<ApiToolDefinition, String> {
    let catalog = state.api_catalog.read().await;
    catalog.create_tool_definition(
        &catalog_id,
        &tool_name,
        &tool_description,
        &endpoint,
        &method,
        parameters.as_deref(),
        auth_required,
        &category,
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_api_tool(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let catalog = state.api_catalog.read().await;
    catalog.delete_tool_definition(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_api_tools(
    state: State<'_, AppState>,
    enabled_only: bool,
) -> Result<Vec<ApiToolDefinition>, String> {
    let catalog = state.api_catalog.read().await;
    catalog.get_tool_definitions(enabled_only).map_err(|e| e.to_string())
}
