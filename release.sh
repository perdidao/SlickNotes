#!/bin/bash
set -e

# Get current version from tauri.conf.json
CURRENT=$(grep '"version"' src-tauri/tauri.conf.json | head -1 | sed 's/.*"\([0-9]*\.[0-9]*\.[0-9]*\)".*/\1/')
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

echo "Current version: $CURRENT"
echo ""
echo "Select release type:"
echo "  1) patch  → $MAJOR.$MINOR.$((PATCH + 1))"
echo "  2) minor  → $MAJOR.$((MINOR + 1)).0"
echo "  3) major  → $((MAJOR + 1)).0.0"
echo ""
read -rp "Choice [1/2/3]: " CHOICE

case "$CHOICE" in
  1) NEW="$MAJOR.$MINOR.$((PATCH + 1))" ;;
  2) NEW="$MAJOR.$((MINOR + 1)).0" ;;
  3) NEW="$((MAJOR + 1)).0.0" ;;
  *) echo "Invalid choice"; exit 1 ;;
esac

echo ""
echo "Bumping $CURRENT → $NEW"

# Bump version in tauri.conf.json and package.json
sed -i "s/\"version\": \"$CURRENT\"/\"version\": \"$NEW\"/" src-tauri/tauri.conf.json
sed -i "s/\"version\": \"$CURRENT\"/\"version\": \"$NEW\"/" package.json

# Commit version bump
git add src-tauri/tauri.conf.json package.json
git commit -m "chore: bump version to $NEW"

# Build
echo ""
echo "Building release..."
source "$HOME/.cargo/env"
pnpm tauri build

# Tag and push
git tag "v$NEW"
git push origin main
git push origin "v$NEW"

# Create GitHub release with .deb
DEB=$(find src-tauri/target/release/bundle/deb -name "*.deb" | head -1)
gh release create "v$NEW" "$DEB" \
  --title "SlickNotes v$NEW" \
  --generate-notes

echo ""
echo "Released v$NEW"
