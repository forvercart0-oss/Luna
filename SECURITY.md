# Security Policy

## Credential Storage

LUNA stores API keys and credentials locally in an SQLite database.

- Keys are masked in the UI (first 4 + last 4 characters)
- Keys are transmitted only to their intended API endpoint (OpenRouter)
- Keys are never logged to console, files, or network
- The database file is stored in the user's application data directory

### Recommendations

- Never share your API keys
- Never commit API keys to version control
- Use environment variables for CI/CD
- Rotate keys if compromised

## Permission System

LUNA implements a tool permission system:

| Level | Behavior |
|-------|----------|
| `allowed` | Tool executes without user confirmation |
| `denied` | Tool is blocked, execution refused |
| `ask` | User is prompted before each execution |

Configure per-tool permissions in **Settings → Permissions**.

## Terminal Command Restrictions

The `terminal.execute` tool runs shell commands. Consider:
- Setting permission to `ask` for untrusted operations
- Reviewing commands before approving execution
- Using the audit log to monitor command history

## Audit Logging

All tool executions and significant operations are logged:
- Timestamp
- Action type
- Tool identifier
- Arguments summary
- Result summary
- Success/failure
- Permission state

View logs in **Settings → Activity**.

## API Communication

- All API requests use HTTPS
- OpenRouter requests include `Authorization: Bearer <key>` header
- Request/response bodies are not logged
- Timeouts prevent hanging connections

## Update Verification

- Updates are downloaded from GitHub Releases over HTTPS
- The CLI checks for updates via the GitHub API
- Artifact integrity is verified through SHA-256 checksums published with releases

## What We Do NOT Do

- We do not collect telemetry
- We do not send data to third parties
- We do not store credentials in plain text configuration files
- We do not execute arbitrary remote code
- We do not require external services beyond OpenRouter

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly:

1. Do not disclose publicly
2. Open an issue at [GitHub Issues](https://github.com/forvercart0-oss/Luna/issues) with the `security` label
3. Include steps to reproduce
4. Allow time for a fix before public disclosure

## Data Storage Location

LUNA stores its database in the platform's standard application data directory:

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/luna/luna.db` |
| macOS | `~/Library/Application Support/com.luna.desktop/luna.db` |
| Windows | `%APPDATA%\com.luna.desktop\data\luna.db` |

Updates never modify or delete the database. User data persists across versions.
