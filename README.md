# LUNA — Desktop AI Assistant

A real, production-oriented desktop AI assistant built with Tauri, React, TypeScript, and Rust.

LUNA is NOT a demo, UI mockup, or simple API wrapper. It is a fully functional desktop application with real AI integration, persistent storage, and extensible architecture.

## Supported Platforms

- Linux
- Windows
- macOS

## Architecture

```
LUNA
├── Desktop Shell (Tauri)
├── Frontend (React + TypeScript)
├── Application Core
│   ├── Conversation Manager
│   ├── Model Manager
│   ├── Memory Manager
│   ├── Agent Manager
│   ├── Tool Manager
│   ├── Permission Manager
│   └── Task Manager
├── AI Layer (OpenRouter)
├── Memory Layer (SQLite)
├── Voice Layer (Kokoro TTS / Whisper STT)
├── Tool Layer (Filesystem, Terminal, System)
└── Security Layer (Permissions, Audit Logs)
```

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- System dependencies for Tauri (see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/))

### Linux (Arch)

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file xdg-utils
```

### Build

```bash
# Clone the repository
git clone https://github.com/your-username/luna.git
cd luna

# Install frontend dependencies
pnpm install

# Build the application
pnpm tauri build
```

### Development

```bash
# Start development mode
pnpm tauri dev
```

## Configuration

### OpenRouter Setup

1. Get an API key from [OpenRouter](https://openrouter.ai/)
2. Open LUNA → Settings → AI Providers
3. Click "+ Add Key"
4. Enter your API key and a profile name
5. Set the key as active

### Model Profiles

1. Go to Settings → Models
2. Create model profiles with OpenRouter model IDs
3. Example: `openai/gpt-4o`, `anthropic/claude-3.5-sonnet`
4. Set one as active for chat

### Kokoro TTS (Optional)

Configure Kokoro TTS endpoint in Settings → Voice for voice output.

## Features

- **Real AI Chat** — Communicate with real AI models via OpenRouter
- **Multiple API Keys** — Manage multiple OpenRouter accounts
- **Model Profiles** — Create named profiles with custom parameters
- **Persistent Conversations** — All chats saved in SQLite
- **Memory System** — Store and retrieve important information
- **Tool System** — Filesystem, terminal, and system tools
- **Permission Control** — Granular permission management
- **Audit Logging** — Track all actions and tool usage
- **Voice Support** — TTS/STT abstractions (Kokoro, Whisper)
- **Agent Engine** — Foundation for agentic task execution

## Security Model

- API keys stored in local SQLite (not in logs or Git)
- Tool permissions require explicit user approval
- Dangerous operations require confirmation
- Full audit trail for all actions
- No secrets committed to repository

## Tech Stack

- **Desktop**: Tauri 2
- **Frontend**: React 19, TypeScript, Zustand
- **Backend**: Rust, rusqlite, reqwest
- **Database**: SQLite (WAL mode)
- **AI Provider**: OpenRouter
- **Build**: Vite, Cargo

## Development

### Commands

```bash
pnpm tauri dev      # Development mode with hot reload
pnpm tauri build    # Production build
pnpm typecheck      # TypeScript type checking
pnpm lint           # ESLint
pnpm build          # Frontend only build
```

### Project Structure

```
luna/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── lib/                # Stores, types, hooks
│   └── styles.css          # Global styles
├── src-tauri/              # Rust backend
│   ├── src/                # Rust source
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── providers/      # AI provider implementations
│   │   └── ...
│   ├── migrations/         # SQLite migrations
│   └── tauri.conf.json     # Tauri configuration
├── package.json
└── README.md
```

## License

MIT
