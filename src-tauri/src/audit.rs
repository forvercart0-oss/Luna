use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: String,
    pub action: String,
    pub tool_id: Option<String>,
    pub arguments_summary: Option<String>,
    pub result_summary: Option<String>,
    pub success: bool,
    pub permission_state: Option<String>,
    pub metadata: Option<String>,
}

pub struct AuditLog {
    db: Arc<Database>,
}

impl AuditLog {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn log(
        &self,
        action: &str,
        tool_id: Option<&str>,
        args_summary: Option<&str>,
        result_summary: Option<&str>,
        success: bool,
        permission_state: Option<&str>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let success_i64: i64 = if success { 1 } else { 0 };
        self.db.execute(
            "INSERT INTO audit_logs (id, action, tool_id, arguments_summary, result_summary, success, permission_state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&id as &dyn rusqlite::types::ToSql, &action, &tool_id, &args_summary, &result_summary, &success_i64, &permission_state],
        )?;
        Ok(())
    }

    pub fn get_logs(&self, limit: Option<i64>) -> Result<Vec<AuditEntry>> {
        let limit = limit.unwrap_or(100);
        self.db.query_map(
            "SELECT id, timestamp, action, tool_id, arguments_summary, result_summary, success, permission_state, metadata FROM audit_logs ORDER BY timestamp DESC LIMIT ?1",
            &[&limit as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(AuditEntry {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    action: row.get(2)?,
                    tool_id: row.get(3)?,
                    arguments_summary: row.get(4)?,
                    result_summary: row.get(5)?,
                    success: row.get::<_, i64>(6)? != 0,
                    permission_state: row.get(7)?,
                    metadata: row.get(8)?,
                })
            },
        )
    }
}
