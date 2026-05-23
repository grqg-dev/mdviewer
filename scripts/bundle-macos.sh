#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo build --release
BIN="$ROOT/target/release/mdviewer"

mkdir -p "$HOME/.cargo/bin"
cp "$BIN" "$HOME/.cargo/bin/mdviewer"
chmod +x "$HOME/.cargo/bin/mdviewer"

echo "Installed $HOME/.cargo/bin/mdviewer"
echo "Try: mdviewer '$ROOT/example.md'"
