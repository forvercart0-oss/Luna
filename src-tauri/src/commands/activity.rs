use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub id: String,
    pub event_type: String,
    pub category: String,
    pub title: String,
    pub detail: Option<String>,
    pub status: String,
    pub metadata: Option<String>,
    pub created_at: String,
}

#[tauri::command]
pub async fn create_activity_event(
    state: State<'_, AppState>,
    event_type: String,
    category: String,
    title: String,
    detail: Option<String>,
    status: Option<String>,
    metadata: Option<String>,
) -> Result<ActivityEvent, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let status = status.unwrap_or_else(|| "running".to_string());

    state.db.execute(
        "INSERT INTO activity_events (id, event_type, category, title, detail, status, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[
            &id as &dyn rusqlite::types::ToSql,
            &event_type,
            &category,
            &title,
            &detail,
            &status,
            &metadata,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(ActivityEvent {
        id,
        event_type,
        category,
        title,
        detail,
        status,
        metadata,
        created_at: now,
    })
}

#[tauri::command]
pub async fn get_activity_events(
    state: State<'_, AppState>,
    limit: Option<i64>,
    event_type: Option<String>,
) -> Result<Vec<ActivityEvent>, String> {
    let limit = limit.unwrap_or(100);

    let (sql, params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = if let Some(ref et) = event_type {
        (
            "SELECT id, event_type, category, title, detail, status, metadata, created_at FROM activity_events WHERE event_type = ?1 ORDER BY created_at DESC LIMIT ?2".to_string(),
            vec![Box::new(et.clone()), Box::new(limit)],
        )
    } else {
        (
            "SELECT id, event_type, category, title, detail, status, metadata, created_at FROM activity_events ORDER BY created_at DESC LIMIT ?1".to_string(),
            vec![Box::new(limit)],
        )
    };

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    state.db.query_map(&sql, &param_refs, |row| {
        Ok(ActivityEvent {
            id: row.get(0)?,
            event_type: row.get(1)?,
            category: row.get(2)?,
            title: row.get(3)?,
            detail: row.get(4)?,
            status: row.get(5)?,
            metadata: row.get(6)?,
            created_at: row.get(7)?,
        })
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_activity_event(
    state: State<'_, AppState>,
    id: String,
    status: String,
    detail: Option<String>,
) -> Result<(), String> {
    state.db.execute(
        "UPDATE activity_events SET status = ?1, detail = ?2 WHERE id = ?3",
        &[
            &status as &dyn rusqlite::types::ToSql,
            &detail,
            &id,
        ],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
