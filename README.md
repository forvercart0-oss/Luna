# LUNA

A cross-platform desktop AI assistant built with Tauri 2, React, TypeScript, Rust, and SQLite.

LUNA connects to [OpenRouter](https://openrouter.ai/) for conversational AI, stores conversations and memories locally in SQLite, and provides system tools, voice interaction, and an API catalog — all from a dark-themed desktop interface.

## Table of Contents

- [Features](#features)
- [Platform Support](#platform-support)
- [Installation](#installation)
- [First Launch](#first-launch)
- [CLI](#cli)
- [OpenRouter Setup](#openrouter-setup)
- [Model Profiles](#model-profiles)
- [Conversations](#conversations)
- [Memory](#memory)
- [Tools and Permissions](#tools-and-permissions)
- [API Catalog](#api-catalog)
- [Voice](#voice)
- [Desktop Experience](#desktop-experience)
- [Settings](#settings)
- [Updates](#updates)
- [Offline Mode](#offline-mode)
- [User Data](#user-data)
- [Backup](#backup)
- [Uninstall](#uninstall)
- [Security](#security)
- [Privacy](#privacy)
- [Troubleshooting](#troubleshooting)
- [Development](#development)
- [Architecture](#architecture)
- [Database](#database)
- [Release Process](#release-process)
- [Contributing](#contributing)
- [FAQ](#faq)
- [License](#license)

---

## Features

### AI Chat

- OpenRouter integration — access hundreds of AI models through a single API
- Multiple provider accounts — manage several OpenRouter API keys
- Model profiles — configure system prompts, temperature, and max tokens per model
- Streaming responses — real-time token-by-token output via SSE
- Stop generation — cancel a streaming response mid-flight
- Persistent conversations — all chat history stored in local SQLite

### Conversations

- Create, rename, and delete conversations
- Full message history with markdown rendering and syntax-highlighted code blocks
- Model selector per conversation

### Memory

- Persistent memory store in SQLite
- Create, edit, search, and delete memories
- Memory types: fact, preference, context, instruction, project, person, location, event
- Importance levels (1–10) and source tracking
- Tag-based organization

### Tools

- `filesystem.read` — read file contents
- `filesystem.write` — write content to files
- `filesystem.list` — list directory contents
- `terminal.execute` — execute shell commands (via `sh -c`, 30s timeout)
- `system.info` — CPU, memory, disk, hostname, OS
- Permission system: allowed / denied / ask per tool
- Audit logging of all tool executions

### API Catalog

- Sync from the [public-apis](https://github.com/public-apis/public-apis) directory (~1,400 APIs)
- Search and filter by name, category, and authentication type
- Create reusable tool definitions from catalog entries
- Credential storage and testing for API keys

### Voice

- Kokoro TTS — text-to-speech via Kokoro provider (optional, requires running server)
- Whisper STT — speech-to-text via Whisper provider (optional, requires running server)
- Configurable voice name, speed (0.5–2.0), and volume (0.0–1.0)

### Desktop Experience

- Thinking Orb — animated status indicator reflecting LUNA's current state (idle, thinking, working, searching, listening, speaking, error)
- Sound effects — audio feedback via Web Audio API oscillators (tied to TTS enabled setting)
- Activity timeline — real-time event feed
- System stats — live CPU, memory, storage monitoring (Linux; unavailable on other platforms)
- Dark theme with teal accent

---

## Platform Support

| Platform | Architecture | Status | Artifacts |
|----------|-------------|--------|-----------|
| Linux | x86_64 | Supported | `.AppImage`, `.deb` |
| Windows | x86_64 | Supported | `.msi`, `.exe` |
| macOS | Apple Silicon (aarch64) | Supported | `.app.tar.gz` |
| macOS | Intel (x86_64) | Not built | — |

macOS Intel is not currently built by CI. The Tauri configuration supports it, but no automated build produces Intel artifacts.

---

## Installation

**No Rust, Node.js, or development tools are required.** Download the pre-built release for your platform from [GitHub Releases](https://github.com/forvercart0-oss/Luna/releases).

### Linux

#### AppImage (any distribution)

```bash
chmod +x LUNA_0.1.0_amd64.AppImage
./LUNA_0.1.0_amd64.AppImage
```

#### .deb (Debian / Ubuntu)

```bash
sudo dpkg -i LUNA_0.1.0_amd64.deb
sudo apt-get install -f   # install missing dependencies if needed
```

#### Linux Desktop Icon

The AppImage and .deb do not automatically create a desktop launcher. To add LUNA to your application menu:

```bash
mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/luna.desktop << 'EOF'
[Desktop Entry]
Name=LUNA
Exec=/path/to/LUNA_0.1.0_amd64.AppImage
Icon=luna
Type=Application
Categories=Utility;
Terminal=false
EOF
```

Replace `/path/to/` with the actual path to the AppImage. If you installed via `.deb`, the binary is typically at `/opt/LUNA/luna-app` — use that path instead.

Then refresh your desktop's application cache:

- **GNOME:** log out and log back in, or run `update-desktop-database ~/.local/share/applications/`
- **KDE:** the menu usually updates automatically
- **XFCE:** run `xfce4-panel -r` or log out and back in

To pin LUNA to your dock or taskbar, open LUNA from the application menu and use your desktop environment's "Add to Favorites" or "Pin to Taskbar" option.

### Windows

1. Download `LUNA_0.1.0_x64-setup.exe` from [GitHub Releases](https://github.com/forvercart0-oss/Luna/releases)
2. Run the installer
3. If Windows SmartScreen appears, click "More info" then "Run anyway"
4. Choose installation directory (default: `C:\Program Files\LUNA`)
5. LUNA will appear in the Start Menu
6. Optionally select "Create Desktop Shortcut" if offered

### macOS

1. Download `LUNA_0.1.0_aarch64.app.tar.gz` from [GitHub Releases](https://github.com/forvercart0-oss/Luna/releases)
2. Extract the archive:
   ```bash
   tar -xzf LUNA_0.1.0_aarch64.app.tar.gz
   ```
3. Drag `LUNA.app` to your `/Applications` folder
4. On first launch, macOS may block the app. Go to **System Settings → Privacy & Security** and click "Open Anyway"
5. LUNA will appear in your Applications folder and Dock

---

## First Launch

1. Launch LUNA
2. The application initializes a SQLite database in the data directory (see [User Data](#user-data))
3. Go to **Settings → Providers** and add your OpenRouter API key
4. Go to **Models** (sidebar) and create a model profile with your preferred OpenRouter model ID
5. Set the model as active
6. Start chatting from the Home screen or the Conversations page

The system stats panel (CPU, memory, disk) only displays data on Linux. On other platforms it shows "Unavailable."

---

## CLI

The `luna` CLI is a standalone binary included in release artifacts. It is not automatically installed to your PATH.

### Installing the CLI

```bash
# Copy to a directory in your PATH
sudo cp luna /usr/local/bin/

# Or to your personal bin directory
mkdir -p ~/.local/bin
cp luna ~/.local/bin/
```

On Windows, copy `luna.exe` to a directory in your PATH, or add the directory to your PATH environment variable.

### Commands

| Command | Description |
|---------|-------------|
| `luna` | Launch LUNA desktop application (if in PATH) |
| `luna version` | Display version, platform, install directory, and data directory |
| `luna doctor` | Run diagnostic checks (CLI, database, credentials, connectivity, update manifest, voice) |
| `luna update` | Check for updates, download, verify checksum, and install |

### Examples

```bash
$ luna version
LUNA Desktop AI Assistant
Version:    0.1.0
Identifier: com.luna.desktop
Install:    /usr/local/bin
Data:       /home/user/.local/share/luna
Platform:   linux (x86_64)

$ luna doctor
LUNA Doctor
===========
[1/8] CLI binary .................. OK (/usr/local/bin/luna)
[2/8] Data directory ............. OK (/home/user/.local/share/luna)
[3/8] SQLite database ............ OK (524288 bytes)
[4/8] OpenRouter credentials ..... OK (1 account(s) configured)
[5/8] GitHub connectivity ......... OK (HTTP 200)
[6/8] OpenRouter API .............. OK
[7/8] Update manifest ............. OK (latest: v0.1.0)
[8/8] Voice services (optional) .. NOT RUNNING (optional)
Result: 6 passed, 0 failed, 2 skipped
LUNA is healthy.
```

---

## OpenRouter Setup

LUNA uses [OpenRouter](https://openrouter.ai/) to access AI models. An OpenRouter account and API key are required for AI chat.

### Setup

1. Create an account at [openrouter.ai](https://openrouter.ai/)
2. Generate an API key
3. In LUNA, go to **Settings → Providers**
4. Click **+ Add Key**
5. Enter a friendly name (e.g., "Personal", "Work")
6. Paste your API key
7. Optionally enter a custom base URL (defaults to `https://openrouter.ai/api/v1`)
8. Click **Add**
9. Click **Set Active** on the account you want to use

### Multiple Accounts

You can add multiple OpenRouter accounts with different API keys. Only one account is active at a time and is used for all requests.

### API Key Security

- API keys are stored locally in SQLite
- Keys are masked in the UI (only first 4 and last 4 characters shown)
- Keys are never sent to any server other than OpenRouter
- Keys are never logged
- Never commit API keys to version control or paste them into bug reports

---

## Model Profiles

Model profiles configure which AI model to use and how it behaves.

### Creating a Profile

1. Go to **Models** (sidebar)
2. Click **+ Add Model**
3. Enter a display name (e.g., "Luna Main")
4. Enter the OpenRouter model ID (e.g., `anthropic/claude-3.5-sonnet`)
5. Optionally set a system prompt, temperature (0.0–2.0), and max tokens
6. Click **Add**

### Fields

| Field | Description |
|-------|-------------|
| Name | Display name for this profile |
| Model ID | OpenRouter model identifier |
| System Prompt | Custom instructions for the model |
| Temperature | Creativity level (0.0 = deterministic, 2.0 = maximum creativity) |
| Max Tokens | Maximum response length |
| Enabled | Whether this profile appears in model selectors |

Set a profile as active by clicking **Set Active** on the profile card. Only one profile is active at a time.

---

## Conversations

### Creating Conversations

- From the Home screen, type a message and press Enter — a new conversation is created automatically
- From the Conversations page, click **+ New**

### Managing Conversations

- **Rename**: hover over a conversation, click the pencil icon, type a new name, press Enter
- **Delete**: hover over a conversation, click the X icon
- **Switch**: click any conversation in the sidebar to load its messages

### Chat Features

- **Markdown rendering** — messages support full markdown with syntax-highlighted code blocks
- **Streaming** — responses appear token by token in real time
- **Stop generation** — click the red stop button to cancel a streaming response
- **Model selector** — choose which model profile to use per message

---

## Memory

LUNA maintains a persistent memory store separate from conversations.

### Operations

- **Create**: go to **Memory → + Add Memory**, choose a type, enter content, set importance (1–10), add source and tags
- **Search**: use the search bar to find memories by keyword
- **Edit**: click **Edit** on any memory card
- **Delete**: click **Delete** on any memory card

### Memory Types

| Type | Purpose |
|------|---------|
| fact | Factual information |
| preference | User preferences |
| context | Contextual information |
| instruction | User instructions |
| project | Project-related information |
| person | Information about people |
| location | Location information |
| event | Event information |

---

## Tools and Permissions

### Built-in Tools

| Tool | Description | Permission |
|------|-------------|------------|
| `filesystem.read` | Read file contents | `filesystem.read` |
| `filesystem.write` | Write content to files | `filesystem.write` |
| `filesystem.list` | List directory contents | `filesystem.read` |
| `terminal.execute` | Execute shell commands | `terminal.execute` |
| `system.info` | Get CPU, memory, disk, hostname, OS | `system.info` |

### Permission Levels

| Level | Behavior |
|-------|----------|
| allowed | Execute without asking |
| denied | Never execute |
| ask | Prompt user before executing (default) |

Configure permissions in the **Tools** (Permissions) page. Click Allow, Deny, or Ask for each tool.

### Terminal Security

`terminal.execute` runs commands via `sh -c` with a 30-second timeout. All executions are logged in the audit trail. Be cautious with the "allowed" permission level for terminal commands.

### Audit Logging

All tool executions are logged with timestamp, tool ID, arguments, result, success/failure status, and permission state. View logs in the **Activity** page.

---

## API Catalog

LUNA integrates with the [public-apis](https://github.com/public-apis/public-apis) directory, which is a catalog of free APIs — not a credential provider.

### Syncing

Go to **API Catalog** and click **Sync from public-apis**. This imports ~1,400 API entries from the public-apis GitHub repository.

### Searching and Filtering

- Type in the search bar to filter by name or description
- Use the category dropdown to filter by API category
- Use the auth filter to filter by authentication type (No Auth, API Key, OAuth, Bearer)

### Creating API Tools

Click the toggle on any catalog entry to enable it. Enabled entries can be used as tool definitions that the AI can invoke.

### Credentials

Go to **Credentials** (sidebar) to store API keys for external services. You can test credentials and set one as active per provider.

---

## Voice

Voice features are optional and require running external services.

### TTS (Text-to-Speech)

LUNA supports [Kokoro](https://github.com/remsky/Kokoro-FastAPI) for text-to-speech.

**Requirements:**
- Kokoro TTS server running (default: `http://localhost:8880`)
- Set `KOKORO_TTS_ENDPOINT` environment variable if using a different address

**Configuration (Settings → Voice):**
- TTS Enabled toggle
- Voice name
- Speed: 0.5x–2.0x
- Volume: 0.0–1.0

### STT (Speech-to-Text)

LUNA supports [Whisper](https://github.com/openai/whisper) for speech recognition.

**Requirements:**
- Whisper server running (default: `http://localhost:8080`)
- Set `WHISPER_STT_ENDPOINT` environment variable if using a different address

**Configuration (Settings → Voice):**
- STT Enabled toggle

### Sound Effects

LUNA plays audio feedback via the Web Audio API (no audio files required). Sound effects are tied to the TTS enabled setting and volume setting. Six distinct sounds provide feedback on state transitions.

---

## Desktop Experience

### Thinking Orb

The animated orb on the Home screen reflects LUNA's current state:

| State | Meaning |
|-------|---------|
| IDLE | No active operation |
| THINKING | Processing AI request |
| WORKING | Executing tools |
| SEARCHING | Querying APIs |
| LISTENING | Microphone active |
| SPEAKING | TTS playback |
| ERROR | Operation failed |

### System Stats

The Home screen's left panel shows live system statistics. Currently only implemented for Linux (reads `/proc/stat`, `/proc/meminfo`, and `df`). On other platforms, values show "Unavailable."

---

## Settings

### General

- **Language** — Interface language (English)
- **Default Provider** — Default AI provider (OpenRouter)
- **Auto-save conversations** — Automatically save conversations (default: on)
- **Memory enabled** — Enable persistent memory (default: on)
- **Max context messages** — Number of messages sent as context (default: 20)

### Providers

- Add, remove, and set active OpenRouter API key accounts
- Keys are masked in the UI

### Voice

- TTS/STT enabled toggles
- Voice name, speed, volume configuration

### Updates

- Current version display
- Update status (up to date, update available, offline, error)
- Manual "Check for Updates" button

---

## Updates

### How Updating Works

LUNA checks for updates automatically at startup and every 6 hours. The app fetches `latest.json` from GitHub Releases to discover new versions. When a newer version is available, a banner appears at the top of the app.

### Updating via CLI

```bash
luna update
```

This:
1. Fetches the update manifest from GitHub
2. Compares the remote version with the installed version
3. Downloads the correct artifact for your platform
4. Verifies the SHA-256 checksum
5. Installs the update (backs up the old binary, replaces with new)
6. Your data (conversations, memories, settings) is never affected

### Updating via GUI

Go to **Settings → Updates → Check for Updates**. If an update is available, follow the instructions shown.

### Update Notification

When an update is available, a teal banner appears at the top of the LUNA window showing the available version and instructions to run `luna update`. The banner can be dismissed.

### Automatic Releases

LUNA uses GitHub Actions for automated builds and releases. When a developer pushes a version change to `main`, CI automatically builds for all platforms, creates a GitHub Release, and publishes `latest.json` with SHA-256 checksums.

---

## Offline Mode

### What works offline

- Conversation history and messages
- Memory creation and search
- Settings management
- Tool permissions
- Activity logs
- System stats (on Linux)

### What requires internet

- AI chat (OpenRouter)
- API catalog sync
- Update checking
- Voice services (if hosted remotely)

LUNA does not include any local AI models. AI chat always requires an internet connection to reach OpenRouter.

---

## User Data

### Data Directory Locations

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/luna/` |
| macOS | `~/Library/Application Support/com.luna.desktop/` |
| Windows | `%APPDATA%\com.luna.desktop\data\` |

### What is stored

| File | Contents |
|------|----------|
| `luna.db` | SQLite database — conversations, messages, memories, settings, providers, permissions, audit logs, API catalog |

All user data is stored in a single SQLite database file. There are no other data files, caches, or configuration files outside this directory.

---

## Backup

To back up LUNA data, copy the `luna.db` file from the data directory to a safe location.

For a safe backup while LUNA is running:

```bash
# Linux example
cp ~/.local/share/luna/luna.db ~/luna-backup-$(date +%Y%m%d).db
```

SQLite's WAL mode means the database file is safe to copy while the application is running, though a brief inconsistency is possible if a write occurs during the copy. For a perfectly consistent backup, close LUNA first.

---

## Uninstall

### Remove application only

**Linux AppImage:**
```bash
rm /path/to/LUNA_0.1.0_amd64.AppImage
```

**Linux .deb:**
```bash
sudo dpkg -r luna
```

**Windows:** Use **Settings → Apps → Installed apps → LUNA → Uninstall**, or run the uninstaller from the installation directory.

**macOS:** Drag `LUNA.app` from `/Applications` to the Trash.

### Remove user data

```bash
# Linux
rm -rf ~/.local/share/luna/

# macOS
rm -rf ~/Library/Application Support/com.luna.desktop/

# Windows
# Delete %APPDATA%\com.luna.desktop\ from File Explorer
```

Uninstalling the application does not automatically remove user data.

---

## Security

See [SECURITY.md](SECURITY.md) for full details.

- API keys stored locally in SQLite, masked in the UI
- Permission system controls tool access (allowed / denied / ask)
- Terminal commands require permission and are logged
- All network requests use HTTPS
- Update downloads verified with SHA-256 checksums
- CSP (Content Security Policy) enabled in production
- No telemetry, no third-party data collection
- No credentials committed to the repository

To report a security vulnerability, see [SECURITY.md](SECURITY.md).

---

## Privacy

LUNA stores all data locally in SQLite. No data is sent anywhere except:

- **OpenRouter API requests** — conversation messages are sent to OpenRouter's API for AI processing
- **Update checks** — version information is checked against GitHub Releases
- **API catalog sync** — fetches the public-apis README from GitHub

LUNA does not collect telemetry, analytics, or usage data. No data is sold or shared with third parties.

---

## Troubleshooting

### LUNA does not launch

1. Check system requirements (see [Platform Support](#platform-support))
2. On Linux, ensure WebKitGTK 4.1 and related libraries are installed
3. On macOS, check **System Settings → Privacy & Security** for blocked app notifications
4. On Windows, ensure WebView2 is installed

### CLI command not found

The `luna` CLI is not automatically installed to your PATH. See [CLI](#cli) for installation instructions.

### `luna update` says "No published release is currently available"

This means no GitHub Release exists yet for the current version. The first release will be created when the version in `Cargo.toml` is bumped and pushed to `main`.

### OpenRouter connection fails

1. Verify your API key is correct in **Settings → Providers**
2. Check your internet connection
3. Run `luna doctor` to test connectivity
4. Verify the API key is active (click **Set Active**)

### Invalid API key

OpenRouter returns an error. Verify your key at [openrouter.ai/keys](https://openrouter.ai/keys).

### Voice unavailable

Voice services (Kokoro TTS, Whisper STT) are optional external servers. Ensure they are running and accessible at the configured endpoints. LUNA gracefully disables voice features when services are unavailable.

### Tool permission denied

Go to **Tools** and change the permission for the relevant tool from "denied" to "ask" or "allowed."

### System stats show "Unavailable"

System stats (CPU, memory) are only implemented for Linux. On Windows and macOS, these values are not available.

### Database initialization failure

If LUNA cannot create or open the database, it will fail to start. Check that the data directory exists and is writable. Run `luna doctor` to diagnose.

### Linux desktop icon missing

The AppImage and .deb do not automatically create a desktop launcher. See [Linux Desktop Icon](#linux-desktop-icon) for manual setup.

### Network unavailable

When offline, LUNA still works for conversations history, memory, settings, and local tools. AI chat requires an internet connection.

---

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 9+
- [Rust](https://rustup.rs/) (latest stable)
- Tauri system dependencies (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

#### Linux (Arch Linux)

```bash
sudo pacman -S webkit2gtk-4.1 libsoup3 javascriptcoregtk4.1 libappindicator-gtk3 librsvg
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

#### macOS

```bash
xcode-select --install
```

#### Windows

Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

### Setup

```bash
git clone https://github.com/forvercart0-oss/Luna.git
cd Luna
pnpm install
```

### Development Commands

| Command | Description |
|---------|-------------|
| `pnpm tauri dev` | Start development server with hot reload |
| `pnpm tauri build` | Build production application |
| `pnpm typecheck` | Run TypeScript type checking |
| `pnpm lint` | Run ESLint |
| `pnpm build` | Build frontend only (TypeScript + Vite) |
| `cargo check` | Check Rust code compiles |
| `cargo test` | Run Rust unit tests |

### Production Build

```bash
pnpm tauri build
```

Generated artifacts are in `src-tauri/target/release/bundle/`:
- **Linux**: `deb/` and `appimage/` subdirectories
- **Windows**: `msi/` and `nsis/` subdirectories
- **macOS**: `macos/` subdirectory (`.app` bundle)

---

## Architecture

```
React + TypeScript + Zustand (frontend)
        ↓
Tauri IPC bridge (54 commands)
        ↓
Rust application core
    ├── AI Layer — OpenRouter streaming SSE
    ├── Memory Layer — SQLite
    ├── Voice Layer — Kokoro TTS + Whisper STT (optional)
    ├── Tool Layer — filesystem, terminal, system
    ├── API Layer — catalog + HTTP connector
    ├── Security Layer — permissions + audit
    └── SQLite (WAL mode, 9 migrations)
```

### Project Structure

```
Luna/
├── src/                          # Frontend (React + TypeScript)
│   ├── App.tsx                   # Root component, page routing
│   ├── styles.css                # Global CSS (dark theme)
│   ├── components/
│   │   ├── layout/               # TopBar, Sidebar
│   │   ├── home/                 # HomeScreen (orb, stats, composer)
│   │   ├── chat/                 # ChatPage (conversations, messages)
│   │   ├── settings/             # SettingsPage (general, providers, voice, updates)
│   │   ├── models/               # ModelsPage (model profile management)
│   │   ├── memory/               # MemoryPage (memory CRUD)
│   │   ├── permissions/          # PermissionsPage (tool access)
│   │   ├── activity/             # ActivityPage (event timeline)
│   │   ├── catalog/              # ApiCatalogPage (public APIs)
│   │   ├── credentials/          # ApiCredentialsPage (API keys)
│   │   └── orb/                  # OrbStatus (ThinkingOrb)
│   └── lib/
│       ├── types/index.ts        # TypeScript interfaces
│       ├── stores/index.ts       # Zustand state stores
│       └── hooks/useSoundEffects.ts  # Web Audio API sounds
├── src-tauri/                    # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs               # Tauri app setup, command registration
│   │   ├── cli.rs                # CLI binary (luna)
│   │   ├── db.rs                 # SQLite wrapper (WAL mode)
│   │   ├── settings.rs           # Settings manager
│   │   ├── models.rs             # Model/provider/conversation managers
│   │   ├── memory.rs             # Memory manager
│   │   ├── tools.rs              # Tool registry
│   │   ├── permissions.rs        # Permission system
│   │   ├── audit.rs              # Audit logging
│   │   ├── voice.rs              # TTS/STT providers
│   │   ├── api_catalog.rs        # API catalog (public-apis sync)
│   │   ├── api_connector.rs      # Generic HTTP connector, credentials
│   │   ├── providers/
│   │   │   ├── mod.rs            # Provider types
│   │   │   └── openrouter.rs     # OpenRouter (chat + streaming)
│   │   └── commands/             # 54 Tauri IPC commands
│   ├── migrations/               # SQLite migrations (001–009)
│   ├── icons/                    # Application icons
│   └── tauri.conf.json           # Tauri configuration
├── .github/workflows/ci.yml      # CI/CD pipeline
├── latest.json                   # Update manifest
├── package.json
├── README.md
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

---

## Database

LUNA uses SQLite with WAL (Write-Ahead Logging) mode for concurrent read/write access.

### Tables

| Table | Purpose |
|-------|---------|
| `settings` | Application settings |
| `provider_accounts` | OpenRouter API key accounts |
| `model_profiles` | AI model configurations |
| `conversations` | Chat conversations |
| `messages` | Chat messages |
| `memories` | Persistent memory entries |
| `permissions` | Tool permission levels |
| `audit_log` | Tool execution audit trail |
| `tasks` | Agent task management |
| `task_steps` | Agent task steps |
| `api_catalog_entries` | Public API catalog entries |
| `api_tool_definitions` | API tool definitions from catalog |
| `api_credentials` | Stored API credentials |
| `activity_events` | Activity timeline events |
| `migrations` | Applied migration tracking |

### Migrations

9 migration files are applied in order on startup. The database schema is forward-compatible — updates add tables/columns without removing existing ones.

---

## Release Process

### Developer workflow

```bash
# Make changes
git add .
git commit -m "..."
git push
```

### What happens on push to main

1. CI runs Rust checks (`cargo check`, `cargo test`, `cargo clippy`) and frontend checks (`pnpm typecheck`, `pnpm lint`, `pnpm build`)
2. CI builds for Linux x86_64, Windows x86_64, and macOS aarch64
3. If the version in `src-tauri/Cargo.toml` has changed (compared to the latest GitHub Release tag), CI creates a GitHub Release with:
   - Platform-specific artifacts (AppImage, deb, msi, exe, app.tar.gz)
   - CLI binaries (luna, luna.exe)
   - SHA-256 checksums
   - `latest.json` update manifest
4. If the version has not changed, only checks and builds run — no release is created

### Version bumping

Edit the `version` field in these three files (keep them in sync):

| File | Field |
|------|-------|
| `src-tauri/Cargo.toml` | `version = "X.Y.Z"` |
| `src-tauri/tauri.conf.json` | `"version": "X.Y.Z"` |
| `package.json` | `"version": "X.Y.Z"` |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

### Quick start

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `pnpm typecheck && pnpm lint && cargo check`
5. Commit with a descriptive message
6. Push and open a pull request

---

## FAQ

### What is LUNA?

LUNA is a cross-platform desktop AI assistant that connects to OpenRouter for conversational AI, stores data locally in SQLite, and provides system tools, voice interaction, and an API catalog.

### Which operating systems are supported?

Linux (x86_64), Windows (x86_64), and macOS (Apple Silicon). macOS Intel is not currently built by CI.

### Does LUNA require an OpenRouter API key?

Yes. LUNA uses OpenRouter to access AI models. You need an account at [openrouter.ai](https://openrouter.ai/) and an API key.

### Can LUNA work offline?

Partially. Conversation history, memory, settings, and local tools work offline. AI chat requires an internet connection to reach OpenRouter.

### Where are my conversations stored?

In a SQLite database at:
- Linux: `~/.local/share/luna/luna.db`
- macOS: `~/Library/Application Support/com.luna.desktop/luna.db`
- Windows: `%APPDATA%\com.luna.desktop\data\luna.db`

### How do I update LUNA?

Run `luna update` from the command line, or go to **Settings → Updates → Check for Updates** in the GUI.

### How do I create a desktop icon on Linux?

See [Linux Desktop Icon](#linux-desktop-icon). The AppImage and .deb do not automatically create a launcher.

### How do I uninstall LUNA?

Remove the application binary. User data is stored separately and must be removed manually if desired. See [Uninstall](#uninstall).

### Is LUNA free?

Yes. LUNA is open source under the MIT License. OpenRouter may charge for API usage depending on the models you use.

### Does LUNA collect my data?

No. All data is stored locally. The only network requests are to OpenRouter (for AI chat), GitHub (for updates and catalog sync), and optionally to local voice servers.

---

## License

[MIT License](LICENSE) — Copyright (c) 2026 LUNA
