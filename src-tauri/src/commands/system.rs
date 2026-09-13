use std::process::Command;

#[derive(serde::Serialize)]
pub struct SystemStats {
    pub cpu_usage: Option<f64>,
    pub memory_used_mb: Option<u64>,
    pub memory_total_mb: Option<u64>,
    pub disk_used_gb: Option<f64>,
    pub disk_total_gb: Option<f64>,
    pub hostname: Option<String>,
    pub os: Option<String>,
}

#[tauri::command]
pub async fn get_system_stats() -> Result<SystemStats, String> {
    let mut stats = SystemStats {
        cpu_usage: None,
        memory_used_mb: None,
        memory_total_mb: None,
        disk_used_gb: None,
        disk_total_gb: None,
        hostname: None,
        os: None,
    };

    // CPU usage from /proc/stat
    if let Ok(content) = std::fs::read_to_string("/proc/stat") {
        if let Some(line) = content.lines().next() {
            if line.starts_with("cpu ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let idle = parts[4].parse::<u64>().unwrap_or(0);
                    let total: u64 = parts[1..].iter()
                        .filter_map(|p| p.parse::<u64>().ok())
                        .sum();
                    if total > 0 {
                        stats.cpu_usage = Some(((total - idle) as f64 / total as f64) * 100.0);
                    }
                }
            }
        }
    }

    // Memory from /proc/meminfo
    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        let mut total_kb = 0u64;
        let mut avail_kb = 0u64;
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
            }
            if line.starts_with("MemAvailable:") {
                avail_kb = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
            }
        }
        if total_kb > 0 {
            stats.memory_total_mb = Some(total_kb / 1024);
            stats.memory_used_mb = Some((total_kb - avail_kb) / 1024);
        }
    }

    // Disk usage from df
    if let Ok(output) = Command::new("df").arg("-BM").arg("/").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let total = parts[1].trim_end_matches('M').parse::<f64>().unwrap_or(0.0);
                let used = parts[2].trim_end_matches('M').parse::<f64>().unwrap_or(0.0);
                stats.disk_total_gb = Some(total / 1024.0);
                stats.disk_used_gb = Some(used / 1024.0);
            }
        }
    }

    // Hostname
    if let Ok(name) = hostname::get() {
        stats.hostname = Some(name.to_string_lossy().to_string());
    }

    // OS
    if let Ok(output) = Command::new("uname").arg("-sr").output() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        stats.os = Some(stdout);
    }

    Ok(stats)
}
