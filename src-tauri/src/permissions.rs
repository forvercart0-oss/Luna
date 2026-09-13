use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub tool_id: String,
    pub permission: String,
    pub scope: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct PermissionManager {
    db: Arc<Database>,
}

impl PermissionManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn check(&self, tool_id: &str, scope: Option<&str>) -> Result<String> {
        if let Some(s) = scope {
            let result: Option<String> = self.db.query_row(
                "SELECT permission FROM tool_permissions WHERE tool_id = ?1 AND scope = ?2",
                &[&tool_id as &dyn rusqlite::types::ToSql, &s],
                |row| row.get(0),
            ).ok();
            if let Some(p) = result {
                return Ok(p);
            }
        }

        let result: Option<String> = self.db.query_row(
            "SELECT permission FROM tool_permissions WHERE tool_id = ?1 AND scope IS NULL",
            &[&tool_id as &dyn rusqlite::types::ToSql],
            |row| row.get(0),
        ).ok();

        Ok(result.unwrap_or_else(|| "ask".to_string()))
    }

    pub fn get_all(&self) -> Result<Vec<Permission>> {
        self.db.query_map(
            "SELECT id, tool_id, permission, scope, created_at, updated_at FROM tool_permissions ORDER BY tool_id",
            &[],
            |row| {
                Ok(Permission {
                    id: row.get(0)?,
                    tool_id: row.get(1)?,
                    permission: row.get(2)?,
                    scope: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
    }

    pub fn update(&self, tool_id: &str, scope: Option<&str>, permission: &str) -> Result<()> {
        let existing: Option<String> = self.db.query_row(
            "SELECT id FROM tool_permissions WHERE tool_id = ?1 AND ((scope = ?2) OR (scope IS NULL AND ?2 IS NULL))",
            &[&tool_id as &dyn rusqlite::types::ToSql, &scope],
            |row| row.get(0),
        ).ok();

        if let Some(existing_id) = existing {
            self.db.execute(
                "UPDATE tool_permissions SET permission = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&permission as &dyn rusqlite::types::ToSql, &existing_id],
            )?;
        } else {
            let id = Uuid::new_v4().to_string();
            self.db.execute(
                "INSERT INTO tool_permissions (id, tool_id, permission, scope) VALUES (?1, ?2, ?3, ?4)",
                &[&id as &dyn rusqlite::types::ToSql, &tool_id, &permission, &scope],
            )?;
        }
        Ok(())
    }
}
