# Contributing to LUNA

Thank you for your interest in contributing to LUNA!

## Development Setup

1. Install Rust and Node.js
2. Clone the repository
3. Run `pnpm install`
4. Run `pnpm tauri dev`

## Code Style

- Rust: Follow standard rustfmt conventions
- TypeScript: Strict mode, no `any` types
- Components: Functional components with hooks
- State: Zustand stores, no prop drilling

## Testing

Before submitting a PR, ensure:

```bash
pnpm typecheck   # No TypeScript errors
pnpm lint        # No lint errors
cargo check      # No Rust compilation errors
pnpm tauri build # Successful build
```

## Architecture

- Keep modules small and focused
- Use dependency injection via Tauri state
- Never hard-code credentials
- All user-facing data must come from real sources
- No mock data, no placeholder AI

## Security

- Never commit API keys or secrets
- All tool operations require permission checks
- Log security-relevant actions
- Treat model-generated tool calls as untrusted input
