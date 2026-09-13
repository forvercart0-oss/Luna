# Changelog

All notable changes to LUNA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- In-app update checking via `check_for_updates` Tauri command
- Update store with 6-hour check interval, startup check, manual check from Settings
- Update banner notification when a newer version is available
- Updates tab in Settings with version info and "Check for Updates" button
- Models and API Credentials pages added to sidebar navigation
- 5 unit tests for update command (version comparison, platform detection)

### Fixed
- CLI `luna update` now handles 404 gracefully ("No published release is currently available")
- CI auto-creates releases when version changes in Cargo.toml (not just on tag push)
- CI generates `latest.json` with real SHA-256 checksums from built artifacts
- Provider "Set Active" button now actually works (was passing `is_active` but backend ignored it)
- Select/dropdown colors fixed for dark theme (options now have dark background, light text)
- HomeScreen status line now shows actual model name and assistant state (was hardcoded "Voice: Kokoro")
- Panel toggle arrows (▾) removed — they had no click handler

### Changed
- TopBar simplified: removed dead Microphone, Voice, Settings buttons and decorative window dots
- Sidebar now includes all reachable pages (Models, API Credentials were unreachable before)
- Language dropdown only shows "English" (removed fake "Urdu" option that did nothing)
- Theme dropdown removed (only dark theme exists in CSS)
- System stats show "Unavailable" instead of "—" when data cannot be obtained

### Removed
- Dead Microphone button from TopBar and HomeScreen (no handler)
- Dead Voice dropdown button from TopBar (no handler)
- Dead Settings button from TopBar (sidebar already has Settings)
- Dead window control dots from TopBar (decorative, no handlers)
- Dead panel toggle arrows from HomeScreen (cursor: pointer but no click handler)
- Dead `_current_exe` variable from CLI `install_appimage`
- Dead `VoiceConfig` struct from voice.rs
- Dead `AiProvider` trait, `StreamDelta`, `StreamChoice`, `StreamResponse` from providers/mod.rs
- Dead `ProviderManager::set_active` and `ProviderManager::get_key_by_id` methods
- Dead `UpdateProviderAccount.is_active` field (now actually used for provider activation)
- Dead topbar-voice, topbar-window, topbar-btn, composer-btn CSS rules
- Duplicate model-selector CSS (now defined alongside form-select)

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
- `luna version` — Display version information
- `luna doctor` — Run diagnostic checks
- `luna update` — Check for and install updates from GitHub Releases

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
- No splash screen on startup
