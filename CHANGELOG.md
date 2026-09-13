# Changelog

All notable changes to LUNA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.1.0] - 2026-09-13

### Added

#### Core
- Initial LUNA desktop application architecture
- Tauri 2 framework with Rust backend and React frontend
- SQLite database with WAL mode (9 migration files, 13 tables)
- 48 Tauri IPC commands

#### AI Chat
- OpenRouter integration for AI model access
- Multiple provider account management
- Model profiles with system prompts, temperature, max tokens
- Streaming SSE responses with real-time token display
- Stop generation (cancel streaming mid-response)
- Persistent conversation history
- Context window management

#### Memory
- Persistent memory store in SQLite
- CRUD operations with search
- Importance levels and source tracking
- Memory types (fact, preference, context, instruction)

#### Tools
- Tool registry with 5 built-in tools
- Filesystem tools (read, write, list)
- Terminal execution
- System information
- Permission system (allowed/denied/ask per tool)
- Audit logging of all tool executions

#### API Catalog
- Public APIs directory integration (~1,400 APIs)
- Sync from public-apis repository
- Search and filter by name, category, auth type
- API tool definitions from catalog entries
- Health tracking

#### API Credentials
- Multi-provider credential storage
- Credential testing
- Active credential selection

#### Voice
- Kokoro TTS provider
- Whisper STT provider
- Voice availability checks
- TTS/STT configuration

#### Desktop Experience
- Thinking Orb with state mapping (idle, thinking, working, searching, listening, speaking, error)
- Sound effects via Web Audio API (6 distinct sounds)
- Activity timeline with real-time events
- System stats (CPU, memory, storage, hostname, OS)
- Dark theme with teal accent (JARVIS-inspired)

#### Settings
- General settings (theme, language)
- AI provider management
- Voice configuration
- Permission management

#### CLI
- `luna-cli version` — Display version information
- `luna-cli doctor` — Run diagnostic checks
- `luna-cli update` — Check for updates from GitHub Releases

#### Branding
- LUNA desktop icon (lunar crescent + AI core design)
- All required icon sizes (16x16 through 512x512)
- Windows ICO format
- macOS ICNS format

#### CI/CD
- GitHub Actions workflow
- Rust checks (cargo check, clippy)
- Frontend checks (typecheck, lint, build)
- Cross-platform Tauri builds (Windows, Linux, macOS)
- Automated release creation
- Checksum generation
- Update manifest (latest.json)

#### Documentation
- Professional README with architecture diagram
- SECURITY.md with credential storage and permission documentation
- CONTRIBUTING.md with development guidelines
- CHANGELOG.md

### Known Limitations

- System stats implementation reads Linux-specific paths (`/proc/stat`, `/proc/meminfo`)
- Credential storage uses XOR obfuscation (not OS keychain)
- Agent engine has task/step management but no frontend UI
- API tool execution not wired through the tool registry
- No system tray support
- No native notifications
- No automatic in-app update (CLI update check available)
- No splash screen on startup
