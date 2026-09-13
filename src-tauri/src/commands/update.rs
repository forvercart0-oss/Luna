use serde::{Deserialize, Serialize};

const MANIFEST_URL: &str =
    "https://github.com/forvercart0-oss/Luna/releases/latest/download/latest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub platforms: Option<std::collections::HashMap<String, PlatformInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub url: String,
    pub sha256: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub status: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub download_url: Option<String>,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateCheckResult, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();

    let client = match reqwest::Client::builder()
        .user_agent(format!("LUNA-Desktop/{}", current_version))
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return Ok(UpdateCheckResult {
                status: "error".into(),
                current_version,
                latest_version: None,
                download_url: None,
                notes: Some(format!("Failed to create HTTP client: {}", e)),
            });
        }
    };

    let resp = match client.get(MANIFEST_URL).send().await {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            let status = if msg.contains("connection") || msg.contains("resolve") || msg.contains("timed out") {
                "offline"
            } else {
                "error"
            };
            return Ok(UpdateCheckResult {
                status: status.into(),
                current_version,
                latest_version: None,
                download_url: None,
                notes: Some(msg),
            });
        }
    };

    let status_code = resp.status().as_u16();
    if status_code == 404 {
        return Ok(UpdateCheckResult {
            status: "up_to_date".into(),
            current_version,
            latest_version: None,
            download_url: None,
            notes: Some("No published release available".into()),
        });
    }

    if !resp.status().is_success() {
        return Ok(UpdateCheckResult {
            status: "error".into(),
            current_version,
            latest_version: None,
            download_url: None,
            notes: Some(format!("HTTP {}", status_code)),
        });
    }

    let manifest: UpdateManifest = match resp.json().await {
        Ok(m) => m,
        Err(e) => {
            return Ok(UpdateCheckResult {
                status: "error".into(),
                current_version,
                latest_version: None,
                download_url: None,
                notes: Some(format!("Invalid manifest: {}", e)),
            });
        }
    };

    let latest_version = manifest.version.clone();

    let (os, arch) = detect_platform();
    let platform_key = format!("{}-{}", os, arch);

    let download_url = manifest
        .platforms
        .as_ref()
        .and_then(|p| p.get(&platform_key))
        .map(|p| p.url.clone());

    let has_update = version_cmp(&current_version, &latest_version) == std::cmp::Ordering::Less;

    if has_update {
        Ok(UpdateCheckResult {
            status: "update_available".into(),
            current_version,
            latest_version: Some(latest_version),
            download_url,
            notes: manifest.notes,
        })
    } else {
        Ok(UpdateCheckResult {
            status: "up_to_date".into(),
            current_version,
            latest_version: Some(latest_version),
            download_url: None,
            notes: None,
        })
    }
}

fn detect_platform() -> (&'static str, &'static str) {
    let os = match std::env::consts::OS {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "darwin",
        _ => "unknown",
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => other,
    };
    (os, arch)
}

fn version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let pb: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    pa.cmp(&pb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_cmp_equal() {
        assert_eq!(version_cmp("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
    }

    #[test]
    fn version_cmp_less() {
        assert_eq!(version_cmp("0.1.0", "0.2.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.0.0", "2.0.0"), std::cmp::Ordering::Less);
    }

    #[test]
    fn version_cmp_greater() {
        assert_eq!(version_cmp("0.2.0", "0.1.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("2.0.0", "1.0.0"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn detect_platform_returns_valid_os() {
        let (os, _) = detect_platform();
        assert!(
            ["windows", "linux", "darwin", "unknown"].contains(&os),
            "unexpected OS: {}",
            os
        );
    }

    #[test]
    fn detect_platform_returns_valid_arch() {
        let (_, arch) = detect_platform();
        assert!(
            ["x86_64", "aarch64", "arm", "wasm32"].contains(&arch)
                || arch.starts_with("x86")
                || arch.starts_with("arm"),
            "unexpected arch: {}",
            arch
        );
    }
}
