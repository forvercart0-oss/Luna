use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::types::ToSql]) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.execute(sql, params)?)
    }

    pub fn query_row<T, F>(&self, sql: &str, params: &[&dyn rusqlite::types::ToSql], f: F) -> Result<T>
    where
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(sql, params, f)?)
    }

    pub fn query_map<T, F>(&self, sql: &str, params: &[&dyn rusqlite::types::ToSql], f: F) -> Result<Vec<T>>
    where
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, f)?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;

        let migrations: Vec<(&str, &str)> = vec![
            ("001_settings", include_str!("../migrations/001_settings.sql")),
            ("002_providers", include_str!("../migrations/002_providers.sql")),
            ("003_models", include_str!("../migrations/003_models.sql")),
            ("004_conversations", include_str!("../migrations/004_conversations.sql")),
            ("005_memories", include_str!("../migrations/005_memories.sql")),
            ("006_permissions", include_str!("../migrations/006_permissions.sql")),
            ("007_audit", include_str!("../migrations/007_audit.sql")),
            ("008_tasks", include_str!("../migrations/008_tasks.sql")),
            ("009_api_catalog", include_str!("../migrations/009_api_catalog.sql")),
        ];

        for (name, sql) in migrations {
            let already_applied: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM migrations WHERE name = ?1",
                    rusqlite::params![name],
                    |row| row.get(0),
                )
                .unwrap_or(false);

            if !already_applied {
                conn.execute_batch(sql)?;
                conn.execute(
                    "INSERT INTO migrations (name) VALUES (?1)",
                    rusqlite::params![name],
                )?;
                log::info!("Applied migration: {}", name);
            }
        }

        Ok(())
    }
}
