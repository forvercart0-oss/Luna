CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    action TEXT NOT NULL,
    tool_id TEXT,
    arguments_summary TEXT,
    result_summary TEXT,
    success INTEGER NOT NULL DEFAULT 1,
    permission_state TEXT,
    metadata TEXT
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_tool ON audit_logs(tool_id);
