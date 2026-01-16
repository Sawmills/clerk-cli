# Agent Instructions

## Commit Messages

This repo uses [Conventional Commits](https://www.conventionalcommits.org/) with release-please for automated releases.

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
| `ci` | - | CI/CD changes |
| `test` | - | Test changes |

### Writing Good Commit Messages

The commit body becomes the release note description. Write for users, not developers.

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

1. Subject line: imperative mood, no period, max 72 chars
2. Body: explain what and why, not how
3. Include usage examples for new features
4. Breaking changes: add `BREAKING CHANGE:` footer or `!` after type

### Examples

```bash
# Feature with examples
git commit -m "feat(users): add bulk delete command

Delete multiple users at once:
- \`clerk users delete <id1> <id2> ...\`
- \`clerk users delete --all --inactive-days 90\`"

# Bug fix
git commit -m "fix(completions): resolve zsh completion ordering

Subcommands now appear before dynamic suggestions."

# Breaking change
git commit -m "feat(api)!: change auth token format

BREAKING CHANGE: API tokens now use JWT format.
Migrate by regenerating tokens in the dashboard."
```
