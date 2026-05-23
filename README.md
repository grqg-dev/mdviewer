# mdviewer

A minimal native markdown viewer for macOS. Open a file from the terminal, read it in a clean window, move on.

Built for terminal-heavy workflows — preview docs, notes, and script output without opening a browser or IDE.

## Features

- Native GUI window (Rust + [egui](https://github.com/emilk/egui))
- CommonMark rendering: headings, lists, tables, blockquotes, code blocks, links, images
- Syntax highlighting in fenced code blocks
- Two visual styles: clean **default** or **[Glow](https://github.com/charmbracelet/glow)-inspired glow-latte** (Catppuccin Latte, terminal font)
- Drag-and-drop or file picker to open documents
- Multiple windows share one app instance — Cmd+`~` works as expected
- Keyboard scrolling and quick quit

## Install

Requires Rust (stable) and macOS.

```bash
cargo install --path .
```

Or run directly from the repo:

```bash
cargo run -- path/to/file.md
```

## Usage

```bash
mdviewer notes.md
mdviewer --style glow-latte stoicism.md
mdviewer                    # open empty window, drop a file or click "Open file…"
```

Each additional invocation while the app is running opens a new window in the existing process instead of spawning another Dock icon.

## Styles

| Style | Names | Description |
|-------|-------|-------------|
| **default** | `default`, `main` | Light GitHub-style layout, proportional body text, SF Pro / SF Mono |
| **glow-latte** | `glow-latte`, `glow` | Glow-like terminal aesthetic — Catppuccin Latte colors, monospace font (reads your Ghostty `font-family` when available) |

Pick a style with config, env var, or CLI flag. Priority:

1. `--style` / `-s`
2. `MDVIEWER_STYLE`
3. `~/.config/mdviewer/config.toml`
4. `default`

### Config file

```toml
# ~/.config/mdviewer/config.toml
style = "glow-latte"
```

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
cargo build --release
cargo test
```

## Stack

- [eframe](https://github.com/emilk/egui) / egui — immediate-mode GUI
- [egui_commonmark](https://github.com/lampsitter/egui_commonmark) — markdown parsing and rendering
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — CommonMark parser
- Unix domain sockets — single-instance IPC
