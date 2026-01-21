#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSION_DIR="$(dirname "$SCRIPT_DIR")"
VERSION="${1:-0.1.0}"

echo "📦 Packaging Raycast extension v${VERSION}..."

cd "$EXTENSION_DIR"

npm install --production

PACKAGE_DIR="clerk-raycast-${VERSION}"
mkdir -p "$PACKAGE_DIR"

cp -r src "$PACKAGE_DIR/"
cp -r assets "$PACKAGE_DIR/"
cp package.json "$PACKAGE_DIR/"
cp tsconfig.json "$PACKAGE_DIR/"
cp README.md "$PACKAGE_DIR/"
cp -r node_modules "$PACKAGE_DIR/" 2>/dev/null || echo "Skipping node_modules (will be installed on target)"

tar -czf "${PACKAGE_DIR}.tar.gz" "$PACKAGE_DIR"

rm -rf "$PACKAGE_DIR"

echo "✅ Package created: ${PACKAGE_DIR}.tar.gz"
echo ""
echo "To create a GitHub release:"
echo "  1. git tag raycast-v${VERSION}"
echo "  2. git push origin raycast-v${VERSION}"
echo "  3. gh release create raycast-v${VERSION} ${PACKAGE_DIR}.tar.gz --title 'Raycast Extension v${VERSION}'"
echo ""
echo "SHA256: $(shasum -a 256 "${PACKAGE_DIR}.tar.gz" | cut -d' ' -f1)"
