use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::permissions::PermissionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub permission_level: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub tool_id: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub tool_id: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

pub struct ToolRegistry {
    tools: HashMap<String, ToolDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut tools = HashMap::new();
        Self::register_builtin_tools(&mut tools);
        Self { tools }
    }

    fn register_builtin_tools(tools: &mut HashMap<String, ToolDefinition>) {
        let builtins = vec![
            ToolDefinition {
                id: "filesystem.read".to_string(),
                name: "Read File".to_string(),
                description: "Read contents of a file".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File path to read" }
                    },
                    "required": ["path"]
                }),
                permission_level: "filesystem.read".to_string(),
                category: "filesystem".to_string(),
            },
            ToolDefinition {
                id: "filesystem.write".to_string(),
                name: "Write File".to_string(),
                description: "Write content to a file".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File path to write" },
                        "content": { "type": "string", "description": "Content to write" }
                    },
                    "required": ["path", "content"]
                }),
                permission_level: "filesystem.write".to_string(),
                category: "filesystem".to_string(),
            },
            ToolDefinition {
                id: "filesystem.list".to_string(),
                name: "List Directory".to_string(),
                description: "List files in a directory".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Directory path" }
                    },
                    "required": ["path"]
                }),
                permission_level: "filesystem.read".to_string(),
                category: "filesystem".to_string(),
            },
            ToolDefinition {
                id: "terminal.execute".to_string(),
                name: "Execute Command".to_string(),
                description: "Execute a terminal command".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "Command to execute" },
                        "cwd": { "type": "string", "description": "Working directory" },
                        "timeout_ms": { "type": "integer", "description": "Timeout in milliseconds" }
                    },
                    "required": ["command"]
                }),
                permission_level: "terminal.execute".to_string(),
                category: "terminal".to_string(),
            },
            ToolDefinition {
                id: "system.info".to_string(),
                name: "System Info".to_string(),
                description: "Get system information".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
                permission_level: "system.info".to_string(),
                category: "system".to_string(),
            },
        ];

        for tool in builtins {
            tools.insert(tool.id.clone(), tool);
        }
    }

    pub fn register_api_tool(&mut self, def: ToolDefinition) {
        self.tools.insert(def.id.clone(), def);
    }

    pub fn unregister_api_tool(&mut self, id: &str) {
        self.tools.remove(id);
    }

    pub fn get_all(&self) -> Vec<&ToolDefinition> {
        self.tools.values().collect()
    }

    pub fn get(&self, id: &str) -> Option<&ToolDefinition> {
        self.tools.get(id)
    }

    pub async fn execute(
        &self,
        call: &ToolCall,
        permission_manager: &PermissionManager,
    ) -> Result<ToolResult> {
        let tool = self.tools.get(&call.tool_id)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", call.tool_id))?;

        let perm = permission_manager.check(&tool.permission_level, None)?;

        if perm == "denied" {
            return Ok(ToolResult {
                call_id: call.id.clone(),
                tool_id: call.tool_id.clone(),
                success: false,
                output: String::new(),
                error: Some("Permission denied".to_string()),
            });
        }

        match call.tool_id.as_str() {
            "system.info" => {
                let info = serde_json::json!({
                    "os": std::env::consts::OS,
                    "arch": std::env::consts::ARCH,
                    "exe": std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default(),
                });
                Ok(ToolResult {
                    call_id: call.id.clone(),
                    tool_id: call.tool_id.clone(),
                    success: true,
                    output: serde_json::to_string_pretty(&info)?,
                    error: None,
                })
            }
            "filesystem.read" => {
                let path = call.arguments["path"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
                match std::fs::read_to_string(path) {
                    Ok(content) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: true,
                        output: content,
                        error: None,
                    }),
                    Err(e) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to read {}: {}", path, e)),
                    }),
                }
            }
            "filesystem.write" => {
                let path = call.arguments["path"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
                let content = call.arguments["content"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'content' argument"))?;
                match std::fs::write(path, content) {
                    Ok(()) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: true,
                        output: format!("Written {} bytes to {}", content.len(), path),
                        error: None,
                    }),
                    Err(e) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to write {}: {}", path, e)),
                    }),
                }
            }
            "filesystem.list" => {
                let path = call.arguments["path"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
                match std::fs::read_dir(path) {
                    Ok(entries) => {
                        let mut items: Vec<String> = Vec::new();
                        for entry in entries {
                            if let Ok(e) = entry {
                                let file_type = e.file_type().map(|ft| {
                                    if ft.is_dir() { "dir" } else { "file" }
                                }).unwrap_or("unknown");
                                items.push(format!("{} [{}]", e.file_name().display(), file_type));
                            }
                        }
                        items.sort();
                        Ok(ToolResult {
                            call_id: call.id.clone(),
                            tool_id: call.tool_id.clone(),
                            success: true,
                            output: items.join("\n"),
                            error: None,
                        })
                    }
                    Err(e) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to list {}: {}", path, e)),
                    }),
                }
            }
            "terminal.execute" => {
                let command = call.arguments["command"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'command' argument"))?;
                let cwd = call.arguments["cwd"].as_str().map(|s| s.to_string());
                let timeout_ms = call.arguments["timeout_ms"].as_u64().unwrap_or(30000);

                let mut cmd = std::process::Command::new("sh");
                cmd.arg("-c").arg(command);
                if let Some(ref dir) = cwd {
                    cmd.current_dir(dir);
                }

                let output = tokio::time::timeout(
                    std::time::Duration::from_millis(timeout_ms),
                    tokio::task::spawn_blocking(move || cmd.output()),
                ).await;

                match output {
                    Ok(Ok(Ok(out))) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                        let success = out.status.success();
                        let mut result = stdout;
                        if !stderr.is_empty() {
                            if !result.is_empty() {
                                result.push_str("\n--- stderr ---\n");
                            }
                            result.push_str(&stderr);
                        }
                        Ok(ToolResult {
                            call_id: call.id.clone(),
                            tool_id: call.tool_id.clone(),
                            success,
                            output: result,
                            error: if !success { Some(format!("Exit code: {}", out.status)) } else { None },
                        })
                    }
                    Ok(Ok(Err(e))) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some(format!("Command execution failed: {}", e)),
                    }),
                    Ok(Err(_)) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some("Command timed out".to_string()),
                    }),
                    Err(_) => Ok(ToolResult {
                        call_id: call.id.clone(),
                        tool_id: call.tool_id.clone(),
                        success: false,
                        output: String::new(),
                        error: Some("Command timed out".to_string()),
                    }),
                }
            }
            _ => Ok(ToolResult {
                call_id: call.id.clone(),
                tool_id: call.tool_id.clone(),
                success: false,
                output: String::new(),
                error: Some(format!("Tool '{}' not implemented", call.tool_id)),
            }),
        }
    }
}
