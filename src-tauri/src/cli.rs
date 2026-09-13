use clap::{Parser, Subcommand};
use std::process::Command;

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

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_install_dir() -> Option<String> {
    // Check common installation paths
    let paths = [
        "/opt/LUNA",
        "/usr/local/bin",
        "/usr/bin",
    ];
    
    // On Linux, check AppImage or package install locations
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return Some(parent.to_string_lossy().to_string());
        }
    }
    
    for path in &paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

fn get_data_dir() -> Option<String> {
    if let Some(home) = std::env::var_os("HOME") {
        let data_dir = std::path::PathBuf::from(home).join(".local/share/luna");
        if data_dir.exists() {
            return Some(data_dir.to_string_lossy().to_string());
        }
    }
    if let Some(data_dir) = dirs_next::data_dir() {
        let data_dir = data_dir.join("luna");
        if data_dir.exists() {
            return Some(data_dir.to_string_lossy().to_string());
        }
    }
    None
}

fn cmd_version() {
    println!("LUNA Desktop AI Assistant");
    println!("Version: {}", get_version());
    println!("Identifier: com.luna.desktop");
    
    if let Some(dir) = get_install_dir() {
        println!("Install directory: {}", dir);
    }
    if let Some(dir) = get_data_dir() {
        println!("Data directory: {}", dir);
    }
    
    // Check Rust version
    if let Ok(output) = Command::new("rustc").arg("--version").output() {
        let rust_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("Rust: {}", rust_version);
    }
    
    // Check Node version
    if let Ok(output) = Command::new("node").arg("--version").output() {
        let node_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("Node: {}", node_version);
    }
}

fn cmd_doctor() {
    println!("LUNA Diagnostic Report");
    println!("======================");
    println!();
    
    let mut issues = 0;
    
    // Check installation
    println!("[1/7] Installation");
    if let Some(dir) = get_install_dir() {
        println!("  ✓ Install directory: {}", dir);
    } else {
        println!("  ✗ Install directory not found");
        issues += 1;
    }
    
    // Check data directory
    println!("[2/7] Data Directory");
    if let Some(dir) = get_data_dir() {
        println!("  ✓ Data directory: {}", dir);
        
        // Check database
        let db_path = std::path::PathBuf::from(&dir).join("luna.db");
        if db_path.exists() {
            let size = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
            println!("  ✓ Database exists ({} bytes)", size);
        } else {
            println!("  ✗ Database not found at {}", db_path.display());
            issues += 1;
        }
    } else {
        println!("  ✗ Data directory not found");
        issues += 1;
    }
    
    // Check Rust
    println!("[3/7] Rust Toolchain");
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ {}", version);
        }
        Err(_) => {
            println!("  ✗ rustc not found");
            issues += 1;
        }
    }
    
    match Command::new("cargo").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ {}", version);
        }
        Err(_) => {
            println!("  ✗ cargo not found");
            issues += 1;
        }
    }
    
    // Check Node/pnpm
    println!("[4/7] Node.js Toolchain");
    match Command::new("node").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ {}", version);
        }
        Err(_) => {
            println!("  ✗ node not found");
            issues += 1;
        }
    }
    
    match Command::new("pnpm").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ pnpm {}", version);
        }
        Err(_) => {
            println!("  ✗ pnpm not found");
            issues += 1;
        }
    }
    
    // Check Tauri
    println!("[5/7] Tauri CLI");
    match Command::new("pnpm").args(["tauri", "--version"]).output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ {}", version);
        }
        Err(_) => {
            println!("  ✗ tauri CLI not found");
            issues += 1;
        }
    }
    
    // Check network
    println!("[6/7] Network Connectivity");
    match reqwest::blocking::get("https://openrouter.ai/api/v1/models") {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("  ✓ OpenRouter API reachable");
            } else {
                println!("  ✗ OpenRouter API returned {}", resp.status());
                issues += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Cannot reach OpenRouter API: {}", e);
            issues += 1;
        }
    }
    
    // Check voice services
    println!("[7/7] Voice Services");
    let kokoro_endpoint = std::env::var("KOKORO_TTS_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8880".to_string());
    match reqwest::blocking::get(format!("{}/v1/models", kokoro_endpoint)) {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("  ✓ Kokoro TTS available at {}", kokoro_endpoint);
            } else {
                println!("  - Kokoro TTS not available at {} (optional)", kokoro_endpoint);
            }
        }
        Err(_) => {
            println!("  - Kokoro TTS not available (optional)");
        }
    }
    
    println!();
    println!("======================");
    if issues == 0 {
        println!("All checks passed. LUNA is ready.");
    } else {
        println!("{} issue(s) found.", issues);
    }
}

