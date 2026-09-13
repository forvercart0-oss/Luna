use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub auth_type: String,
    pub https: bool,
    pub cors: String,
    pub category: String,
    pub homepage: String,
    pub enabled: bool,
    pub health_status: String,
    pub last_health_check: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSyncResult {
    pub added: usize,
    pub updated: usize,
    pub removed: usize,
    pub total: usize,
    pub sync_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogStats {
    pub total_apis: usize,
    pub categories: Vec<CategoryCount>,
    pub last_synced: Option<String>,
    pub sync_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToolDefinition {
    pub id: String,
    pub catalog_id: String,
    pub tool_name: String,
    pub tool_description: String,
    pub endpoint: String,
    pub method: String,
    pub parameters: Option<String>,
    pub auth_required: bool,
    pub enabled: bool,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ApiCatalogManager {
    db: Arc<Database>,
}

impl ApiCatalogManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn sync_from_readme(&self, readme_content: &str) -> Result<CatalogSyncResult> {
        let entries = parse_public_apis_readme(readme_content)?;
        let mut added = 0;
        let mut updated = 0;

        let existing: Vec<String> = self.db.query_map(
            "SELECT id FROM api_catalog",
            &[],
            |row| row.get::<_, String>(0),
        ).unwrap_or_default();
        let existing_set: std::collections::HashSet<String> = existing.into_iter().collect();
        let mut seen_ids = std::collections::HashSet::new();

        for entry in entries {
            let id = format_api_id(&entry.name, &entry.category);
            seen_ids.insert(id.clone());

            let exists = existing_set.contains(&id);
            if exists {
                self.db.execute(
                    "UPDATE api_catalog SET name = ?1, description = ?2, auth_type = ?3, https = ?4, cors = ?5, homepage = ?6, updated_at = datetime('now') WHERE id = ?7",
                    &[
                        &entry.name as &dyn rusqlite::types::ToSql,
                        &entry.description,
                        &entry.auth,
                        &(entry.https as i64),
                        &entry.cors,
                        &entry.url,
                        &id,
                    ],
                )?;
                updated += 1;
            } else {
                let id_clone = id.clone();
                self.db.execute(
                    "INSERT INTO api_catalog (id, name, description, auth_type, https, cors, category, homepage) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    &[
                        &id_clone as &dyn rusqlite::types::ToSql,
                        &entry.name,
                        &entry.description,
                        &entry.auth,
                        &(entry.https as i64),
                        &entry.cors,
                        &entry.category,
                        &entry.url,
                    ],
                )?;
                added += 1;
            }
        }

        // Remove entries not in current sync
        let current_ids: Vec<String> = self.db.query_map(
            "SELECT id FROM api_catalog",
            &[],
            |row| row.get::<_, String>(0),
        ).unwrap_or_default();

        let mut removed = 0;
        for id in &current_ids {
            if !seen_ids.contains(id.as_str()) {
                self.db.execute("DELETE FROM api_catalog WHERE id = ?1", &[id as &dyn rusqlite::types::ToSql])?;
                removed += 1;
            }
        }

        let total = self.db.query_row(
            "SELECT COUNT(*) FROM api_catalog",
            &[],
            |row| row.get::<_, usize>(0),
        ).unwrap_or(0);

        let now = chrono::Utc::now().to_rfc3339();
        self.set_meta("last_synced", &now)?;
        self.set_meta("total_apis", &total.to_string())?;

        Ok(CatalogSyncResult {
            added,
            updated,
            removed,
            total,
            sync_time: now,
        })
    }

    pub fn search(&self, query: &str, category: Option<&str>, auth_filter: Option<&str>) -> Result<Vec<ApiCatalogEntry>> {
        let mut sql = "SELECT id, name, description, auth_type, https, cors, category, homepage, enabled, health_status, last_health_check, created_at, updated_at FROM api_catalog WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if !query.is_empty() {
            sql.push_str(" AND (name LIKE ?1 OR description LIKE ?1 OR category LIKE ?1)");
            params.push(Box::new(format!("%{}%", query)));
        }

        if let Some(cat) = category {
            if !cat.is_empty() {
                sql.push_str(" AND category = ?");
                params.push(Box::new(cat.to_string()));
            }
        }

        if let Some(auth) = auth_filter {
            if !auth.is_empty() {
                sql.push_str(" AND auth_type = ?");
                params.push(Box::new(auth.to_string()));
            }
        }

        sql.push_str(" ORDER BY name ASC LIMIT 200");

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        self.db.query_map(&sql, &param_refs, |row| {
            Ok(ApiCatalogEntry {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                auth_type: row.get(3)?,
                https: row.get::<_, i64>(4)? != 0,
                cors: row.get(5)?,
                category: row.get(6)?,
                homepage: row.get(7)?,
                enabled: row.get::<_, i64>(8)? != 0,
                health_status: row.get(9)?,
                last_health_check: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })
    }

    pub fn get_by_id(&self, id: &str) -> Result<ApiCatalogEntry> {
        self.db.query_row(
            "SELECT id, name, description, auth_type, https, cors, category, homepage, enabled, health_status, last_health_check, created_at, updated_at FROM api_catalog WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(ApiCatalogEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    auth_type: row.get(3)?,
                    https: row.get::<_, i64>(4)? != 0,
                    cors: row.get(5)?,
                    category: row.get(6)?,
                    homepage: row.get(7)?,
                    enabled: row.get::<_, i64>(8)? != 0,
                    health_status: row.get(9)?,
                    last_health_check: row.get(10)?,
                    created_at: row.get(11)?,
                    updated_at: row.get(12)?,
                })
            },
        )
    }

    pub fn toggle_enabled(&self, id: &str, enabled: bool) -> Result<()> {
        let v: i64 = if enabled { 1 } else { 0 };
        self.db.execute(
            "UPDATE api_catalog SET enabled = ?1, updated_at = datetime('now') WHERE id = ?2",
            &[&v as &dyn rusqlite::types::ToSql, &id],
        )?;
        Ok(())
    }

    pub fn update_health(&self, id: &str, status: &str, response_time_ms: Option<i64>, error: Option<&str>) -> Result<()> {
        let log_id = Uuid::new_v4().to_string();
        self.db.execute(
            "INSERT INTO api_health_log (id, catalog_id, status, response_time_ms, error_message) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &log_id as &dyn rusqlite::types::ToSql,
                &id,
                &status,
                &response_time_ms,
                &error,
            ],
        )?;
        self.db.execute(
            "UPDATE api_catalog SET health_status = ?1, last_health_check = datetime('now'), updated_at = datetime('now') WHERE id = ?2",
            &[&status as &dyn rusqlite::types::ToSql, &id],
        )?;
        Ok(())
    }

    pub fn get_stats(&self) -> Result<CatalogStats> {
        let total = self.db.query_row(
            "SELECT COUNT(*) FROM api_catalog",
            &[],
            |row| row.get::<_, usize>(0),
        ).unwrap_or(0);

        let categories = self.db.query_map(
            "SELECT category, COUNT(*) as cnt FROM api_catalog GROUP BY category ORDER BY cnt DESC",
            &[],
            |row| {
                Ok(CategoryCount {
                    category: row.get(0)?,
                    count: row.get::<_, usize>(1)?,
                })
            },
        ).unwrap_or_default();

        let last_synced = self.get_meta("last_synced").ok();
        let sync_commit = self.get_meta("sync_commit").ok();

        Ok(CatalogStats {
            total_apis: total,
            categories,
            last_synced,
            sync_commit,
        })
    }

    pub fn get_categories(&self) -> Result<Vec<String>> {
        self.db.query_map(
            "SELECT DISTINCT category FROM api_catalog ORDER BY category",
            &[],
            |row| row.get::<_, String>(0),
        )
    }

    // Tool definitions from catalog
    pub fn get_tool_definitions(&self, enabled_only: bool) -> Result<Vec<ApiToolDefinition>> {
        let sql = if enabled_only {
            "SELECT id, catalog_id, tool_name, tool_description, endpoint, method, parameters, auth_required, enabled, category, created_at, updated_at FROM api_tool_definitions WHERE enabled = 1 ORDER BY tool_name"
        } else {
            "SELECT id, catalog_id, tool_name, tool_description, endpoint, method, parameters, auth_required, enabled, category, created_at, updated_at FROM api_tool_definitions ORDER BY tool_name"
        };

        self.db.query_map(sql, &[], |row| {
            Ok(ApiToolDefinition {
                id: row.get(0)?,
                catalog_id: row.get(1)?,
                tool_name: row.get(2)?,
                tool_description: row.get(3)?,
                endpoint: row.get(4)?,
                method: row.get(5)?,
                parameters: row.get(6)?,
                auth_required: row.get::<_, i64>(7)? != 0,
                enabled: row.get::<_, i64>(8)? != 0,
                category: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
    }

    pub fn create_tool_definition(
        &self,
        catalog_id: &str,
        tool_name: &str,
        tool_description: &str,
        endpoint: &str,
        method: &str,
        parameters: Option<&str>,
        auth_required: bool,
        category: &str,
    ) -> Result<ApiToolDefinition> {
        let id = Uuid::new_v4().to_string();
        let auth_i64 = if auth_required { 1 } else { 0 };
        let now = chrono::Utc::now().to_rfc3339();

        self.db.execute(
            "INSERT INTO api_tool_definitions (id, catalog_id, tool_name, tool_description, endpoint, method, parameters, auth_required, category) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[
                &id as &dyn rusqlite::types::ToSql,
                &catalog_id,
                &tool_name,
                &tool_description,
                &endpoint,
                &method,
                &parameters,
                &auth_i64,
                &category,
            ],
        )?;

        Ok(ApiToolDefinition {
            id,
            catalog_id: catalog_id.to_string(),
            tool_name: tool_name.to_string(),
            tool_description: tool_description.to_string(),
            endpoint: endpoint.to_string(),
            method: method.to_string(),
            parameters: parameters.map(|s| s.to_string()),
            auth_required,
            enabled: true,
            category: category.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn delete_tool_definition(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM api_tool_definitions WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }

    fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.db.execute(
            "INSERT OR REPLACE INTO api_catalog_meta (key, value, updated_at) VALUES (?1, ?2, datetime('now'))",
            &[&key as &dyn rusqlite::types::ToSql, &value],
        )?;
        Ok(())
    }

    fn get_meta(&self, key: &str) -> Result<String> {
        self.db.query_row(
            "SELECT value FROM api_catalog_meta WHERE key = ?1",
            &[&key as &dyn rusqlite::types::ToSql],
            |row| row.get(0),
        )
    }
}

fn format_api_id(name: &str, category: &str) -> String {
    let slug = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("{}.{}", category.to_lowercase(), slug)
}

#[derive(Debug, Deserialize)]
struct RawApiEntry {
    name: String,
    description: String,
    auth: String,
    https: bool,
    cors: String,
    category: String,
    url: String,
}

pub fn parse_public_apis_readme(content: &str) -> Result<Vec<RawApiEntry>> {
    let mut entries = Vec::new();
    let mut current_category = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("### ") {
            current_category = trimmed[4..].trim().to_string();
            continue;
        }

        if trimmed.starts_with("| ") && !trimmed.starts_with("| Name")
            && !trimmed.starts_with("|---")
        {
            let parts: Vec<&str> = trimmed.split('|').collect();
            if parts.len() >= 7 {
                let name_part = parts[1].trim();
                let desc = parts[2].trim();
                let auth = parts[3].trim();
                let https = parts[4].trim();
                let cors = parts[5].trim();
                let url = parts[6].trim();

                if !name_part.is_empty() && !name_part.starts_with('[') == false || name_part.contains(']') {
                    let name = extract_link_text(name_part);
                    let homepage = extract_link_url(name_part).unwrap_or_else(|| url.to_string());

                    let auth_type = normalize_auth_type(auth);
                    let https_bool = https.to_lowercase() == "yes";

                    entries.push(RawApiEntry {
                        name,
                        description: desc.to_string(),
                        auth: auth_type,
                        https: https_bool,
                        cors: cors.to_string(),
                        category: current_category.clone(),
                        url: homepage,
                    });
                }
            }
        }
    }

    Ok(entries)
}

fn extract_link_text(s: &str) -> String {
    if let Some(start) = s.find('[') {
        if let Some(end) = s.find(']') {
            return s[start + 1..end].to_string();
        }
    }
    s.trim().to_string()
}

fn extract_link_url(s: &str) -> Option<String> {
    if let Some(start) = s.find("](") {
        if let Some(end) = s[start + 2..].find(')') {
            return Some(s[start + 2..start + 2 + end].to_string());
        }
    }
    None
}

fn normalize_auth_type(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("apikey") || lower.contains("api key") || lower.contains("x-api-key") {
        "apiKey".to_string()
    } else if lower.contains("oauth") {
        "oauth".to_string()
    } else if lower.contains("bearer") || lower.contains("token") {
        "bearer".to_string()
    } else if lower == "no" || lower == "none" || lower.is_empty() {
        "none".to_string()
    } else {
        raw.to_string()
    }
}
