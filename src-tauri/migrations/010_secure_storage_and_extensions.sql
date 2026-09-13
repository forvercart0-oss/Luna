-- 010_secure_storage_and_extensions
-- Add secure_storage column to provider_accounts
-- This column stores either "keyring" (key stored in OS keyring, id is the ref)
-- or "obfuscated" (XOR-obfuscated key stored directly in api_key_encrypted)
ALTER TABLE provider_accounts ADD COLUMN secure_storage TEXT NOT NULL DEFAULT 'obfuscated';

-- Update existing rows: if api_key_encrypted is non-empty, mark as obfuscated
UPDATE provider_accounts SET secure_storage = 'obfuscated' WHERE secure_storage = 'obfuscated' AND api_key_encrypted != '';

-- Table for screen awareness settings
CREATE TABLE IF NOT EXISTS screen_settings (
    id TEXT PRIMARY KEY,
    enabled INTEGER NOT NULL DEFAULT 0,
    mode TEXT NOT NULL DEFAULT 'monitor' CHECK (mode IN ('monitor', 'window', 'region', 'monitor_all')),
    monitor_index INTEGER,
    region_x INTEGER,
    region_y INTEGER,
    region_width INTEGER,
    region_height INTEGER,
    capture_interval_ms INTEGER NOT NULL DEFAULT 5000,
    change_detection INTEGER NOT NULL DEFAULT 1,
    quality INTEGER NOT NULL DEFAULT 80 CHECK (quality BETWEEN 1 AND 100),
    resolution_scale REAL NOT NULL DEFAULT 1.0 CHECK (resolution_scale BETWEEN 0.1 AND 2.0),
    processing_mode TEXT NOT NULL DEFAULT 'local' CHECK (processing_mode IN ('local', 'external')),
    last_capture_at TEXT,
    status TEXT NOT NULL DEFAULT 'off' CHECK (status IN ('off', 'on', 'paused', 'error')),
    error_message TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Table for proactive copilot
CREATE TABLE IF NOT EXISTS proactive_settings (
    id TEXT PRIMARY KEY,
    enabled INTEGER NOT NULL DEFAULT 0,
    sensitivity REAL NOT NULL DEFAULT 0.7 CHECK (sensitivity BETWEEN 0.1 AND 1.0),
    cooldown_minutes INTEGER NOT NULL DEFAULT 5,
    quiet_hours_start TEXT,
    quiet_hours_end TEXT,
    do_not_disturb INTEGER NOT NULL DEFAULT 0,
    voice_suggestions INTEGER NOT NULL DEFAULT 0,
    visual_suggestions INTEGER NOT NULL DEFAULT 1,
    last_suggestion_at TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Table for desktop automation
CREATE TABLE IF NOT EXISTS automation_settings (
    id TEXT PRIMARY KEY,
    permission_level TEXT NOT NULL DEFAULT 'ask' CHECK (permission_level IN ('observe', 'suggest', 'ask', 'approved')),
    confirmation_required INTEGER NOT NULL DEFAULT 1,
    emergency_stop_key TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Table for automation task plans
CREATE TABLE IF NOT EXISTS automation_tasks (
    id TEXT PRIMARY KEY,
    conversation_id TEXT,
    goal TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    plan TEXT,
    result TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_automation_tasks_status ON automation_tasks(status);
CREATE INDEX IF NOT EXISTS idx_automation_tasks_created ON automation_tasks(created_at);

-- Table for automation action steps
CREATE TABLE IF NOT EXISTS automation_steps (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    step_order INTEGER NOT NULL,
    description TEXT NOT NULL,
    tool_id TEXT,
    tool_args TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'skipped', 'cancelled')),
    observation TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (task_id) REFERENCES automation_tasks(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_automation_steps_task ON automation_steps(task_id, step_order);

-- Table for system information cache
CREATE TABLE IF NOT EXISTS system_info_cache (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL,
    cached_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Table for update state
CREATE TABLE IF NOT EXISTS update_state (
    id TEXT PRIMARY KEY,
    current_version TEXT NOT NULL,
    latest_version TEXT,
    last_checked TEXT,
    auto_download INTEGER NOT NULL DEFAULT 0,
    auto_install INTEGER NOT NULL DEFAULT 0
);

-- Table for clipboard history
CREATE TABLE IF NOT EXISTS clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_clipboard_created ON clipboard_history(created_at DESC);

-- Table for notifications log
CREATE TABLE IF NOT EXISTS notifications_log (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT,
    icon TEXT,
    category TEXT,
    clicked INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_notifications_created ON notifications_log(created_at DESC);
