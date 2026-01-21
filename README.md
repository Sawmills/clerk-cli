# clerk-cli

Unofficial CLI for [Clerk](https://clerk.com) admin tasks. Manage users, organizations, and generate JWTs from your terminal.

## Install

### Homebrew (macOS)

```bash
brew install sawmills/tap/clerk
```

### Cargo (any platform)

```bash
cargo install --git https://github.com/Sawmills/clerk-cli
```

## Setup

Set your Clerk secret key:

```bash
export CLERK_API_KEY="sk_test_..."
```

Add to your shell profile (`~/.zshrc`, `~/.bashrc`, etc.) to persist.

## Usage

### Users

```bash
clerk users                              # List users (default: 10)
clerk users list -l 50                   # List 50 users
clerk users list -q "john@example.com"   # Search by email/name
clerk users create -e user@example.com   # Create user
clerk users <user_id>                    # Show user details
clerk users <user_id> impersonate        # Generate sign-in link
clerk users <user_id> jwt                # Generate JWT (interactive template)
clerk users <user_id> jwt -t my-template # Generate JWT with template
clerk users <user_id> add-to-org -o myorg
clerk users <user_id> remove-from-org -o myorg
```

### Organizations

```bash
clerk orgs                               # List all organizations
clerk orgs list -f "acme"                # Fuzzy search by name
clerk orgs create -n "Acme Inc" -s acme  # Create org
clerk orgs pick                          # Interactive picker (prints ID)
clerk orgs <org>                         # Show org details
clerk orgs <org> members                 # List members
clerk orgs <org> members add -u <user_id> -r org:admin
clerk orgs <org> members <user_id> impersonate
clerk orgs <org> members <user_id> jwt
clerk orgs <org> delete                  # Delete (with confirmation)
clerk orgs <org> delete --force          # Delete (skip confirmation)
```

### SSO Connections

```bash
clerk sso                                # List all SSO connections
clerk sso list                           # Same as above
clerk orgs <org> sso list                # List SSO for specific org
clerk orgs <org> sso add \               # Add SAML connection
  --name "Okta" \
  --provider saml_okta \
  --domain "example.com"
clerk orgs <org> sso update "Okta" \     # Update by name or ID
  --metadata-url "https://..." \
  --active true
clerk orgs <org> sso delete "Okta"       # Delete (with confirmation)
clerk orgs <org> sso delete "Okta" -f    # Delete (skip confirmation)
```

Providers: `saml_okta`, `saml_google`, `saml_microsoft`, `saml_custom`

### JWT Generation

```bash
clerk jwt                                # Interactive user + template picker
clerk jwt <user_id>                      # Interactive template picker
clerk jwt <user_id> -t my-template       # Direct generation
clerk jwt --list                         # List available templates
```

### Impersonation

```bash
clerk impersonate                        # Interactive user picker
clerk impersonate <user_id>              # Generate sign-in link
```

Requires `CLERK_FRONTEND_API` for custom domains:

```bash
export CLERK_FRONTEND_API="https://clerk.yourdomain.com"
```

## Shell Completions

### Zsh

```bash
clerk completions zsh > ~/.cache/zsh/completions/_clerk
# Or wherever your fpath points
```

Add to `~/.zshrc`:

```bash
fpath=(~/.cache/zsh/completions $fpath)
autoload -Uz compinit && compinit
```

### Bash

```bash
clerk completions bash > /etc/bash_completion.d/clerk
# Or: ~/.local/share/bash-completion/completions/clerk
```

### Fish

```bash
clerk completions fish > ~/.config/fish/completions/clerk.fish
```

## Examples

### Quick user lookup

```bash
clerk users list -q "john" | head -20
```

### Impersonate a user from an org

```bash
clerk orgs acme-corp members              # Find user ID
clerk users user_xxx impersonate          # Get sign-in link
```

### Script-friendly output

```bash
# Get org IDs only (for scripting)
clerk orgs list --ids-only

# Pick org interactively, use ID
ORG_ID=$(clerk orgs pick)
clerk orgs $ORG_ID members
```

### Generate JWT for API testing

```bash
TOKEN=$(clerk jwt user_xxx -t my-api-template)
curl -H "Authorization: Bearer $TOKEN" https://api.example.com/endpoint
```

### Configure SSO for an organization

```bash
# Step 1: Create connection, get ACS URL and SP Entity ID
clerk orgs acme sso add --name "Okta" --provider saml_okta --domain "acme.com"
# Output: ACS URL and SP Entity ID to configure in your IdP

# Step 2: Configure your IdP with the ACS URL and SP Entity ID

# Step 3: Update connection with IdP metadata
clerk orgs acme sso update "Okta" \
  --metadata-url "https://acme.okta.com/app/.../sso/saml/metadata" \
  --active true
```

## Environment Variables

| Variable             | Required | Description                                          |
| -------------------- | -------- | ---------------------------------------------------- |
| `CLERK_API_KEY`      | Yes      | Clerk secret key (`sk_test_...` or `sk_live_...`)    |
| `CLERK_FRONTEND_API` | No       | Frontend API URL for impersonation on custom domains |

## License

MIT
