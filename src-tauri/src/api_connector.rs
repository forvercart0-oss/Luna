use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredential {
    pub id: String,
    pub provider_id: String,
    pub credential_type: String,
    pub credential_key: String,
    pub credential_value_encrypted: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredentialTest {
    pub success: bool,
    pub message: String,
    pub response_time_ms: Option<i64>,
}

pub struct ApiCredentialManager {
    db: Arc<Database>,
}

impl ApiCredentialManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(
        &self,
        provider_id: &str,
        credential_type: &str,
        credential_key: &str,
        credential_value: &str,
        label: Option<&str>,
    ) -> Result<ApiCredential> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // Simple obfuscation (not encryption, but better than plaintext in UI)
        let obfuscated = obfuscate_value(credential_value);

        self.db.execute(
            "INSERT INTO api_credentials (id, provider_id, credential_type, credential_key, credential_value_encrypted, label) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            &[
                &id as &dyn rusqlite::types::ToSql,
                &provider_id,
                &credential_type,
                &credential_key,
                &obfuscated,
                &label,
            ],
        )?;

        Ok(ApiCredential {
            id,
            provider_id: provider_id.to_string(),
            credential_type: credential_type.to_string(),
            credential_key: credential_key.to_string(),
            credential_value_encrypted: obfuscated,
            label: label.map(|s| s.to_string()),
            is_active: true,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_all(&self) -> Result<Vec<ApiCredential>> {
        self.db.query_map(
            "SELECT id, provider_id, credential_type, credential_key, credential_value_encrypted, label, is_active, created_at, updated_at FROM api_credentials ORDER BY provider_id, created_at DESC",
            &[],
            |row| {
                Ok(ApiCredential {
                    id: row.get(0)?,
                    provider_id: row.get(1)?,
                    credential_type: row.get(2)?,
                    credential_key: row.get(3)?,
                    credential_value_encrypted: row.get(4)?,
                    label: row.get(5)?,
                    is_active: row.get::<_, i64>(6)? != 0,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
    }

    pub fn get_for_provider(&self, provider_id: &str) -> Result<Vec<ApiCredential>> {
        self.db.query_map(
            "SELECT id, provider_id, credential_type, credential_key, credential_value_encrypted, label, is_active, created_at, updated_at FROM api_credentials WHERE provider_id = ?1 AND is_active = 1 ORDER BY created_at DESC",
            &[&provider_id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(ApiCredential {
                    id: row.get(0)?,
                    provider_id: row.get(1)?,
                    credential_type: row.get(2)?,
                    credential_key: row.get(3)?,
                    credential_value_encrypted: row.get(4)?,
                    label: row.get(5)?,
                    is_active: row.get::<_, i64>(6)? != 0,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
    }

    pub fn get_active_for_provider(&self, provider_id: &str) -> Result<Option<ApiCredential>> {
        match self.db.query_row(
            "SELECT id, provider_id, credential_type, credential_key, credential_value_encrypted, label, is_active, created_at, updated_at FROM api_credentials WHERE provider_id = ?1 AND is_active = 1 LIMIT 1",
            &[&provider_id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(ApiCredential {
                    id: row.get(0)?,
                    provider_id: row.get(1)?,
                    credential_type: row.get(2)?,
                    credential_key: row.get(3)?,
                    credential_value_encrypted: row.get(4)?,
                    label: row.get(5)?,
                    is_active: row.get::<_, i64>(6)? != 0,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        ) {
            Ok(c) => Ok(Some(c)),
            Err(e) => {
                if e.to_string().contains("QueryReturnedNoRows") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn get_decrypted_value(&self, id: &str) -> Result<String> {
        let obfuscated: String = self.db.query_row(
            "SELECT credential_value_encrypted FROM api_credentials WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            |row| row.get(0),
        )?;
        Ok(deobfuscate_value(&obfuscated))
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.execute("DELETE FROM api_credentials WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])?;
        Ok(())
    }

    pub fn set_active(&self, id: &str) -> Result<()> {
        let cred = self.get_by_id(id)?;
        self.db.execute(
            "UPDATE api_credentials SET is_active = 0 WHERE provider_id = ?1",
            &[&cred.provider_id as &dyn rusqlite::types::ToSql],
        )?;
        self.db.execute(
            "UPDATE api_credentials SET is_active = 1, updated_at = datetime('now') WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
        )?;
        Ok(())
    }

    pub fn update(&self, id: &str, label: Option<&str>, credential_value: Option<&str>) -> Result<()> {
        if let Some(l) = label {
            self.db.execute(
                "UPDATE api_credentials SET label = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&l as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        if let Some(v) = credential_value {
            let obfuscated = obfuscate_value(v);
            self.db.execute(
                "UPDATE api_credentials SET credential_value_encrypted = ?1, updated_at = datetime('now') WHERE id = ?2",
                &[&obfuscated as &dyn rusqlite::types::ToSql, &id],
            )?;
        }
        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<ApiCredential> {
        self.db.query_row(
            "SELECT id, provider_id, credential_type, credential_key, credential_value_encrypted, label, is_active, created_at, updated_at FROM api_credentials WHERE id = ?1",
            &[&id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(ApiCredential {
                    id: row.get(0)?,
                    provider_id: row.get(1)?,
                    credential_type: row.get(2)?,
                    credential_key: row.get(3)?,
                    credential_value_encrypted: row.get(4)?,
                    label: row.get(5)?,
                    is_active: row.get::<_, i64>(6)? != 0,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )
    }
}

/// Simple obfuscation for credential storage (XOR with rotating key)
/// Note: This is NOT encryption. For production, use OS keychain via the keyring crate.
fn obfuscate_value(value: &str) -> String {
    let key = b"luna-obfuscation-key";
    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    for (i, b) in bytes.iter().enumerate() {
        result.push(b ^ key[i % key.len()]);
    }
    base64_encode(&result)
}

fn deobfuscate_value(encoded: &str) -> String {
    let key = b"luna-obfuscation-key";
    let bytes = base64_decode(encoded);
    let mut result = Vec::with_capacity(bytes.len());
    for (i, b) in bytes.iter().enumerate() {
        result.push(b ^ key[i % key.len()]);
    }
    String::from_utf8_lossy(&result).to_string()
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len() * 4 / 3 + 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn base64_decode(encoded: &str) -> Vec<u8> {
    let lookup = |c: u8| -> u8 {
        match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    };

    let filtered: Vec<u8> = encoded.bytes().filter(|b| *b != b'=' && *b != b'\n').collect();
    let mut result = Vec::with_capacity(filtered.len() * 3 / 4);

    for chunk in filtered.chunks(4) {
        let b0 = lookup(chunk[0]) as u32;
        let b1 = if chunk.len() > 1 { lookup(chunk[1]) as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { lookup(chunk[2]) as u32 } else { 0 };
        let b3 = if chunk.len() > 3 { lookup(chunk[3]) as u32 } else { 0 };

        let triple = (b0 << 18) | (b1 << 12) | (b2 << 6) | b3;
        result.push(((triple >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            result.push((triple & 0xFF) as u8);
        }
    }

    result
}

/// Generic API Connector - makes HTTP requests to API endpoints
pub struct ApiConnector {
    client: Client,
}

impl ApiConnector {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn execute_request(
        &self,
        method: &str,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: Option<&str>,
    ) -> Result<ApiConnectorResponse> {
        let start = std::time::Instant::now();

        let mut req = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
        };

        if let Some(h) = headers {
            for (key, value) in h {
                req = req.header(*key, *value);
            }
        }

        if let Some(b) = body {
            req = req.header("Content-Type", "application/json");
            req = req.body(b.to_string());
        }

        let response = req.send().await?;
        let elapsed = start.elapsed().as_millis() as i64;
        let status = response.status().as_u16();
        let text = response.text().await.unwrap_or_default();

        Ok(ApiConnectorResponse {
            status,
            body: text,
            response_time_ms: elapsed,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConnectorResponse {
    pub status: u16,
    pub body: String,
    pub response_time_ms: i64,
}
