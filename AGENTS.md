# mdviewer — agent guide

Native macOS markdown viewer (Rust + egui). Read this before changing code, rebuilding, or opening files for the user.

## Preview markdown for the user

When the user asks to preview, open, or view a markdown file, run `mdviewer` from the terminal. Do **not** use the browser or IDE markdown preview.

```bash
mdviewer /absolute/path/to/file.md
mdviewer --style glow-mocha /absolute/path/to/file.md
```

Use absolute paths when the working directory is unclear. If mdviewer is already running, a new invocation opens another window in the same app.

### Styles

| Style | Names |
|-------|-------|
| default | `default`, `main` |
| glow-latte | `glow-latte`, `glow`, `latte` |
| glow-frappe | `glow-frappe`, `frappe` |
| glow-macchiato | `glow-macchiato`, `macchiato` |
| glow-mocha | `glow-mocha`, `mocha` |

Style priority: `--style` / `-s` → `MDVIEWER_STYLE` → `~/.config/mdviewer/config.toml` → `default`.

After writing or editing a markdown file the user should read, offer to open it with mdviewer.

## Build and install (important)

The shell command `mdviewer` runs from **`~/.cargo/bin/mdviewer`**, not from `target/debug/` or `target/release/`.

| Command | Updates `~/.cargo/bin/mdviewer`? |
|---------|----------------------------------|
| `cargo install --path . --force` | **Yes** |
| `./scripts/bundle-macos.sh` | **Yes** (release build + copy) |
| `cargo build` / `cargo build --release` | **No** — only updates `target/` |
| `cargo run -- file.md` | **No** — runs from `target/` for that invocation only |

After changing source code, **always reinstall** before asking the user to test:

```bash
cd /path/to/mdviewer
cargo test
cargo install --path . --force
```

Verify the active binary:

```bash
which mdviewer   # should print ~/.cargo/bin/mdviewer
```

First-time install from a clone:

```bash
cargo install --path .
```

Install from GitHub (no local clone): `cargo install --git https://github.com/grqg-dev/mdviewer.git`

## Project layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | Entry point |
| `src/app.rs` | Window, scroll, IPC, multi-window |
| `src/theme/` | Default and Glow themes, `show_markdown` dispatch |
| `src/glow/` | Custom Glow markdown renderer (Catppuccin) |
| `src/config.rs` | Style resolution (CLI → env → config file) |
| `src/ipc.rs` | Single-instance Unix socket |
| `scripts/bundle-macos.sh` | Release build + install to `~/.cargo/bin` |

Default style uses `egui_commonmark::CommonMarkViewer`. Glow styles use a custom `GlowRenderer` in `src/glow/render.rs`.

## Coding notes

- macOS only; keep changes focused and match existing module style.
- Two render paths: `Style::Default` and `Style::Glow` — fix wrapping/layout bugs in both when they share root cause.
- Glow markdown width is passed from `app.rs` into `show_glow_markdown`; do not rely on `options.max_width(ui)` alone (it uses `max(max_image_width, available_width)` and can exceed the column width).
- Only create git commits when the user asks.
