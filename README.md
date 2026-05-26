# mdviewer

A minimal native markdown viewer for macOS. Open a file from the terminal, read it in a clean window, move on.

Built for terminal-heavy workflows — preview docs, notes, and script output without opening a browser or IDE.

## Features

- Native GUI window (Rust + [egui](https://github.com/emilk/egui))
- CommonMark rendering: headings, lists, tables, blockquotes, code blocks, links, images
- Syntax highlighting in fenced code blocks
- Six visual styles: clean **default**, or **[Glow](https://github.com/charmbracelet/glow)-inspired Catppuccin** themes (Latte, Frappé, Macchiato, Mocha) with terminal font
- Drag-and-drop or file picker to open documents
- Multiple windows share one app instance — Cmd+`~` works as expected
- Keyboard scrolling and quick quit

## Install

**Requirements:** macOS, Rust (stable), and `~/.cargo/bin` on your `PATH`.

### From source (recommended)

```bash
git clone https://github.com/grqg-dev/mdviewer.git
cd mdviewer
cargo install --path .
```

That puts `mdviewer` in `~/.cargo/bin`. Verify:

```bash
which mdviewer
mdviewer --style glow-mocha README.md
```

### Upgrade after pulling changes

```bash
cd mdviewer
git pull
cargo install --path . --force
```

### Run without installing

```bash
cargo run -- path/to/file.md
cargo run -- --style glow-mocha path/to/file.md
```

### Install from GitHub (no local clone)

```bash
cargo install --git https://github.com/grqg-dev/mdviewer.git
```

Use `--force` to replace an existing install.

## Usage

```bash
mdviewer notes.md
mdviewer --style glow-latte stoicism.md
mdviewer --style glow-mocha stoicism.md
mdviewer -s frappe coffee.md          # short flag; flavor names work too
mdviewer                            # open empty window, drop a file or click "Open file…"
```

Supported file extensions: `.md`, `.markdown`, `.mdown`, `.mkd`, `.txt`.

Each additional invocation while the app is running opens a new window in the existing process instead of spawning another Dock icon.

## Styles

| Style | Names | Description |
|-------|-------|-------------|
| **default** | `default`, `main` | Light GitHub-style layout, proportional body text, SF Pro / SF Mono |
| **glow-latte** | `glow-latte`, `glow`, `latte` | Light Catppuccin Latte — Glow-like terminal aesthetic, monospace font |
| **glow-frappe** | `glow-frappe`, `frappe` | Dark Catppuccin Frappé |
| **glow-macchiato** | `glow-macchiato`, `macchiato` | Dark Catppuccin Macchiato |
| **glow-mocha** | `glow-mocha`, `mocha` | Dark Catppuccin Mocha |

All glow styles read your Ghostty `font-family` when available, falling back to Fira Code / JetBrains Mono / SF Mono.

Pick a style with config, env var, or CLI flag. Priority:

1. `--style` / `-s`
2. `MDVIEWER_STYLE`
3. `~/.config/mdviewer/config.toml`
4. `default`

### Config file

```toml
# ~/.config/mdviewer/config.toml
style = "glow-mocha"
```

## For Claude (and other agents)

See **[AGENTS.md](AGENTS.md)** for preview commands, style names, and the build/install workflow agents must follow after code changes.

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Space` / `Page Down` | Scroll down |
| `Page Up` | Scroll up |
| `Cmd + ↓` / `Cmd + ↑` | Page scroll |
| `Esc` / `q` | Close window |
| `Cmd + ~` | Switch between open windows |

## Multiple windows

The first `mdviewer` process listens on a Unix socket at `$TMPDIR/mdviewer-{user}.sock`. Later invocations send the file path to that process and exit immediately, so all windows belong to one macOS app.

## Build & test

```bash
cargo test
cargo install --path . --force   # updates ~/.cargo/bin/mdviewer (what the shell runs)
```

For a release binary in `target/` only, without updating the install:

```bash
cargo build --release
./target/release/mdviewer example.md
```

Or use the install script:

```bash
./scripts/bundle-macos.sh
```

## Stack

- [eframe](https://github.com/emilk/egui) / egui — immediate-mode GUI
- [egui_commonmark](https://github.com/lampsitter/egui_commonmark) — markdown parsing and rendering
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — CommonMark parser
- Unix domain sockets — single-instance IPC
