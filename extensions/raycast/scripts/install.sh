#!/bin/bash
set -e

RAYCAST_EXTENSIONS_DIR="$HOME/Library/Application Support/com.raycast.macos/extensions"
EXTENSION_NAME="clerk-admin"

if [ ! -d "$RAYCAST_EXTENSIONS_DIR" ]; then
  echo "❌ Raycast not found. Please install Raycast first: https://raycast.com"
  exit 1
fi

INSTALL_DIR="$RAYCAST_EXTENSIONS_DIR/$EXTENSION_NAME"

echo "📦 Installing Clerk Admin extension to Raycast..."

mkdir -p "$INSTALL_DIR"

cp -r src "$INSTALL_DIR/"
cp -r assets "$INSTALL_DIR/"
cp package.json "$INSTALL_DIR/"
cp tsconfig.json "$INSTALL_DIR/"

cd "$INSTALL_DIR"
npm install --production

echo "✅ Clerk Admin extension installed!"
echo ""
echo "To use:"
echo "  1. Restart Raycast or press ⌘ R to reload"
echo "  2. Open Raycast (⌘ Space)"
echo "  3. Search for 'Clerk Admin'"
echo "  4. Configure your API key in preferences (⌘ ,)"
echo ""
echo "Available commands:"
echo "  - Search Organizations"
echo "  - Search Users"
echo "  - Impersonate User"
echo "  - Generate JWT"
echo "  - Organization Members"
