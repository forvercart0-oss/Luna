-- API Catalog: stores public API entries from public-apis repository
CREATE TABLE IF NOT EXISTS api_catalog (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    auth_type TEXT NOT NULL DEFAULT 'none',
    https INTEGER NOT NULL DEFAULT 1,
    cors TEXT NOT NULL DEFAULT 'unknown',
    category TEXT NOT NULL,
    homepage TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    health_status TEXT NOT NULL DEFAULT 'unknown',
    last_health_check TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_api_catalog_category ON api_catalog(category);
CREATE INDEX IF NOT EXISTS idx_api_catalog_auth_type ON api_catalog(auth_type);
CREATE INDEX IF NOT EXISTS idx_api_catalog_name ON api_catalog(name);

-- API Catalog Metadata: sync state
CREATE TABLE IF NOT EXISTS api_catalog_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- API Credentials: generic credential storage per provider
CREATE TABLE IF NOT EXISTS api_credentials (
    id TEXT PRIMARY KEY,
    provider_id TEXT NOT NULL,
    credential_type TEXT NOT NULL,
    credential_key TEXT NOT NULL,
    credential_value_encrypted TEXT NOT NULL,
    label TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_api_credentials_provider ON api_credentials(provider_id);

-- API Tool Definitions: dynamic tools derived from catalog
CREATE TABLE IF NOT EXISTS api_tool_definitions (
    id TEXT PRIMARY KEY,
    catalog_id TEXT NOT NULL,
    tool_name TEXT NOT NULL,
    tool_description TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL DEFAULT 'GET',
    parameters TEXT,
    auth_required INTEGER NOT NULL DEFAULT 0,
    enabled INTEGER NOT NULL DEFAULT 1,
    category TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (catalog_id) REFERENCES api_catalog(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_api_tool_definitions_catalog ON api_tool_definitions(catalog_id);
CREATE INDEX IF NOT EXISTS idx_api_tool_definitions_category ON api_tool_definitions(category);

-- API Health Log: track health check history
CREATE TABLE IF NOT EXISTS api_health_log (
    id TEXT PRIMARY KEY,
    catalog_id TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms INTEGER,
    error_message TEXT,
    checked_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (catalog_id) REFERENCES api_catalog(id) ON DELETE CASCADE
);

-- Activity Events: detailed activity tracking
CREATE TABLE IF NOT EXISTS activity_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    detail TEXT,
    status TEXT NOT NULL DEFAULT 'running',
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_activity_events_type ON activity_events(event_type);
CREATE INDEX IF NOT EXISTS idx_activity_events_created ON activity_events(created_at);
