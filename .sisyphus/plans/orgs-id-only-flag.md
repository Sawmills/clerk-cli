# Add --id-only flag to `clerk orgs <org>`

## Context

### Original Request
User wants to extract org ID by name for use in shell commands:
```bash
remote-operator ... --org-id $(clerk orgs "acme-corp" --id-only)
```

### Current Behavior
- `clerk orgs <slug_or_id>` shows a table with ID, Name, Slug
- No way to get just the ID for scripting

---

## Work Objectives

### Core Objective
Add `--id-only` flag to output just the org ID when looking up by name/slug.

### Concrete Deliverables
- `clerk orgs <org> --id-only` outputs only the org ID

### Definition of Done
- [x] `clerk orgs acme-corp --id-only` outputs `org_xxx` (no table, no newlines except trailing)
- [x] `clerk orgs org_xxx --id-only` also works (echoes back the ID)
- [x] Without flag, behavior unchanged (shows table)

### Must NOT Have
- No changes to `clerk orgs list --ids-only` (already exists, different command)
- No extra output (no "Found org:" prefix)

---

## TODOs

- [x] 1. Add `--id-only` flag to Orgs command in main.rs

  **What to do**:
  - Add `id_only: bool` field to `Commands::Orgs` with `#[arg(long)]`
  - Pass it through to the show case in the match arm

  **References**:
  - `src/main.rs:29-35` - Current Orgs command definition
  - `src/main.rs:502-504` - Current show match arm

  **Acceptance Criteria**:
  - [x] `clerk orgs --help` shows `--id-only` flag
  - [x] Flag parses without error

  **Commit**: NO (groups with 2)

- [x] 2. Update show() to handle --id-only flag

  **What to do**:
  - Change `show(slug_or_id: &str)` signature to `show(slug_or_id: &str, id_only: bool)`
  - When `id_only` is true, print just `org.id` instead of the table
  - Update call site in main.rs

  **References**:
  - `src/commands/orgs.rs:218-237` - Current show function

  **Acceptance Criteria**:
  - [x] `clerk orgs acme-corp --id-only` outputs only the org ID
  - [x] `clerk orgs acme-corp` still shows table (no regression)

  **Commit**: YES
  - Message: `feat(orgs): add --id-only flag for scripting`
  - Files: `src/main.rs`, `src/commands/orgs.rs`
  - Pre-commit: `cargo test`

---

## Success Criteria

### Verification Commands
```bash
clerk orgs <some-org> --id-only  # Expected: org_xxx (just the ID)
clerk orgs <some-org>            # Expected: table output (unchanged)
```
