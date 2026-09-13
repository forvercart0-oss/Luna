use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::process::Command;

const REPO: &str = "forvercart0-oss/Luna";
const MANIFEST_URL: &str =
    "https://github.com/forvercart0-oss/Luna/releases/latest/download/latest.json";

#[derive(Parser)]
#[command(name = "luna", version, about = "LUNA Desktop AI Assistant CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check for and install updates
    Update,
    /// Show LUNA version information
    Version,
    /// Run diagnostic checks on LUNA installation
    Doctor,
}

// ── Helpers ──

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_exe_path() -> Option<std::path::PathBuf> {
    std::env::current_exe().ok()
}

fn get_install_dir() -> Option<String> {
    if let Some(exe) = get_exe_path() {
        if let Some(parent) = exe.parent() {
            return Some(parent.to_string_lossy().to_string());
        }
    }
    let paths = ["/opt/LUNA", "/usr/local/bin", "/usr/bin"];
    for path in &paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

fn get_data_dir() -> Option<std::path::PathBuf> {
    if let Some(data_dir) = dirs_next::data_dir() {
        let p = data_dir.join("luna");
        if p.exists() {
            return Some(p);
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let p = std::path::PathBuf::from(home).join(".local/share/luna");
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn detect_platform() -> (&'static str, &'static str) {
    let os = match std::env::consts::OS {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "macos",
        _ => "unknown",
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "x86" => "i686",
        other => other,
    };
    (os, arch)
}

fn version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let pb: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    pa.cmp(&pb)
}

fn http_get(url: &str) -> Result<reqwest::blocking::Response, String> {
    reqwest::blocking::Client::builder()
        .user_agent(format!("LUNA-CLI/{}", get_version()))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?
        .get(url)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))
}

fn sha256_file(path: &std::path::Path) -> Result<String, String> {
    let data = std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

// ── luna version ──

fn cmd_version() {
    println!("LUNA Desktop AI Assistant");
    println!("Version:    {}", get_version());
    println!("Identifier: com.luna.desktop");

    if let Some(dir) = get_install_dir() {
        println!("Install:    {}", dir);
    }
    if let Some(dir) = get_data_dir() {
        println!("Data:       {}", dir.display());
    }

    let (os, arch) = detect_platform();
    println!("Platform:   {} ({})", os, arch);
}

// ── luna doctor ──

fn cmd_doctor() {
    println!("LUNA Doctor");
    println!("===========");
    println!();

    let mut pass = 0u32;
    let mut fail = 0u32;
    let mut skip = 0u32;

    // 1. CLI binary
    print!("[1/8] CLI binary .................. ");
    if let Some(exe) = get_exe_path() {
        if exe.exists() {
            println!("OK ({})", exe.display());
            pass += 1;
        } else {
            println!("FAIL (path does not exist)");
            fail += 1;
        }
    } else {
        println!("FAIL (cannot determine path)");
        fail += 1;
    }

    // 2. Data directory
    print!("[2/8] Data directory ............. ");
    match get_data_dir() {
        Some(dir) => {
            println!("OK ({})", dir.display());
            pass += 1;

            // 3. Database
            print!("[3/8] SQLite database ............ ");
            let db = dir.join("luna.db");
            if db.exists() {
                let size = std::fs::metadata(&db).map(|m| m.len()).unwrap_or(0);
                println!("OK ({} bytes)", size);
                pass += 1;
            } else {
                println!("NOT FOUND (will be created on first launch)");
                skip += 1;
            }
        }
        None => {
            println!("NOT FOUND (will be created on first launch)");
            skip += 1;
            println!("[3/8] SQLite database ............ SKIPPED (no data dir)");
            skip += 1;
        }
    }

    // 4. OpenRouter configuration
    print!("[4/8] OpenRouter credentials ..... ");
    if let Some(dir) = get_data_dir() {
        let db_path = dir.join("luna.db");
        if db_path.exists() {
            match rusqlite::Connection::open(&db_path) {
                Ok(conn) => {
                    let count: Result<i64, _> = conn.query_row(
                        "SELECT COUNT(*) FROM provider_accounts WHERE api_key_encrypted IS NOT NULL AND api_key_encrypted != ''",
                        [],
                        |row| row.get(0),
                    );
                    match count {
                        Ok(n) if n > 0 => {
                            println!("OK ({} account(s) configured)", n);
                            pass += 1;
                        }
                        Ok(_) => {
                            println!("NOT CONFIGURED (add API key in Settings > AI Providers)");
                            fail += 1;
                        }
                        Err(_) => {
                            println!("TABLE NOT FOUND (run LUNA once to initialize)");
                            skip += 1;
                        }
                    }
                }
                Err(_) => {
                    println!("CANNOT OPEN DB");
                    skip += 1;
                }
            }
        } else {
            println!("SKIPPED (no database)");
            skip += 1;
        }
    } else {
        println!("SKIPPED (no data directory)");
        skip += 1;
    }

    // 5. Network: GitHub
    print!("[5/8] GitHub connectivity ......... ");
    match http_get("https://api.github.com/") {
        Ok(resp) => {
            if resp.status().is_success() || resp.status().as_u16() == 403 {
                println!("OK (HTTP {})", resp.status().as_u16());
                pass += 1;
            } else {
                println!("WARN (HTTP {})", resp.status().as_u16());
                pass += 1;
            }
        }
        Err(e) => {
            println!("FAIL ({})", e);
            fail += 1;
        }
    }

    // 6. Network: OpenRouter
    print!("[6/8] OpenRouter API .............. ");
    match http_get("https://openrouter.ai/api/v1/models") {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("OK");
                pass += 1;
            } else {
                println!("WARN (HTTP {})", resp.status().as_u16());
                pass += 1;
            }
        }
        Err(e) => {
            println!("FAIL ({})", e);
            fail += 1;
        }
    }

    // 7. Update manifest
    print!("[7/8] Update manifest ............. ");
    match http_get(MANIFEST_URL) {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>() {
                    Ok(v) => {
                        let ver = v["version"].as_str().unwrap_or("?");
                        println!("OK (latest: v{})", ver);
                        pass += 1;
                    }
                    Err(_) => {
                        println!("INVALID (not valid JSON)");
                        fail += 1;
                    }
                }
            } else {
                println!("NOT AVAILABLE (HTTP {})", resp.status().as_u16());
                skip += 1;
            }
        }
        Err(e) => {
            println!("FAIL ({})", e);
            fail += 1;
        }
    }

    // 8. Voice services (optional)
    print!("[8/8] Voice services (optional) .. ");
    let endpoint = std::env::var("KOKORO_TTS_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8880".to_string());
    match http_get(&format!("{}/v1/models", endpoint)) {
        Ok(resp) if resp.status().is_success() => {
            println!("OK ({})", endpoint);
            pass += 1;
        }
        _ => {
            println!("NOT RUNNING (optional)");
            skip += 1;
        }
    }

    println!();
    println!(
        "Result: {} passed, {} failed, {} skipped",
        pass, fail, skip
    );
    if fail == 0 {
        println!("LUNA is healthy.");
    } else {
        println!("{} issue(s) require attention.", fail);
    }
}