fn cmd_update() {
    println!("LUNA Update Check");
    println!("=================");
    println!();
    
    let current_version = get_version();
    println!("Current version: {}", current_version);
    println!("Checking for updates...");
    println!();
    
    // Fetch latest release info from GitHub API
    let api_url = "https://api.github.com/repos/forvercart0-oss/Luna/releases/latest";
    match reqwest::blocking::Client::builder()
        .user_agent("LUNA-Updater/0.1.0")
        .build()
        .and_then(|c| c.get(api_url).send())
    {
        Ok(resp) => {
            if !resp.status().is_success() {
                println!("Could not check for updates (HTTP {})", resp.status());
                println!("You can download manually from:");
                println!("  https://github.com/forvercart0-oss/Luna/releases");
                return;
            }
            
            match resp.json::<serde_json::Value>() {
                Ok(release) => {
                    let tag = release["tag_name"].as_str().unwrap_or("unknown");
                    let latest_version = tag.trim_start_matches('v');
                    
                    println!("Latest version: {}", latest_version);
                    
                    if latest_version == current_version {
                        println!();
                        println!("You are running the latest version.");
                        return;
                    }
                    
                    println!();
                    println!("A new version is available: {} → {}", current_version, latest_version);
                    println!();
                    
                    // Detect platform
                    let (platform, arch) = detect_platform();
                    println!("Platform: {} ({})", platform, arch);
                    
                    // Find matching asset
                    let empty_assets = vec![];
                    let assets = release["assets"].as_array().unwrap_or(&empty_assets);
                    let expected_suffix = match (platform.as_str(), arch.as_str()) {
                        ("windows", "x86_64") => ".msi",
                        ("linux", "x86_64") => ".AppImage",
                        ("macos", "aarch64") => ".app.tar.gz",
                        ("macos", "x86_64") => ".app.tar.gz",
                        _ => "",
                    };
                    
                    let matching_asset = assets.iter().find(|a| {
                        let name = a["name"].as_str().unwrap_or("");
                        name.contains(expected_suffix)
                    });
                    
                    if let Some(asset) = matching_asset {
                        let name = asset["name"].as_str().unwrap_or("unknown");
                        let size = asset["size"].as_u64().unwrap_or(0);
                        let url = asset["browser_download_url"].as_str().unwrap_or("");
                        
                        println!();
                        println!("Asset: {} ({} MB)", name, size / 1024 / 1024);
                        println!("URL: {}", url);
                        println!();
                        println!("To install, download the artifact and run:");
                        println!("  - Windows: Run the .msi installer");
                        println!("  - Linux: chmod +x the .AppImage && run it");
                        println!("  - macOS: Drag the .app to Applications");
                    } else {
                        println!();
                        println!("No matching build found for {} ({}).", platform, arch);
                        println!("Download manually from: https://github.com/forvercart0-oss/Luna/releases");
                    }
                }
                Err(e) => {
                    println!("Failed to parse release info: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to check for updates: {}", e);
            println!();
            println!("You can check manually at:");
            println!("  https://github.com/forvercart0-oss/Luna/releases");
        }
    }
}

fn detect_platform() -> (String, String) {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    
    let platform = match os {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "macos",
        _ => "unknown",
    };
    
    let arch_str = match arch {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "x86" => "i686",
        _ => arch,
    };
    
    (platform.to_string(), arch_str.to_string())
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Update => cmd_update(),
        Commands::Version => cmd_version(),
        Commands::Doctor => cmd_doctor(),
    }
}
