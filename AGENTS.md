# Agent Instructions

Unofficial Clerk CLI for admin tasks. Rust + Tokio + Clap.

## Build & Run

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo run -- users list        # Run with args
```

## Testing

```bash
cargo test                     # All tests
cargo test cli_help            # Single test by name
cargo test cli_                # Tests matching pattern
cargo test -- --nocapture      # Show println! output
```

Test files are in `tests/`. Use `assert_cmd` for CLI testing.

## Linting

This repo uses [Trunk](https://trunk.io) for linting:

```bash
trunk check                    # Run all linters
trunk check --fix              # Auto-fix issues
trunk fmt                      # Format all files
```

Individual tools:
```bash
cargo fmt                      # Format Rust code
cargo clippy                   # Rust lints
```

## Project Structure

```
src/
├── main.rs           # CLI entry, clap Commands enum
├── client.rs         # ClerkClient API wrapper
├── models.rs         # Request/response structs
└── commands/
    ├── mod.rs
    ├── users.rs      # clerk users subcommands
    ├── orgs.rs       # clerk orgs subcommands
    ├── impersonate.rs
    ├── jwt.rs
    └── completions.rs
tests/
└── cli_test.rs       # CLI integration tests
```

## Code Style

### Imports

Group and order imports:
```rust
use crate::client::ClerkClient;           // 1. Local crate
use crate::models::{User, Organization};
use anyhow::Result;                        // 2. External crates
use clap::{Parser, Subcommand};
use std::io;                               // 3. Standard library
```

### Error Handling

- Use `anyhow::Result` for command handlers
- Use `thiserror` for typed errors in `client.rs`
- Propagate errors with `?`, don't unwrap in library code
- User-facing errors via `anyhow::bail!("message")`

```rust
// Good
pub async fn list() -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let users = client.list_users(10, None).await?;
    Ok(())
}

// Bad
pub async fn list() {
    let client = ClerkClient::new().unwrap();  // Don't unwrap
}
```

### Naming

- Commands: `snake_case` functions (`list`, `create`, `add_to_org`)
- Structs: `PascalCase` (`ClerkClient`, `CreateUserRequest`)
- CLI args: `kebab-case` (`--first-name`, `--ids-only`)

### Async

All API calls are async. Command handlers are `pub async fn`.

### Output

- Tables: Use `comfy_table` with `UTF8_FULL` preset
- Success messages: `println!("Created user: {}", id)`
- Errors: Return `Err`, don't print directly

### Models

- Response structs: `#[derive(Debug, Deserialize)]`
- Request structs: `#[derive(Debug, Serialize)]`
- Optional fields: `#[serde(skip_serializing_if = "Option::is_none")]`
- Use `#[allow(dead_code)]` for unused response fields

---

## Commit Messages

This repo uses [Conventional Commits](https://www.conventionalcommits.org/) with release-please.

### Format

```
<type>(<scope>): <subject>

<body>
```

### Types

| Type | Release | Description |
|------|---------|-------------|
| `feat` | minor | New feature |
| `fix` | patch | Bug fix |
| `perf` | patch | Performance improvement |
| `refactor` | - | Code refactoring (no release) |
| `docs` | - | Documentation only |
| `chore` | - | Maintenance tasks |

### Writing Good Commit Messages

The body becomes the release note. Write for users, not developers.

**Bad:**
```
feat(orgs): add delete command with confirmation prompt
```

**Good:**
```
feat(orgs): add delete command

Delete organizations directly from CLI:
- `clerk orgs <org> delete` - with confirmation prompt
- `clerk orgs <org> delete --force` - skip confirmation
```

### Rules

1. Subject: imperative mood, no period, max 72 chars
2. Body: explain what and why, include usage examples
3. Breaking changes: add `BREAKING CHANGE:` footer or `!` after type

---

## Environment

Required:
```bash
export CLERK_API_KEY="sk_test_..."
```

Optional:
```bash
export CLERK_FRONTEND_API="https://clerk.example.com"  # For impersonation
```

## Adding a New Command

1. Add variant to `Commands` or subcommand enum in `main.rs`
2. Add handler in `src/commands/<module>.rs`
3. Add API method in `client.rs` if needed
4. Add request/response structs in `models.rs`
5. Update `src/completions/clerk.zsh` for shell completions
6. Add CLI tests in `tests/cli_test.rs`
