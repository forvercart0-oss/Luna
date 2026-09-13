use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model_id: String,
    pub system_prompt: Option<String>,
    pub temperature: f64,
    pub max_tokens: i64,
    pub is_active: bool,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateModelProfile {
    pub name: String,
    pub provider: String,
    pub model_id: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModelProfile {
    pub name: Option<String>,
    pub provider: Option<String>,
    pub model_id: Option<String>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i64>,
    pub enabled: Option<bool>,
}

fn row_to_model(row: &rusqlite::Row<'_>) -> rusqlite::Result<ModelProfile> {
    Ok(ModelProfile {
        id: row.get(0)?,
        name: row.get(1)?,
        provider: row.get(2)?,
        model_id: row.get(3)?,
        system_prompt: row.get(4)?,
        temperature: row.get(5)?,
        max_tokens: row.get(6)?,
        is_active: row.get::<_, i64>(7)? != 0,
        enabled: row.get::<_, i64>(8)? != 0,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}

pub struct ModelManager {
    db: Arc<Database>,
}

impl ModelManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, input: CreateModelProfile) -> Result<ModelProfile> {
        let id = Uuid::new_v4().to_string();
        let temp = input.temperature.unwrap_or(0.7);
        let max_tok = input.max_tokens.unwrap_or(4096);

        self.db.execute(
            "INSERT INTO model_profiles (id, name, provider, model_id, system_prompt, temperature, max_tokens) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&id as &dyn rusqlite::types::ToSql, &input.name, &input.provider, &input.model_id, &input.system_prompt, &temp, &max_tok],
        )?;

        self.get_by_id(&id)
    }

    pub fn get_all(&self) -> Result<Vec<ModelProfile>> {
        self.db.query_map(
            "SELECT id, name, provider, model_id, system_prompt, temperature, max_tokens, is_active, enabled, created_at, updated_at FROM model_profiles ORDER BY created_at DESC",
            &[],
            row_to_model,
        )
    }

    pub fn get_by_id(&self, id: &str) -> Result<ModelProfile> {
        self.db.query_row(
            "SELECT id, name, provider, model_id, system_prompt, temperature, max_tokens, is_active, enabled, created_at, updated_at FROM model_profiles WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            row_to_model,
        )
    }

    pub fn get_active(&self) -> Result<Option<ModelProfile>> {
        match self.db.query_row(
            "SELECT id, name, provider, model_id, system_prompt, temperature, max_tokens, is_active, enabled, created_at, updated_at FROM model_profiles WHERE is_active = 1 LIMIT 1",
            &[],
            row_to_model,
        ) {
            Ok(p) => Ok(Some(p)),
            Err(e) => {
                if e.to_string().contains("QueryReturnedNoRows") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn set_active(&self, id: &str) -> Result<()> {
        self.db.execute("UPDATE model_profiles SET is_active = 0", &[])?;
        self.db.execute(
            "UPDATE model_profiles SET is_active = 1, updated_at = datetime('now') WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
        )?;
        Ok(())
    }

    pub fn update(&self, id: &str, input: UpdateModelProfile) -> Result<ModelProfile> {
        if let Some(ref name) = input.name {
            self.db.execute(
                "UPDATE model_profiles SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[name as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref provider) = input.provider {
            self.db.execute(
                "UPDATE model_profiles SET provider = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[provider as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref model_id) = input.model_id {
            self.db.execute(
                "UPDATE model_profiles SET model_id = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[model_id as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref sp) = input.system_prompt {
            self.db.execute(
                "UPDATE model_profiles SET system_prompt = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[sp as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(temp) = input.temperature {
            self.db.execute(
                "UPDATE model_profiles SET temperature = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&temp as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(mt) = input.max_tokens {
            self.db.execute(
                "UPDATE model_profiles SET max_tokens = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&mt as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(enabled) = input.enabled {
            let v: i64 = if enabled { 1 } else { 0 };
            self.db.execute(
                "UPDATE model_profiles SET enabled = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&v as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        self.get_by_id(id)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM model_profiles WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }
}

// ── Provider Account ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAccount {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub api_key_masked: String,
    pub base_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProviderAccount {
    pub provider: String,
    pub name: String,
    pub api_key: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProviderAccount {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    #[allow(dead_code)]
    pub is_active: Option<bool>,
}

pub struct ProviderManager {
    db: Arc<Database>,
}

impl ProviderManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, input: CreateProviderAccount) -> Result<ProviderAccount> {
        let id = Uuid::new_v4().to_string();
        let masked = mask_api_key(&input.api_key);

        self.db.execute(
            "INSERT INTO provider_accounts (id, provider, name, api_key_encrypted, base_url) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&id as &dyn rusqlite::types::ToSql, &input.provider, &input.name, &input.api_key, &input.base_url],
        )?;

        Ok(ProviderAccount {
            id,
            provider: input.provider,
            name: input.name,
            api_key_masked: masked,
            base_url: input.base_url,
            is_active: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub fn get_all(&self) -> Result<Vec<ProviderAccount>> {
        self.db.query_map(
            "SELECT id, provider, name, api_key_encrypted, base_url, is_active, created_at, updated_at FROM provider_accounts ORDER BY created_at DESC",
            &[],
            |row| {
                let api_key: String = row.get(3)?;
                Ok(ProviderAccount {
                    id: row.get(0)?,
                    provider: row.get(1)?,
                    name: row.get(2)?,
                    api_key_masked: mask_api_key(&api_key),
                    base_url: row.get(4)?,
                    is_active: row.get::<_, i64>(5)? != 0,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
    }

    pub fn get_active_key(&self) -> Result<Option<(String, Option<String>)>> {
        match self.db.query_row(
            "SELECT api_key_encrypted, base_url FROM provider_accounts WHERE is_active = 1 LIMIT 1",
            &[],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ) {
            Ok(pair) => Ok(Some(pair)),
            Err(e) => {
                if e.to_string().contains("QueryReturnedNoRows") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_active(&self, id: &str) -> Result<()> {
        self.db.execute("UPDATE provider_accounts SET is_active = 0", &[])?;
        self.db.execute(
            "UPDATE provider_accounts SET is_active = 1, updated_at = datetime('now') WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM provider_accounts WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }

    pub fn update(&self, id: &str, input: UpdateProviderAccount) -> Result<()> {
        if let Some(ref name) = input.name {
            self.db.execute(
                "UPDATE provider_accounts SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[name as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref api_key) = input.api_key {
            self.db.execute(
                "UPDATE provider_accounts SET api_key_encrypted = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[api_key as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(ref base_url) = input.base_url {
            self.db.execute(
                "UPDATE provider_accounts SET base_url = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[base_url as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_key_by_id(&self, id: &str) -> Result<String> {
        self.db.query_row(
            "SELECT api_key_encrypted FROM provider_accounts WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            |row| row.get(0),
        )
    }
}

fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        return "*".repeat(key.len());
    }
    let start = &key[..4];
    let end = &key[key.len() - 4..];
    format!("{}...{}", start, end)
}

// ── Conversation Manager ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model_profile_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub model_profile_id: Option<String>,
    pub status: String,
    pub metadata: Option<String>,
    pub created_at: String,
}

pub struct ConversationManager {
    db: Arc<Database>,
}

impl ConversationManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, title: Option<String>, model_profile_id: Option<String>) -> Result<Conversation> {
        let id = Uuid::new_v4().to_string();
        let title = title.unwrap_or_else(|| "New Conversation".to_string());

        self.db.execute(
            "INSERT INTO conversations (id, title, model_profile_id) VALUES (?1, ?2, ?3)",
            &[&id as &dyn rusqlite::types::ToSql, &title, &model_profile_id],
        )?;

        Ok(Conversation {
            id,
            title,
            model_profile_id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub fn get_all(&self) -> Result<Vec<Conversation>> {
        self.db.query_map(
            "SELECT id, title, model_profile_id, created_at, updated_at FROM conversations ORDER BY updated_at DESC",
            &[],
            |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model_profile_id: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM conversations WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }

    pub fn rename(&self, id: &str, title: &str) -> Result<()> {
        self.db.execute(
            "UPDATE conversations SET title = ?1, updated_at = datetime('now') WHERE id = ?2",
            &[&title as &dyn rusqlite::types::ToSql, &id],
        )?;
        Ok(())
    }

    pub fn add_message(&self, msg: &Message) -> Result<()> {
        self.db.execute(
            "INSERT INTO messages (id, conversation_id, role, content, model_profile_id, status, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&msg.id as &dyn rusqlite::types::ToSql, &msg.conversation_id, &msg.role, &msg.content, &msg.model_profile_id, &msg.status, &msg.metadata],
        )?;
        self.db.execute(
            "UPDATE conversations SET updated_at = datetime('now') WHERE id = ?1",
            &[&msg.conversation_id as &dyn rusqlite::types::ToSql],
        )?;
        Ok(())
    }

    pub fn get_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        self.db.query_map(
            "SELECT id, conversation_id, role, content, model_profile_id, status, metadata, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC",
            &[&conversation_id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(Message {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    model_profile_id: row.get(4)?,
                    status: row.get(5)?,
                    metadata: row.get(6)?,
                    created_at: row.get(7)?,
                })
            },
        )
    }

    pub fn update_message_content(&self, id: &str, content: &str, status: &str) -> Result<()> {
        self.db.execute(
            "UPDATE messages SET content = ?1, status = ?2 WHERE id = ?3",
            &[&content as &dyn rusqlite::types::ToSql, &status, &id],
        )?;
        Ok(())
    }
}
