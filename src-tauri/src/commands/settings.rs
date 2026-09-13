use crate::settings::Settings;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let settings = state.settings.read().await;
    Ok(settings.get().clone())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings, String> {
    let mut mgr = state.settings.write().await;
    mgr.update(settings.clone()).map_err(|e| e.to_string())?;
    Ok(settings)
}
