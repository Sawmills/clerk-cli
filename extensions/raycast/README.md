# Clerk Admin for Raycast

Manage your Clerk users and organizations directly from Raycast. Quick access to common admin operations without leaving your keyboard.

## Features

### 🔍 Search Organizations
- Browse all organizations in your Clerk instance
- Client-side fuzzy search by name, slug, or ID
- Quick actions: Copy ID, View Members, View Details

### 👥 Search Users
- Search users by name or email
- Debounced server-side search for fast results
- Quick actions: Copy ID, Impersonate, Generate JWT

### 🎭 Impersonate User
- Generate sign-in tokens for user impersonation
- Opens impersonation link in your default browser
- Works standalone or from other commands

### 🔐 Generate JWT
- List available JWT templates
- Generate tokens for any user
- Automatically copies token to clipboard

### 🏢 Organization Members
- View members of any organization
- See member roles at a glance
- Quick actions: Copy User ID, Impersonate, Generate JWT

## Setup

### Prerequisites
- Raycast app installed (macOS only)
- Clerk account with API access

### Installation

#### Option 1: Homebrew (Recommended)

```bash
brew install --cask sawmills/tap/clerk-raycast
```

This automatically installs the extension to Raycast. Just open Raycast and search for "Clerk Admin" to get started.

#### Option 2: Manual Installation

1. Clone or download this extension
2. Navigate to the extension directory:
   ```bash
   cd extensions/raycast
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Build the extension:
   ```bash
   npm run build
   ```
5. Install to Raycast:
   ```bash
   ./scripts/install.sh
   ```

#### Option 3: Development Mode

For development and testing:

```bash
cd extensions/raycast
npm install
npm run dev
```

This opens the extension in Raycast's development mode with hot reload.

### Configuration

After installation, configure your Clerk API key in Raycast preferences:

1. Open Raycast
2. Search for any Clerk Admin command
3. Press `⌘ ,` to open preferences
4. Enter your Clerk API key (required)
5. Optionally set your Clerk Frontend API URL for custom domains

**Required:**
- **Clerk API Key**: Your Clerk secret key (`sk_test_...` or `sk_live_...`)

**Optional:**
- **Clerk Frontend API**: Frontend API URL for impersonation on custom domains (e.g., `https://clerk.yourdomain.com`)

## Usage

### Search Organizations
1. Open Raycast
2. Type "Search Organizations"
3. Browse or search for organizations
4. Press `⌘ K` to see available actions

### Search Users
1. Open Raycast
2. Type "Search Users"
3. Start typing to search by name or email
4. Press `⌘ K` to see available actions

### Impersonate User
1. Open Raycast
2. Type "Impersonate User"
3. Search for the user
4. Press `↵` to generate and open sign-in link

### Generate JWT
1. Open Raycast
2. Type "Generate JWT"
3. Search for the user
4. Select a JWT template
5. Token is automatically copied to clipboard

### Organization Members
1. Open Raycast
2. Type "Organization Members"
3. Select an organization
4. View members and perform actions

## Development

### Build
```bash
npm run build
```

### Development Mode
```bash
npm run dev
```

### Lint
```bash
npm run lint
```

### Fix Linting Issues
```bash
npm run fix-lint
```

## Commands

| Command | Description | Shortcut |
|---------|-------------|----------|
| Search Organizations | Browse and search organizations | - |
| Search Users | Search users by name or email | - |
| Impersonate User | Generate sign-in link for user | - |
| Generate JWT | Create JWT token for user | - |
| Organization Members | View and manage org members | - |

## Troubleshooting

### "Clerk API key is required"
Make sure you've configured your API key in Raycast preferences (`⌘ ,`).

### "Failed to load organizations/users"
- Verify your API key is correct
- Check your internet connection
- Ensure your Clerk account has the necessary permissions

### "No active sessions found for user"
JWT generation requires the user to have an active session. The user must be logged in to generate a JWT.

## Security

- API keys are stored securely in Raycast preferences
- Never commit your API keys to version control
- Use test keys for development, live keys for production

## License

MIT

## Related

- [Clerk CLI](https://github.com/Sawmills/clerk-cli) - Command-line interface for Clerk admin tasks
- [Clerk Documentation](https://clerk.com/docs) - Official Clerk documentation
- [Raycast Extensions](https://www.raycast.com/store) - Browse other Raycast extensions
