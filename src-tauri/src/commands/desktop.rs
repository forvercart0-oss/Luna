use crate::AppState;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardContent {
    pub content: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotResult {
    pub base64: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub monitors: Vec<MonitorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}

// ── Clipboard ──

#[tauri::command]
pub async fn read_clipboard(state: State<'_, AppState>) -> Result<ClipboardContent, String> {
    let content = if is_wayland() {
        let output = Command::new("wl-paste")
            .output()
            .map_err(|e| format!("Failed to read clipboard: {}", e))?;
        if !output.status.success() {
            return Err("Failed to read clipboard".to_string());
        }
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        let output = Command::new("xclip")
            .args(["-o", "-selection", "clipboard"])
            .output()
            .map_err(|e| format!("Failed to read clipboard: {}", e))?;
        if !output.status.success() {
            return Err("Failed to read clipboard".to_string());
        }
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let _ = state.audit_log.read().await.log(
        "clipboard.read",
        None,
        None,
        Some(&format!("{} chars", content.len())),
        true,
        Some("allowed"),
    );

    Ok(ClipboardContent {
        content,
        content_type: "text".to_string(),
    })
}

#[tauri::command]
pub async fn write_clipboard(
    state: State<'_, AppState>,
    content: String,
) -> Result<(), String> {
    if is_wayland() {
        let mut child = Command::new("wl-copy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to write clipboard: {}", e))?;

        if let Some(stdin) = &mut child.stdin {
            use std::io::Write;
            stdin.write_all(content.as_bytes()).map_err(|e| format!("Failed to write clipboard: {}", e))?;
        }
        child.wait().map_err(|e| format!("Failed to write clipboard: {}", e))?;
    } else {
        let mut child = Command::new("xclip")
            .args(["-i", "-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to write clipboard: {}", e))?;

        if let Some(stdin) = &mut child.stdin {
            use std::io::Write;
            stdin.write_all(content.as_bytes()).map_err(|e| format!("Failed to write clipboard: {}", e))?;
        }
        child.wait().map_err(|e| format!("Failed to write clipboard: {}", e))?;
    }

    let _ = state.audit_log.read().await.log(
        "clipboard.write",
        None,
        None,
        Some(&format!("{} chars", content.len())),
        true,
        Some("allowed"),
    );

    Ok(())
}

#[tauri::command]
pub async fn get_clipboard_history(state: State<'_, AppState>) -> Result<Vec<ClipboardContent>, String> {
    let rows = state.db.query_map(
        "SELECT content, content_type FROM clipboard_history ORDER BY created_at DESC LIMIT 50",
        &[],
        |row| {
            Ok(ClipboardContent {
                content: row.get(0)?,
                content_type: row.get(1)?,
            })
        },
    ).map_err(|e| e.to_string())?;

    Ok(rows)
}

// ── Notifications ──

#[tauri::command]
pub async fn send_notification(
    state: State<'_, AppState>,
    title: String,
    body: String,
) -> Result<(), String> {
    let _ = state.audit_log.read().await.log(
        "notification.sent",
        None,
        Some(&title),
        Some(&body),
        true,
        Some("allowed"),
    );

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("notify-send")
            .args(["--app-name=LUNA", &title, &body])
            .output();
        match output {
            Ok(o) if o.status.success() => Ok(()),
            Ok(o) => Err(format!("notify-send failed: {}", String::from_utf8_lossy(&o.stderr))),
            Err(e) => Err(format!("notify-send command failed: {}", e)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("msg")
                .args(["*", &format!("{}: {}", title, body)])
                .output();
            Ok(())
        }
        #[cfg(target_os = "macos")]
        {
            let script = format!(
                r#"display notification "{}" with title "{}""#,
                body.replace('"', "\\\""),
                title.replace('"', "\\\"")
            );
            let output = Command::new("osascript")
                .args(["-e", &script])
                .output();
            match output {
                Ok(o) if o.status.success() => Ok(()),
                Ok(o) => Err(format!("osascript failed: {}", String::from_utf8_lossy(&o.stderr))),
                Err(e) => Err(format!("Notification failed: {}", e)),
            }
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err("Notifications not supported on this platform".to_string())
        }
    }
}

// ── Screen Capture ──

fn is_wayland() -> bool {
    std::env::var("XDG_SESSION_TYPE")
        .map(|v| v == "wayland")
        .unwrap_or(false)
        || std::env::var("WAYLAND_DISPLAY").is_ok()
}

#[tauri::command]
pub async fn get_screen_info() -> Result<ScreenInfo, String> {
    let monitors: Vec<MonitorInfo> = if is_waylist() {
        parse_hyprland_monitors()
    } else if std::env::var("DISPLAY").is_ok() {
        parse_xrandr_monitors()
    } else {
        Vec::new()
    };

    Ok(ScreenInfo { monitors })
}

fn is_waylist() -> bool {
    is_wayland()
}

#[tauri::command]
pub async fn capture_screenshot(
    state: State<'_, AppState>,
    monitor_index: Option<u32>,
) -> Result<ScreenshotResult, String> {
    let _ = state.audit_log.read().await.log(
        "screen.capture",
        None,
        None,
        Some(&format!("monitor_index={:?}", monitor_index)),
        true,
        Some("allowed"),
    );

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();

    let tmp_path = std::env::temp_dir().join(format!("luna-screenshot-{}.png", timestamp));
    let tmp_str = tmp_path.to_string_lossy().to_string();

    let cmd_result = if is_wayland() {
        let mut cmd = Command::new("grim");
        if let Some(idx) = monitor_index {
            cmd.args(["-o", &format!("eDP-{}", idx)]);
        }
        cmd.arg(&tmp_str);
        cmd.output()
    } else {
        let mut cmd = Command::new("import");
        if let Some(idx) = monitor_index {
            cmd.arg("-window");
            cmd.arg(format!("root-{}", idx));
        }
        cmd.arg(&tmp_str);
        cmd.output()
    };

    match cmd_result {
        Ok(o) if o.status.success() => {
            let img = image::open(&tmp_path)
                .map_err(|e| format!("Failed to read screenshot: {}", e))?;
            let w = img.width();
            let h = img.height();
            let format = img.color();

            let mut buf = Vec::new();
            {
                let mut cursor = std::io::Cursor::new(&mut buf);
                img.write_to(&mut cursor, image::ImageFormat::Png)
                    .map_err(|e| format!("Failed to encode: {}", e))?;
            }

            let _ = std::fs::remove_file(&tmp_path);

            let _ = state.db.execute(
                "INSERT INTO clipboard_history (content, content_type) VALUES (?1, ?2)",
                &[
                    &base64::encode(&buf) as &dyn rusqlite::types::ToSql,
                    &"image/png",
                ],
            );

            Ok(ScreenshotResult {
                base64: base64::encode(&buf),
                width: w,
                height: h,
                format: format!("{:?}", format),
            })
        }
        Ok(o) => {
            let _ = std::fs::remove_file(&tmp_path);
            Err(format!(
                "Screenshot failed ({}): {}",
                o.status,
                String::from_utf8_lossy(&o.stderr)
            ))
        }
        Err(e) => Err(format!("Screenshot tool not available: {}", e)),
    }
}

fn parse_hyprland_monitors() -> Vec<MonitorInfo> {
    let output = Command::new("hyprctl")
        .args(["-j", "monitors"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            match serde_json::from_str::<Vec<serde_json::Value>>(&text) {
                Ok(monitors) => monitors
                    .iter()
                    .enumerate()
                    .map(|(i, m)| MonitorInfo {
                        name: m["name"].as_str().unwrap_or("unknown").to_string(),
                        x: m["x"].as_i64().unwrap_or(0) as i32,
                        y: m["y"].as_i64().unwrap_or(0) as i32,
                        width: m["width"].as_u64().unwrap_or(0) as u32,
                        height: m["height"].as_u64().unwrap_or(0) as u32,
                        primary: i == 0,
                    })
                    .collect(),
                Err(_) => Vec::new(),
            }
        }
        _ => Vec::new(),
    }
}

fn parse_xrandr_monitors() -> Vec<MonitorInfo> {
    let output = Command::new("xrandr")
        .args(["--query"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            let mut monitors = Vec::new();
            for line in text.lines() {
                if line.contains(" connected") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(name) = parts.first() {
                        let mut x = 0;
                        let mut y = 0;
                        let mut w = 0u32;
                        let mut h = 0u32;
                        for part in &parts[1..] {
                            if let Some(_pos) = part.find('+') {
                                let dims: Vec<&str> = part.split('+').collect();
                                if dims.len() >= 3 {
                                    let dim_parts: Vec<&str> = dims[0].split('x').collect();
                                    if dim_parts.len() >= 2 {
                                        w = dim_parts[0].parse::<u32>().unwrap_or(0);
                                        h = dim_parts[1].parse::<u32>().unwrap_or(0);
                                    }
                                    x = dims[1].parse::<i32>().unwrap_or(0);
                                    y = dims[2].parse::<i32>().unwrap_or(0);
                                }
                            }
                        }
                        if w > 0 && h > 0 {
                            monitors.push(MonitorInfo {
                                name: name.to_string(),
                                x,
                                y,
                                width: w,
                                height: h,
                                primary: x == 0 && y == 0,
                            });
                        }
                    }
                }
            }
            monitors
        }
        _ => Vec::new(),
    }
}

// ── File Operations ──

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut result: Vec<String> = Vec::new();
    for entry in entries {
        if let Ok(e) = entry {
            let file_type = e.file_type().map(|ft| {
                if ft.is_dir() { "dir" } else { "file" }
            }).unwrap_or("unknown");
            result.push(format!("{} [{}]", e.file_name().display(), file_type));
        }
    }
    result.sort();
    Ok(result)
}

#[tauri::command]
pub async fn file_exists(path: String) -> Result<bool, String> {
    Ok(std::path::Path::new(&path).exists())
}
