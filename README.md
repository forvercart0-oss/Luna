# LUNA

A modern, cross-platform desktop AI assistant built with Tauri 2, React, TypeScript, Rust, and SQLite.

LUNA provides conversational AI through OpenRouter, persistent memory, voice interaction, system tools, API integrations, and a futuristic desktop experience.

## Features

### AI Chat
- **OpenRouter integration** — Access hundreds of AI models through a single API
- **Multiple provider accounts** — Manage several OpenRouter API keys
- **Model profiles** — Configure system prompts, temperature, max tokens per model
- **Streaming responses** — Real-time token-by-token output with stop generation
- **Persistent conversations** — All chat history stored in local SQLite database

### Memory
- **Persistent memory system** — Store and retrieve important information across sessions
- **Search** — Find memories by keyword
- **Importance levels** — Prioritize critical information
- **Source tracking** — Know where each memory came from

### Tool System
- **Filesystem tools** — Read, write, and list files
- **Terminal execution** — Run shell commands
- **System information** — Get CPU, memory, storage, hostname, OS details
- **Permission system** — Control tool access (allowed/denied/ask per tool)
- **Audit logging** — Track all tool executions

### API Catalog
- **Public APIs integration** — Sync from the public-apis directory (1,400+ APIs)
- **Search and filter** — Find APIs by name, category, or auth type
- **API tool definitions** — Create reusable tool definitions from catalog entries
- **Health tracking** — Monitor API availability

### API Credentials
- **Multi-provider credential storage** — Store API keys for different services
- **Credential testing** — Verify credentials work before using them
- **Active credential selection** — Choose which credential to use per provider

### Voice
- **Kokoro TTS** — Text-to-speech via Kokoro provider
- **Whisper STT** — Speech-to-text via Whisper provider
- **Voice settings** — Configure voice, speed, volume
- **Availability checks** — Detect when voice services are running

### Desktop Experience
- **Thinking Orb** — Animated status indicator reflecting LUNA's current state
- **Sound effects** — Audio feedback on state transitions (Web Audio API)
- **Activity timeline** — Real-time event feed
- **System stats** — Live CPU, memory, storage monitoring
- **Dark theme** — JARVIS-inspired UI with teal accents

### Settings
- **General** — Theme, language preferences
- **AI Providers** — Manage OpenRouter accounts and API keys
- **Voice** — TTS/STT configuration
- **Permissions** — Tool access control

## Installation

### End Users

Download the pre-built application for your platform:

| Platform | File | Notes |
|----------|------|-------|
| **Windows x64** | `.msi` installer | Run the installer, launch from Start Menu |
| **Linux x64** | `.AppImage` | `chmod +x` then run, or install `.deb` on Debian/Ubuntu |
| **macOS ARM64** | `.app.tar.gz` | Extract, drag LUNA.app to Applications |
| **macOS x64** | `.app.tar.gz` | Extract, drag LUNA.app to Applications |

**No Rust, Node.js, or development tools required.** Just download and run.