// ── luna update ──

fn cmd_update() {
    let current = get_version();
    let (os, arch) = detect_platform();

    println!("LUNA Update");
    println!("===========");
    println!();
    println!("Current:  v{}", current);
    println!("Platform: {} {}", os, arch);
    println!();
    println!("Fetching update manifest...");

    // Fetch manifest
    let manifest: serde_json::Value = match http_get(MANIFEST_URL) {
        Ok(resp) => {
            let status = resp.status();
            if status == reqwest::StatusCode::NOT_FOUND {
                println!("No published release is currently available.");
                println!("Check back later or download manually:");
                println!("  https://github.com/{}/releases", REPO);
                return;
            }
            if !status.is_success() {
                eprintln!(
                    "ERROR: Could not fetch update manifest (HTTP {})",
                    status
                );
                eprintln!("Download manually: https://github.com/{}/releases", REPO);
                std::process::exit(1);
            }
            match resp.json() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("ERROR: Invalid manifest: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            eprintln!("Check your network connection.");
            eprintln!("Download manually: https://github.com/{}/releases", REPO);
            std::process::exit(1);
        }
    };

    let latest = match manifest["version"].as_str() {
        Some(v) => v,
        None => {
            eprintln!("ERROR: Manifest missing version field");
            std::process::exit(1);
        }
    };

    println!("Latest:   v{}", latest);
    println!();

    if version_cmp(latest, &current) != std::cmp::Ordering::Greater {
        println!("You are already up to date.");
        return;
    }

    println!("Update available: v{} -> v{}", current, latest);
    println!();

    // Find artifact for this platform
    let platform_key = format!("{}-{}", os, arch);
    let platform_info = match manifest["platforms"][&platform_key].as_object() {
        Some(p) => p,
        None => {
            eprintln!(
                "ERROR: No build available for {} ({})",
                os, arch
            );
            eprintln!("Download manually: https://github/{}/releases", REPO);
            std::process::exit(1);
        }
    };

    let url = match platform_info.get("url").and_then(|v| v.as_str()) {
        Some(u) => u,
        None => {
            eprintln!("ERROR: Artifact URL missing from manifest");
            std::process::exit(1);
        }
    };

    let expected_sha = platform_info
        .get("sha256")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let filename = url.rsplit('/').next().unwrap_or("update");

    println!("Artifact: {}", filename);
    println!("URL:      {}", url);
    if !expected_sha.is_empty() {
        println!("SHA-256:  {}...", &expected_sha[..16.min(expected_sha.len())]);
    }
    println!();

    // Download
    let tmp_dir = std::env::temp_dir().join("luna-update");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let tmp_path = tmp_dir.join(filename);

    println!("Downloading...");
    let client = match reqwest::blocking::Client::builder()
        .user_agent(format!("LUNA-CLI/{}", get_version()))
        .timeout(std::time::Duration::from_secs(600))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: Failed to create HTTP client: {}", e);
            std::process::exit(1);
        }
    };

    let resp = match client.get(url).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: Download failed: {}", e);
            std::process::exit(1);
        }
    };

    if !resp.status().is_success() {
        eprintln!("ERROR: Download returned HTTP {}", resp.status());
        std::process::exit(1);
    }

    let total_size = resp.content_length().unwrap_or(0);
    let mut file = match std::fs::File::create(&tmp_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: Cannot create temp file: {}", e);
            std::process::exit(1);
        }
    };

    let mut downloaded: u64 = 0;
    let mut reader = resp;
    let chunk_size = 1024 * 256;
    loop {
        let mut buf = vec![0u8; chunk_size];
        use std::io::Read;
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                buf.truncate(n);
                if file.write_all(&buf).is_err() {
                    eprintln!("ERROR: Write failed");
                    let _ = std::fs::remove_file(&tmp_path);
                    std::process::exit(1);
                }
                downloaded += n as u64;
                if total_size > 0 {
                    let pct = downloaded * 100 / total_size;
                    print!("\r  {} / {} MB ({}%)   ",
                        downloaded / 1024 / 1024,
                        total_size / 1024 / 1024,
                        pct);
                } else {
                    print!("\r  {} MB downloaded...   ", downloaded / 1024 / 1024);
                }
                use std::io::stdout;
                let _ = stdout().flush();
            }
            Err(e) => {
                eprintln!("\nERROR: Download interrupted: {}", e);
                let _ = std::fs::remove_file(&tmp_path);
                std::process::exit(1);
            }
        }
    }
    println!();
    println!("Download complete ({} bytes).", downloaded);

    // Verify checksum
    if !expected_sha.is_empty() {
        println!("Verifying checksum...");
        let actual = match sha256_file(&tmp_path) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                let _ = std::fs::remove_file(&tmp_path);
                std::process::exit(1);
            }
        };
        if actual != expected_sha {
            eprintln!("ERROR: Checksum mismatch!");
            eprintln!("  Expected: {}", expected_sha);
            eprintln!("  Got:      {}", actual);
            eprintln!("The downloaded file may be corrupted.");
            let _ = std::fs::remove_file(&tmp_path);
            std::process::exit(1);
        }
        println!("Checksum verified.");
    }

    // Install
    println!();
    println!("Installing...");

    let success = match (os, filename) {
        ("linux", name) if name.ends_with(".AppImage") => {
            install_appimage(&tmp_path, name)
        }
        ("linux", name) if name.ends_with(".deb") => {
            install_deb(&tmp_path)
        }
        ("macos", name) if name.ends_with(".app.tar.gz") => {
            install_macos_app(&tmp_path)
        }
        ("windows", name) if name.ends_with(".msi") => {
            install_msi(&tmp_path)
        }
        _ => {
            // CLI binary or unknown: try to replace self
            install_binary(&tmp_path, filename)
        }
    };

    // Cleanup
    let _ = std::fs::remove_file(&tmp_path);
    let _ = std::fs::remove_dir_all(&tmp_dir);

    println!();
    if success {
        println!("Update installed successfully (v{} -> v{}).", current, latest);
        println!("Restart LUNA to use the new version.");
    } else {
        eprintln!("Update installation failed.");
        eprintln!("Download the artifact manually from:");
        eprintln!("  https://github/{}/releases", REPO);
    }
}

