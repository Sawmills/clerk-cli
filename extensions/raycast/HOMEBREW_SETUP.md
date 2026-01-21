# Homebrew Installation Setup

This guide explains how to make the Clerk Raycast extension installable via Homebrew.

## Quick Install (Once Published)

```bash
brew install --cask sawmills/tap/clerk-raycast
```

## Publishing Process

### 1. Build and Package

```bash
cd extensions/raycast
./scripts/package.sh 0.1.0
```

This creates `clerk-raycast-0.1.0.tar.gz` with the built extension.

### 2. Create GitHub Release

```bash
# Tag the release
git tag raycast-v0.1.0
git push origin raycast-v0.1.0

# Create release with the tarball
gh release create raycast-v0.1.0 \
  clerk-raycast-0.1.0.tar.gz \
  --title "Raycast Extension v0.1.0" \
  --notes "Initial release of Clerk Admin for Raycast"
```

### 3. Update Cask Formula

Get the SHA256 of the tarball:

```bash
shasum -a 256 clerk-raycast-0.1.0.tar.gz
```

Update `Casks/clerk-raycast.rb`:
- Replace `REPLACE_WITH_ACTUAL_SHA256` with the actual SHA256
- Verify the version number matches

### 4. Add to Homebrew Tap

```bash
# Clone your tap repository
git clone git@github.com:Sawmills/homebrew-tap.git
cd homebrew-tap

# Copy the cask formula
cp /path/to/clerk-cli/extensions/raycast/Casks/clerk-raycast.rb Casks/

# Commit and push
git add Casks/clerk-raycast.rb
git commit -m "Add clerk-raycast cask"
git push origin main
```

### 5. Test Installation

```bash
# Uninstall if already installed
brew uninstall --cask clerk-raycast

# Install from tap
brew install --cask sawmills/tap/clerk-raycast

# Verify
ls -la ~/Library/Application\ Support/Raycast/extensions/clerk-admin
```

## Manual Installation (Development)

For testing without Homebrew:

```bash
cd extensions/raycast
npm install
npm run build
./scripts/install.sh
```

## Updating the Extension

### For New Versions

1. Update version in `package.json`
2. Build and package: `./scripts/package.sh 0.2.0`
3. Create new GitHub release
4. Update cask formula with new version and SHA256
5. Push to homebrew-tap

### For Users

```bash
brew upgrade --cask clerk-raycast
```

## Cask Formula Structure

The cask does the following:

1. **Downloads** the tarball from GitHub releases
2. **Verifies** SHA256 checksum
3. **Checks** Raycast is installed
4. **Creates** Raycast extensions directory if needed
5. **Copies** extension files to Raycast
6. **Shows** setup instructions

## Troubleshooting

### "Raycast not found"
Install Raycast first: `brew install --cask raycast`

### "Extension not appearing in Raycast"
1. Restart Raycast
2. Check installation: `ls ~/Library/Application\ Support/Raycast/extensions/clerk-admin`
3. Reinstall: `brew reinstall --cask clerk-raycast`

### "SHA256 mismatch"
The tarball was modified. Regenerate with `./scripts/package.sh` and update the cask formula.

## Alternative: Raycast Store

For wider distribution, consider publishing to the official Raycast Store:

1. Fork [raycast/extensions](https://github.com/raycast/extensions)
2. Add your extension to the repository
3. Submit a pull request
4. Follow [Raycast's contribution guidelines](https://developers.raycast.com/basics/publish-an-extension)

The Homebrew approach is faster for internal/team use, while the Raycast Store provides broader reach.

## Files Overview

```
extensions/raycast/
├── scripts/
│   ├── package.sh       # Creates distributable tarball
│   └── install.sh       # Manual installation script
├── Casks/
│   └── clerk-raycast.rb # Homebrew cask formula
└── HOMEBREW_SETUP.md    # This file
```

## Benefits of Homebrew Distribution

- ✅ One-command installation
- ✅ Automatic updates via `brew upgrade`
- ✅ Consistent with CLI tool distribution
- ✅ Easy uninstallation
- ✅ Version management
- ✅ Works with existing sawmills/tap

## Next Steps

1. Test the package script: `./scripts/package.sh 0.1.0`
2. Create a test release on GitHub
3. Update the cask formula with real SHA256
4. Add to sawmills/homebrew-tap
5. Test installation: `brew install --cask sawmills/tap/clerk-raycast`