Download the latest release from [GitHub Releases](https://github.com/forvercart0-oss/Luna/releases).

### First Launch

1. Launch LUNA
2. Go to **Settings → AI Providers**
3. Add your OpenRouter API key
4. Go to **Settings → Models** (or the Models page)
5. Create a model profile with your preferred model
6. Set the model as active
7. Start chatting

### CLI Installation

The `luna-cli` binary provides command-line utilities:

```bash
# Check version
luna-cli version

# Run diagnostics
luna-cli doctor

# Check for updates
luna-cli update
```

## Development

### Requirements

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

### Commands

| Command | Description |
|---------|-------------|
| `pnpm tauri dev` | Start development server with hot reload |
| `pnpm tauri build` | Build production application |
| `pnpm typecheck` | Run TypeScript type checking |
| `pnpm lint` | Run ESLint |
| `pnpm build` | Build frontend only (TypeScript + Vite) |

### Project Structure

```
LUNA/
├── src/                          # Frontend (React + TypeScript)
│   ├── App.tsx                   # Root component, page routing
│   ├── main.tsx                  # React entry point
│   ├── styles.css                # Global CSS (dark theme)
│   ├── components/
│   │   ├── layout/               # TopBar, Sidebar
│   │   ├── home/                 # HomeScreen (orb, stats, composer)
│   │   ├── chat/                 # ChatPage (conversations, messages)
│   │   ├── settings/             # SettingsPage (general, providers, voice)
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
│   │   ├── cli.rs                # CLI binary (luna-cli)
│   │   ├── db.rs                 # SQLite wrapper (WAL mode)
│   │   ├── settings.rs           # Settings manager
│   │   ├── models.rs             # Model/provider/conversation managers
│   │   ├── memory.rs             # Memory manager
│   │   ├── tools.rs              # Tool registry (filesystem, terminal, system)
│   │   ├── permissions.rs        # Permission system
│   │   ├── audit.rs              # Audit logging
│   │   ├── agent.rs              # Agent engine (task/step management)
│   │   ├── voice.rs              # TTS/STT providers
│   │   ├── api_catalog.rs        # API catalog (public-apis sync)
│   │   ├── api_connector.rs      # Generic HTTP connector, credentials
│   │   ├── providers/
│   │   │   ├── mod.rs            # Provider traits and types
│   │   │   └── openrouter.rs     # OpenRouter (chat + streaming)
│   │   └── commands/
│   │       ├── mod.rs            # Command module declarations
│   │       ├── chat.rs           # Chat commands (streaming)
│   │       ├── settings.rs       # Settings commands
│   │       ├── providers.rs      # Provider account commands
│   │       ├── models.rs         # Model profile commands
│   │       ├── memory.rs         # Memory commands
│   │       ├── permissions.rs    # Permission commands
│   │       ├── audit.rs          # Audit log commands
│   │       ├── tools.rs          # Tool list commands
│   │       ├── catalog.rs        # API catalog commands
│   │       ├── credentials.rs    # Credential commands
│   │       ├── activity.rs       # Activity event commands
│   │       ├── system.rs         # System stats commands
│   │       └── voice.rs          # Voice commands
│   ├── migrations/               # SQLite migrations (001-009)
│   ├── icons/                    # Application icons
│   └── tauri.conf.json           # Tauri configuration
├── .github/workflows/ci.yml      # CI/CD pipeline
├── latest.json                   # Update manifest
├── package.json                  # Frontend dependencies
├── Cargo.toml                    # Rust dependencies
├── README.md
├── LICENSE                       # MIT
├── SECURITY.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

### Architecture

```
Desktop Shell (Tauri)
    ↓
Frontend (React + TypeScript + Zustand)
    ↓
IPC Bridge (Tauri Commands)
    ↓
Application Core (Rust)
    ├── AI Layer (OpenRouter + Streaming)
    ├── Memory Layer (SQLite)
    ├── Voice Layer (Kokoro TTS + Whisper STT)
    ├── Tool Layer (Filesystem, Terminal, System)
    ├── API Layer (Catalog + HTTP Connector)
    ├── Security Layer (Permissions + Audit)
    └── SQLite (WAL mode, migrations)
```

## OpenRouter Configuration

LUNA uses [OpenRouter](https://openrouter.ai/) to access AI models.

### Setup

1. Create an account at [openrouter.ai](https://openrouter.ai/)
2. Generate an API key
3. In LUNA, go to **Settings → AI Providers**
4. Click **Add** and enter your API key
5. Go to **Models** and create a profile for your preferred model
6. Set the model as active

### Multiple Accounts

You can add multiple OpenRouter accounts. Each account can have different API keys. The active account is used for all requests.

### API Key Security

- API keys are stored locally in SQLite
- Keys are masked in the UI (only first 4 and last 4 characters shown)
- Keys are never sent to any server other than OpenRouter
- Keys are never logged
- Never commit API keys to version control

### Model Profiles

Each model profile includes:
- **Name** — Display name
- **Model ID** — OpenRouter model identifier (e.g., `anthropic/claude-3.5-sonnet`)
- **System prompt** — Custom instructions for the model
- **Temperature** — Creativity (0.0–2.0)
- **Max tokens** — Maximum response length
- **Enabled** — Whether the profile is available for selection

## Memory System

LUNA maintains a persistent memory store in SQLite.

### Memory Types

- **fact** — Factual information
- **preference** — User preferences
- **context** — Contextual information
- **instruction** — User instructions

### Operations

- **Create** — Store new memories with type, content, importance, and source
- **Search** — Find memories by keyword with relevance scoring
- **Edit** — Update memory content or metadata
- **Delete** — Remove memories

### Importance Levels

- **1** — Low priority
- **2** — Medium priority
- **3** — High priority
- **4** — Critical priority

## Tool System

LUNA includes built-in tools that can be executed through the AI.

### Available Tools

| Tool | Description |
|------|-------------|
| `filesystem.read` | Read file contents |
| `filesystem.write` | Write content to files |
| `filesystem.list` | List directory contents |
| `terminal.execute` | Execute shell commands |
| `system.info` | Get system information |

### Permission System

Each tool has a permission level:
- **allowed** — Execute without asking
- **denied** — Never execute
- **ask** — Prompt user before executing

Configure permissions in the **Permissions** page.

### Audit Logging

All tool executions are logged with:
- Timestamp
- Tool ID
- Arguments summary
- Result summary
- Success/failure status
- Permission state

View logs in the **Activity** page.

## API Catalog

LUNA integrates with the [public-apis](https://github.com/public-apis/public-apis) directory.

### Syncing

Click **Sync from public-apis** in the API Catalog page to fetch the latest catalog. This imports ~1,400 API entries with:
- Name, description, category
- Authentication type (none, API key, OAuth, etc.)
- HTTPS support
- CORS policy
- Homepage URL

### Searching

Filter APIs by:
- Text search (name, description)
- Category
- Authentication type

### API Tools

Create tool definitions from catalog entries. These define:
- Endpoint URL
- HTTP method
- Required parameters
- Authentication requirements

## Voice System

### TTS (Text-to-Speech)

LUNA supports Kokoro TTS for voice output.

**Requirements:**
- Kokoro TTS server running (default: `http://localhost:8880`)
- Set `KOKORO_TTS_ENDPOINT` environment variable if using a different address

**Configuration:**
- Voice selection
- Speed (0.5x–2.0x)
- Volume

### STT (Speech-to-Text)

LUNA supports Whisper for speech recognition.

**Requirements:**
- Whisper server running (default: `http://localhost:8080`)
- Set `WHISPER_STT_ENDPOINT` environment variable if using a different address

### Availability

LUNA checks voice service availability before attempting to use them. If services are not running, voice features are gracefully disabled with appropriate error messages.

## Thinking Orb

The Thinking Orb is an animated status indicator that reflects LUNA's current state:

| State | Orb Animation | Trigger |
|-------|--------------|---------|
| **IDLE** | Breathing | No active operation |
| **THINKING** | Solving | Processing AI request |
| **WORKING** | Working | Executing tools |
| **SEARCHING** | Searching | Querying APIs |
| **LISTENING** | Listening | Microphone active |
| **SPEAKING** | Composing | TTS playback |
| **ERROR** | Error state | Operation failed |

## Sound Effects

LUNA provides audio feedback using the Web Audio API:

| Event | Sound |
|-------|-------|
| Listening starts | Dual-tone (440Hz + 520Hz) |
| Thinking | Low tone (330Hz) |
| Tool started | Mid tone (600Hz) |
| Tool completed | Rising tone (800Hz + 1000Hz) |
| Message received | Pleasant tone (660Hz) |
| Error | Low buzz (200Hz sawtooth) |

Sound effects can be toggled in Settings.

## Activity System

Track all LUNA operations in the Activity timeline:
- Chat messages sent/received
- Tool executions
- API calls
- Memory operations
- Errors and warnings

Each event includes:
- Timestamp
- Event type
- Category
- Title and detail
- Status (running, completed, error)

## Settings

### General
- **Theme** — Dark (default), Light, System
- **Language** — Interface language

### AI Providers
- Add/remove OpenRouter accounts
- Set active provider
- View masked API keys

### Voice
- TTS enabled/disabled
- STT enabled/disabled
- Voice selection
- Speed and volume

### Permissions
- Per-tool permission levels
- Scope configuration

## Security

See [SECURITY.md](SECURITY.md) for details.

- API keys stored locally with masking
- Permission system for tool access
- Audit logging of all operations
- CSP enabled in production
- No credentials committed to repository

## Updating

### CLI Update Check

```bash
luna-cli update
```

This checks GitHub Releases for the latest version and displays download instructions.

### Manual Update

1. Download the latest release from [GitHub Releases](https://github.com/forvercart0-oss/Luna/releases)
2. Install the new version (your data is preserved)

### User Data

Updates never delete:
- Conversations and messages
- Memories
- Settings
- Provider accounts and API keys
- Model profiles
- Permissions
- Audit logs

## CLI Reference

### `luna-cli version`

Display version and installation information.

### `luna-cli doctor`

Run diagnostic checks:
- Installation directory
- Data directory and database
- Rust toolchain
- Node.js and pnpm
- Tauri CLI
- Network connectivity
- Voice services

### `luna-cli update`

Check for updates from GitHub Releases.

## License

[MIT License](LICENSE) — Copyright (c) 2026 LUNA
