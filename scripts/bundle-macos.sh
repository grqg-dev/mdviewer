#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo build --release
BIN="$ROOT/target/release/mdviewer"
APP="$ROOT/MdViewer.app"

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

cp "$BIN" "$APP/Contents/MacOS/mdviewer-bin"
chmod +x "$APP/Contents/MacOS/mdviewer-bin"

swiftc -O -o "$APP/Contents/MacOS/mdviewer" "$ROOT/macos/Launcher.swift"
chmod +x "$APP/Contents/MacOS/mdviewer"

cat > "$APP/Contents/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>mdviewer</string>
  <key>CFBundleIdentifier</key>
  <string>com.mattdennis.mdviewer</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>mdviewer</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>0.1.0</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>CFBundleDocumentTypes</key>
  <array>
    <dict>
      <key>CFBundleTypeExtensions</key>
      <array>
        <string>md</string>
        <string>markdown</string>
      </array>
      <key>CFBundleTypeName</key>
      <string>Markdown</string>
      <key>CFBundleTypeRole</key>
      <string>Viewer</string>
    </dict>
  </array>
</dict>
</plist>
EOF

cp "$BIN" "$HOME/.cargo/bin/mdviewer"
chmod +x "$HOME/.cargo/bin/mdviewer"

echo "Built $APP"
echo "Try: open -a '$APP' '$ROOT/example.md'"
