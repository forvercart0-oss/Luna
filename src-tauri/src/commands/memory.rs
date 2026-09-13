use crate::memory::{CreateMemory, Memory, UpdateMemory};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_memory(
    state: State<'_, AppState>,
    r#type: String,
    content: String,
    importance: Option<i64>,
    source: Option<String>,
    tags: Option<String>,
) -> Result<Memory, String> {
    let mgr = state.memory_manager.read().await;
    mgr.create(CreateMemory {
        r#type,
        content,
        importance,
        source,
        tags,
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_memories(
    state: State<'_, AppState>,
) -> Result<Vec<Memory>, String> {
    let mgr = state.memory_manager.read().await;
    mgr.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_memories(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<Memory>, String> {
    let mgr = state.memory_manager.read().await;
    mgr.search(&query).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_memory(
    state: State<'_, AppState>,
    id: String,
    r#type: Option<String>,
    content: Option<String>,
    importance: Option<i64>,
    source: Option<String>,
    tags: Option<String>,
) -> Result<Memory, String> {
    let mgr = state.memory_manager.read().await;
    mgr.update(
        &id,
        UpdateMemory {
            r#type,
            content,
            importance,
            source,
            tags,
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_memory(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mgr = state.memory_manager.read().await;
    mgr.delete(&id).map_err(|e| e.to_string())
}
