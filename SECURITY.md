# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in LUNA, please report it responsibly.

Do NOT open a public GitHub issue for security vulnerabilities.

## Security Model

### Credential Storage

- API keys are stored in local SQLite database
- Keys are never logged or exposed in error messages
- API key display is masked (e.g., `sk-o...xxxx`)

### Permission System

- All tool operations check permissions before execution
- Dangerous operations require explicit user confirmation
- Permission states: `allowed`, `denied`, `ask`

### Audit Logging

- All significant actions are logged
- Audit logs include: action, tool, arguments, result, permission state
- Logs never contain secrets or API keys

### Input Validation

- All user inputs are validated
- Model-generated tool calls are treated as untrusted
- SQL injection is prevented via parameterized queries

## Scope

- LUNA desktop application
- Tauri backend
- React frontend
- SQLite database layer
- Tool execution system