fn install_appimage(tmp: &std::path::Path, filename: &str) -> bool {
    // Find current AppImage location
    let _current_exe = match get_exe_path() {
        Some(p) => p,
        None => {
            eprintln!("Cannot determine current executable path.");
            return false;
        }
    };

    // If we're inside an AppImage, the parent is the mount point
    // The actual AppImage is at APPIMAGE env var or we look in common places
    let appimage_path = if let Ok(appimage) = std::env::var("APPIMAGE") {
        std::path::PathBuf::from(appimage)
    } else {
        // Look in ~/Applications, /opt, /usr/local/bin
        let candidates = [
            dirs_next::home_dir()
                .map(|h| h.join("Applications").join(filename)),
            Some(std::path::PathBuf::from(format!("/opt/{}", filename))),
            Some(std::path::PathBuf::from(format!("/usr/local/bin/{}", filename))),
        ];
        candidates
            .into_iter()
            .flatten()
            .find(|p| p.exists())
            .unwrap_or_else(|| {
                let dest = dirs_next::home_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join("Applications")
                    .join(filename);
                dest
            })
    };

    // Backup old
    if appimage_path.exists() {
        let backup = appimage_path.with_extension("AppImage.old");
        if let Err(e) = std::fs::rename(&appimage_path, &backup) {
            eprintln!("Warning: Could not backup old file: {}", e);
        } else {
            let _ = std::fs::remove_file(&backup);
        }
    }

    // Ensure parent exists
    if let Some(parent) = appimage_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::copy(tmp, &appimage_path) {
        Ok(_) => {
            // Make executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(
                    &appimage_path,
                    std::fs::Permissions::from_mode(0o755),
                );
            }
            println!("Installed to: {}", appimage_path.display());
            true
        }
        Err(e) => {
            eprintln!("Failed to install: {}", e);
            false
        }
    }
}

