use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub r#type: String,
    pub content: String,
    pub importance: i64,
    pub source: Option<String>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemory {
    pub r#type: String,
    pub content: String,
    pub importance: Option<i64>,
    pub source: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemory {
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub importance: Option<i64>,
    pub source: Option<String>,
    pub tags: Option<String>,
}

fn row_to_memory(row: &rusqlite::Row<'_>) -> rusqlite::Result<Memory> {
    Ok(Memory {
        id: row.get(0)?,
        r#type: row.get(1)?,
        content: row.get(2)?,
        importance: row.get(3)?,
        source: row.get(4)?,
        tags: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

pub struct MemoryManager {
    db: Arc<Database>,
}

impl MemoryManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, input: CreateMemory) -> Result<Memory> {
        let id = Uuid::new_v4().to_string();
        let importance = input.importance.unwrap_or(5);

        self.db.execute(
            "INSERT INTO memories (id, type, content, importance, source, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            &[&id as &dyn rusqlite::types::ToSql, &input.r#type, &input.content, &importance, &input.source, &input.tags],
        )?;

        self.get_by_id(&id)
    }

    pub fn get_all(&self) -> Result<Vec<Memory>> {
        self.db.query_map(
            "SELECT id, type, content, importance, source, tags, created_at, updated_at FROM memories ORDER BY importance DESC, created_at DESC",
            &[],
            row_to_memory,
        )
    }

    pub fn search(&self, query: &str) -> Result<Vec<Memory>> {
        let pattern = format!("%{}%", query);
        self.db.query_map(
            "SELECT id, type, content, importance, source, tags, created_at, updated_at FROM memories WHERE content LIKE ?1 OR tags LIKE ?1 ORDER BY importance DESC",
            &[&pattern as &dyn rusqlite::types::ToSql],
            row_to_memory,
        )
    }

    pub fn get_relevant(&self, context_keywords: &[String], limit: usize) -> Result<Vec<Memory>> {
        if context_keywords.is_empty() {
            return self.db.query_map(
                "SELECT id, type, content, importance, source, tags, created_at, updated_at FROM memories ORDER BY importance DESC LIMIT ?1",
                &[&(limit as i64) as &dyn rusqlite::types::ToSql],
                row_to_memory,
            );
        }

        let mut all_memories = Vec::new();
        for keyword in context_keywords {
            let pattern = format!("%{}%", keyword);
            let memories: Vec<Memory> = self.db.query_map(
                "SELECT id, type, content, importance, source, tags, created_at, updated_at FROM memories WHERE content LIKE ?1 OR tags LIKE ?1",
                &[&pattern as &dyn rusqlite::types::ToSql],
                row_to_memory,
            )?;
            all_memories.extend(memories);
        }

        all_memories.sort_by(|a, b| b.importance.cmp(&a.importance));
        all_memories.truncate(limit);
        Ok(all_memories)
    }

    pub fn get_by_id(&self, id: &str) -> Result<Memory> {
        self.db.query_row(
            "SELECT id, type, content, importance, source, tags, created_at, updated_at FROM memories WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            row_to_memory,
        )
    }

    pub fn update(&self, id: &str, input: UpdateMemory) -> Result<Memory> {
        if let Some(ref r#type) = input.r#type {
            self.db.execute(
                "UPDATE memories SET type = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[r#type as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref content) = input.content {
            self.db.execute(
                "UPDATE memories SET content = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[content as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(importance) = input.importance {
            self.db.execute(
                "UPDATE memories SET importance = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&importance as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref source) = input.source {
            self.db.execute(
                "UPDATE memories SET source = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[source as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref tags) = input.tags {
            self.db.execute(
                "UPDATE memories SET tags = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[tags as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        self.get_by_id(id)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM memories WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }
}
