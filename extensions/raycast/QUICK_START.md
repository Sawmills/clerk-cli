# Quick Start: Publishing to Homebrew

## TL;DR

```bash
# 1. Package the extension
cd extensions/raycast
./scripts/package.sh 0.1.0

# 2. Create GitHub release
git tag raycast-v0.1.0
git push origin raycast-v0.1.0
gh release create raycast-v0.1.0 clerk-raycast-0.1.0.tar.gz \
  --title "Raycast Extension v0.1.0" \
  --notes "Initial release"

# 3. Update cask formula with SHA256
shasum -a 256 clerk-raycast-0.1.0.tar.gz
# Copy the SHA256 and update Casks/clerk-raycast.rb

# 4. Add to homebrew-tap
cd ~/path/to/homebrew-tap
cp /path/to/clerk-cli/extensions/raycast/Casks/clerk-raycast.rb Casks/
git add Casks/clerk-raycast.rb
git commit -m "Add clerk-raycast cask"
git push

# 5. Test installation
brew install --cask sawmills/tap/clerk-raycast
```

## Users Install With

```bash
brew install --cask sawmills/tap/clerk-raycast
```

That's it! The extension will be installed to Raycast automatically.

## What Happens During Installation

1. Homebrew downloads `clerk-raycast-0.1.0.tar.gz` from GitHub releases
2. Verifies SHA256 checksum
3. Extracts to `~/Library/Application Support/Raycast/extensions/clerk-admin`
4. Runs `npm install --production` in the extension directory
5. Shows setup instructions

## Updating

### For You (Maintainer)

1. Update version in `package.json`
2. Run `./scripts/package.sh 0.2.0`
3. Create new GitHub release
4. Update cask formula (version + SHA256)
5. Push to homebrew-tap

### For Users

```bash
brew upgrade --cask clerk-raycast
```

## Files Overview

```
extensions/raycast/
├── scripts/
│   ├── package.sh              # Creates tarball for distribution
│   └── install.sh              # Manual installation (dev/testing)
├── Casks/
│   └── clerk-raycast.rb        # Homebrew cask formula
├── HOMEBREW_SETUP.md           # Detailed guide
└── QUICK_START.md              # This file
```

## Next Steps

1. **Test locally**: `./scripts/install.sh` (installs to your Raycast)
2. **Create first release**: Follow TL;DR steps above
3. **Add to tap**: Copy cask to sawmills/homebrew-tap
4. **Announce**: Users can now `brew install --cask sawmills/tap/clerk-raycast`

## Troubleshooting

**"Raycast not found"**
→ User needs to install Raycast first: `brew install --cask raycast`

**"Extension not appearing"**
→ Restart Raycast (⌘ R) or reinstall: `brew reinstall --cask clerk-raycast`

**"SHA256 mismatch"**
→ Regenerate tarball and update cask formula with new SHA256

## See Also

- [HOMEBREW_SETUP.md](./HOMEBREW_SETUP.md) - Detailed publishing guide
- [README.md](./README.md) - Extension documentation
- [sawmills/homebrew-tap](https://github.com/Sawmills/homebrew-tap) - Your Homebrew tap
