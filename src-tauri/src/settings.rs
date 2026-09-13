use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub ai: AiSettings,
    pub voice: VoiceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub default_provider: String,
    pub auto_save_conversations: bool,
    pub memory_enabled: bool,
    pub max_context_messages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub tts_enabled: bool,
    pub stt_enabled: bool,
    pub voice: String,
    pub speed: f64,
    pub volume: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                theme: "dark".to_string(),
                language: "en".to_string(),
            },
            ai: AiSettings {
                default_provider: "openrouter".to_string(),
                auto_save_conversations: true,
                memory_enabled: true,
                max_context_messages: 20,
            },
            voice: VoiceSettings {
                tts_enabled: false,
                stt_enabled: false,
                voice: "default".to_string(),
                speed: 1.0,
                volume: 1.0,
            },
        }
    }
}

pub struct SettingsManager {
    db: Arc<Database>,
    cache: Settings,
}

impl SettingsManager {
    pub fn new(db: Arc<Database>) -> Self {
        let cache = Self::load_from_db(&db).unwrap_or_default();
        Self { db, cache }
    }

    pub fn get(&self) -> &Settings {
        &self.cache
    }

    pub fn update(&mut self, settings: Settings) -> Result<()> {
        let json = serde_json::to_string(&settings)?;
        self.db.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('app_settings', ?1, datetime('now'))",
            &[&json as &dyn rusqlite::types::ToSql],
        )?;
        self.cache = settings;
        Ok(())
    }

    fn load_from_db(db: &Database) -> Result<Settings> {
        let json: String = db.query_row(
            "SELECT value FROM settings WHERE key = 'app_settings'",
            &[],
            |row| row.get(0),
        )?;
        Ok(serde_json::from_str(&json)?)
    }
}