fn install_deb(tmp: &std::path::Path) -> bool {
    println!("Installing .deb package (requires sudo)...");
    match Command::new("sudo")
        .args(["dpkg", "-i", &tmp.to_string_lossy()])
        .status()
    {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("Failed to run dpkg: {}", e);
            false
        }
    }
}

fn install_macos_app(tmp: &std::path::Path) -> bool {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let extract_dir = std::env::temp_dir().join("luna-macos-extract");
    let _ = std::fs::create_dir_all(&extract_dir);

    // Extract tar.gz
    let file = match std::fs::File::open(tmp) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot open archive: {}", e);
            return false;
        }
    };

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    if let Err(e) = archive.unpack(&extract_dir) {
        eprintln!("Extraction failed: {}", e);
        let _ = std::fs::remove_dir_all(&extract_dir);
        return false;
    }

    // Find .app directory
    let app_dir = find_app_bundle(&extract_dir);
    let app_dir = match app_dir {
        Some(d) => d,
        None => {
            eprintln!("No .app bundle found in archive");
            let _ = std::fs::remove_dir_all(&extract_dir);
            return false;
        }
    };

    let app_name = app_dir.file_name().unwrap().to_string_lossy().to_string();
    let dest = std::path::PathBuf::from("/Applications").join(&app_name);

    if dest.exists() {
        let _ = std::fs::remove_dir_all(&dest);
    }

    match std::fs::rename(&app_dir, &dest) {
        Ok(_) => {
            println!("Installed to: {}", dest.display());
            let _ = std::fs::remove_dir_all(&extract_dir);
            true
        }
        Err(e) => {
            eprintln!("Failed to move app: {}", e);
            println!("Try manually: sudo mv {} /Applications/", app_dir.display());
            false
        }
    }
}

fn find_app_bundle(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() && path.to_string_lossy().ends_with(".app") {
                return Some(path);
            }
        }
        // Recurse one level
        for entry in std::fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_app_bundle(&path) {
                    return Some(found);
                }
            }
        }
    }
    None
}

fn install_msi(tmp: &std::path::Path) -> bool {
    println!("Running MSI installer...");
    match Command::new("msiexec")
        .args(["/i", &tmp.to_string_lossy(), "/quiet", "/norestart"])
        .status()
    {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("Failed to run msiexec: {}", e);
            println!("Run manually: msiexec /i \"{}\"", tmp.display());
            false
        }
    }
}

