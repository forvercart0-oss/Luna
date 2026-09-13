CREATE TABLE IF NOT EXISTS tool_permissions (
    id TEXT PRIMARY KEY,
    tool_id TEXT NOT NULL,
    permission TEXT NOT NULL CHECK (permission IN ('allowed', 'denied', 'ask')),
    scope TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_tool_permissions_tool ON tool_permissions(tool_id, scope);