fn install_binary(tmp: &std::path::Path, _filename: &str) -> bool {
    // Self-update: replace the current CLI binary
    let current = match get_exe_path() {
        Some(p) => p,
        None => {
            eprintln!("Cannot determine current executable path.");
            return false;
        }
    };

    let old = current.with_extension("old");

    // Rename current to .old
    if let Err(e) = std::fs::rename(&current, &old) {
        eprintln!("Cannot replace current binary: {}", e);
        println!("Manual update:");
        println!("  1. Close all LUNA processes");
        println!("  2. cp \"{}\" \"{}\"", tmp.display(), current.display());
        return false;
    }

    // Move new into place
    if let Err(e) = std::fs::rename(tmp, &current) {
        eprintln!("Failed to install new binary: {}", e);
        // Try to restore old
        let _ = std::fs::rename(&old, &current);
        return false;
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&current, std::fs::Permissions::from_mode(0o755));
    }

    // Cleanup .old
    let _ = std::fs::remove_file(&old);

    println!("CLI updated: {}", current.display());
    true
}

// ── main ──

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => cmd_update(),
        Commands::Version => cmd_version(),
        Commands::Doctor => cmd_doctor(),
    }
}

// ── tests ──

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn detect_platform_returns_valid_os() {
        let (os, _arch) = detect_platform();
        assert!(
            ["windows", "linux", "macos", "unknown"].contains(&os),
            "unexpected OS: {}",
            os
        );
    }

    #[test]
    fn detect_platform_returns_valid_arch() {
        let (_os, arch) = detect_platform();
        assert!(
            ["x86_64", "aarch64", "i686", "arm", "wasm32"].contains(&arch)
                || arch.starts_with("x86")
                || arch.starts_with("arm"),
            "unexpected arch: {}",
            arch
        );
    }

    #[test]
    fn version_cmp_equal() {
        assert_eq!(version_cmp("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
    }

    #[test]
    fn version_cmp_less() {
        assert_eq!(version_cmp("0.1.0", "0.2.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.0.0", "2.0.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.0.0", "1.1.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
    }

    #[test]
    fn version_cmp_greater() {
        assert_eq!(version_cmp("0.2.0", "0.1.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("2.0.0", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("1.1.0", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("1.0.1", "1.0.0"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn version_cmp_short_versions() {
        // Different-length vectors: shorter is less (Vec::cmp is lexicographic)
        assert_eq!(version_cmp("1.0", "1.0.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1", "1.0.0"), std::cmp::Ordering::Less);
        // Same-length short versions still work
        assert_eq!(version_cmp("1.0", "1.0"), std::cmp::Ordering::Equal);
    }

    #[test]
    fn version_cmp_non_numeric_segments() {
        // Non-numeric segments are filtered out by parse().ok()
        // "1.0.0-beta" → [1, 0] (0-beta fails parse), "1.0.0" → [1, 0, 0]
        assert_eq!(
            version_cmp("1.0.0-beta", "1.0.0"),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn sha256_file_computes_hash() {
        let dir = std::env::temp_dir().join("luna-test-sha256");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.bin");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"hello world").unwrap();
        drop(f);

        let hash = sha256_file(&path).unwrap();
        // SHA-256 of "hello world" is well-known
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sha256_file_empty_file() {
        let dir = std::env::temp_dir().join("luna-test-sha256-empty");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("empty.bin");
        std::fs::File::create(&path).unwrap();

        let hash = sha256_file(&path).unwrap();
        // SHA-256 of empty content
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sha256_file_nonexistent_returns_error() {
        let result = sha256_file(std::path::Path::new("/nonexistent/path/file.bin"));
        assert!(result.is_err());
    }

    #[test]
    fn get_version_returns_nonempty() {
        let v = get_version();
        assert!(!v.is_empty());
        // Version should contain at least one dot
        assert!(v.contains('.'), "version should contain dots: {}", v);
    }

    #[test]
    fn manifest_url_is_https() {
        assert!(
            MANIFEST_URL.starts_with("https://"),
            "manifest URL must use HTTPS"
        );
    }

    #[test]
    fn manifest_url_contains_releases() {
        assert!(
            MANIFEST_URL.contains("/releases/"),
            "manifest URL must point to releases"
        );
    }

    #[test]
    fn manifest_url_ends_with_json() {
        assert!(
            MANIFEST_URL.ends_with(".json"),
            "manifest URL must end with .json"
        );
    }
}
