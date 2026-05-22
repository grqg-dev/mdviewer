pub mod theme;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use eframe::egui::{self, Key, RichText, ScrollArea, Vec2, ViewportCommand};
use egui_commonmark::CommonMarkCache;

pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "mkd", "txt"];

pub fn cli_path_from(mut args: impl Iterator<Item = String>) -> Option<PathBuf> {
    args.next();
    args.next().map(PathBuf::from)
}

pub fn cli_path() -> Option<PathBuf> {
    cli_path_from(env::args())
}

pub fn title_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mdviewer")
        .to_owned()
}

pub fn scroll_after_page_down(scroll_offset: f32, viewport_height: f32) -> f32 {
    scroll_offset + viewport_height * 0.9
}

pub fn scroll_after_page_up(scroll_offset: f32, viewport_height: f32) -> f32 {
    (scroll_offset - viewport_height * 0.9).max(0.0)
}

pub fn empty_state_prompt(drag_hover: bool) -> &'static str {
    if drag_hover {
        "Drop to open"
    } else {
        "Drop a .md file here"
    }
}

pub struct ViewerApp {
    markdown: Option<String>,
    title: String,
    cache: CommonMarkCache,
    scroll_offset: f32,
    page_down: bool,
    page_up: bool,
    cli_checked: bool,
}

impl ViewerApp {
    pub fn new() -> Self {
        Self {
            markdown: None,
            title: "mdviewer".to_owned(),
            cache: CommonMarkCache::default(),
            scroll_offset: 0.0,
            page_down: false,
            page_up: false,
            cli_checked: false,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn markdown(&self) -> Option<&str> {
        self.markdown.as_deref()
    }

    pub fn scroll_offset(&self) -> f32 {
        self.scroll_offset
    }

    pub fn cli_checked(&self) -> bool {
        self.cli_checked
    }

    pub fn should_attempt_cli_load(&self) -> bool {
        !self.cli_checked && self.markdown.is_none()
    }

    pub fn load_file(&mut self, path: PathBuf) -> Result<()> {
        let markdown = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        self.title = title_from_path(&path);
        self.markdown = Some(markdown);
        self.scroll_offset = 0.0;
        self.cache = CommonMarkCache::default();
        Ok(())
    }

    fn try_load_cli_path(&mut self, ctx: &egui::Context) {
        if !self.should_attempt_cli_load() {
            return;
        }
        self.cli_checked = true;

        if let Some(path) = cli_path() {
            if self.load_file(path).is_ok() {
                ctx.send_viewport_cmd(ViewportCommand::Title(self.title.clone()));
            }
        }
    }

    fn open_file_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = pick_markdown_file() {
            if self.load_file(path).is_ok() {
                ctx.send_viewport_cmd(ViewportCommand::Title(self.title.clone()));
            }
        }
    }

    pub fn apply_page_scroll(&mut self, viewport_height: f32) {
        if self.page_down {
            self.scroll_offset = scroll_after_page_down(self.scroll_offset, viewport_height);
        }
        if self.page_up {
            self.scroll_offset = scroll_after_page_up(self.scroll_offset, viewport_height);
        }
    }
}

impl eframe::App for ViewerApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.try_load_cli_path(ctx);

        let dropped = ctx.input(|input| input.raw.dropped_files.clone());
        if let Some(file) = dropped.into_iter().find_map(|f| f.path) {
            if self.load_file(file).is_ok() {
                ctx.send_viewport_cmd(ViewportCommand::Title(self.title.clone()));
            }
        }

        if ctx.input(|input| !input.raw.hovered_files.is_empty()) {
            ctx.request_repaint();
        }

        if ctx.input(|input| input.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        self.page_down = ctx.input(|input| {
            input.key_pressed(Key::PageDown)
                || input.key_pressed(Key::Space)
                || (input.key_pressed(Key::ArrowDown) && input.modifiers.command)
        });
        self.page_up = ctx.input(|input| {
            input.key_pressed(Key::PageUp)
                || (input.key_pressed(Key::ArrowUp) && input.modifiers.command)
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.style_mut().url_in_tooltip = true;

        let drag_hover = ui.ctx().input(|input| !input.raw.hovered_files.is_empty());

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(40, 32)),
            )
            .show_inside(ui, |ui| {
                if self.markdown.is_none() {
                    let ctx = ui.ctx().clone();
                    show_empty_state(ui, drag_hover, || self.open_file_dialog(&ctx));
                    return;
                }

                let viewport_height = ui.available_height();
                self.apply_page_scroll(viewport_height);

                let markdown = self.markdown.as_ref().expect("markdown loaded");
                let output = ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .scroll_offset(Vec2::new(0.0, self.scroll_offset))
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                let width = ui.available_width().min(theme::COLUMN_MAX_WIDTH);
                                ui.set_width(width);
                                theme::markdown_viewer()
                                    .max_image_width(Some(width as usize))
                                    .show(ui, &mut self.cache, markdown);
                            },
                        );
                    });

                self.scroll_offset = output.state.offset.y;
            });
    }
}

fn show_empty_state(ui: &mut egui::Ui, drag_hover: bool, open_file: impl FnOnce()) {
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(empty_state_prompt(drag_hover))
                    .color(if drag_hover {
                        theme::LINK
                    } else {
                        theme::MUTED
                    })
                    .size(20.0),
            );
            ui.add_space(16.0);
            if ui.button("Open file…").clicked() {
                open_file();
            }
        });
    });
}

fn pick_markdown_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Markdown", MARKDOWN_EXTENSIONS)
        .set_title("Open Markdown")
        .pick_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn cli_path_from_no_args_returns_none() {
        assert_eq!(cli_path_from(["mdviewer".to_owned()].into_iter()), None);
    }

    #[test]
    fn cli_path_from_one_arg_returns_path() {
        assert_eq!(
            cli_path_from(["mdviewer".to_owned(), "notes.md".to_owned()].into_iter()),
            Some(PathBuf::from("notes.md"))
        );
    }

    #[test]
    fn cli_path_from_uses_first_arg_only() {
        assert_eq!(
            cli_path_from(
                [
                    "mdviewer".to_owned(),
                    "first.md".to_owned(),
                    "second.md".to_owned(),
                ]
                .into_iter()
            ),
            Some(PathBuf::from("first.md"))
        );
    }

    #[test]
    fn title_from_path_uses_file_name() {
        assert_eq!(
            title_from_path(Path::new("/tmp/docs/readme.md")),
            "readme.md"
        );
    }

    #[test]
    fn title_from_path_falls_back_for_empty_name() {
        assert_eq!(title_from_path(Path::new("")), "mdviewer");
    }

    #[test]
    fn scroll_after_page_down_advances_by_ninety_percent() {
        assert_eq!(scroll_after_page_down(100.0, 500.0), 550.0);
    }

    #[test]
    fn scroll_after_page_up_retreats_by_ninety_percent() {
        assert_eq!(scroll_after_page_up(550.0, 500.0), 100.0);
    }

    #[test]
    fn scroll_after_page_up_never_goes_below_zero() {
        assert_eq!(scroll_after_page_up(50.0, 500.0), 0.0);
    }

    #[test]
    fn empty_state_prompt_reflects_drag_hover() {
        assert_eq!(empty_state_prompt(false), "Drop a .md file here");
        assert_eq!(empty_state_prompt(true), "Drop to open");
    }

    #[test]
    fn markdown_extensions_include_common_suffixes() {
        assert!(MARKDOWN_EXTENSIONS.contains(&"md"));
        assert!(MARKDOWN_EXTENSIONS.contains(&"markdown"));
        assert!(MARKDOWN_EXTENSIONS.contains(&"txt"));
    }

    #[test]
    fn load_file_reads_content_and_resets_scroll() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# Hello").unwrap();

        let mut app = ViewerApp::new();
        app.scroll_offset = 42.0;
        app.load_file(file.path().to_path_buf()).unwrap();

        assert_eq!(app.markdown(), Some("# Hello\n"));
        assert_eq!(app.title(), file.path().file_name().unwrap().to_str().unwrap());
        assert_eq!(app.scroll_offset(), 0.0);
    }

    #[test]
    fn load_file_errors_for_missing_path() {
        let mut app = ViewerApp::new();
        let err = app
            .load_file(PathBuf::from("/no/such/file.md"))
            .unwrap_err();

        assert!(err.to_string().contains("failed to read"));
        assert!(err.to_string().contains("file.md"));
    }

    #[test]
    fn should_attempt_cli_load_only_when_uninitialized() {
        let mut app = ViewerApp::new();
        assert!(app.should_attempt_cli_load());

        app.cli_checked = true;
        assert!(!app.should_attempt_cli_load());

        let mut app = ViewerApp::new();
        app.markdown = Some(String::new());
        assert!(!app.should_attempt_cli_load());
    }

    #[test]
    fn apply_page_scroll_updates_offset() {
        let mut app = ViewerApp::new();
        app.scroll_offset = 100.0;
        app.page_down = true;
        app.apply_page_scroll(500.0);
        assert_eq!(app.scroll_offset(), 550.0);

        app.page_down = false;
        app.page_up = true;
        app.apply_page_scroll(500.0);
        assert_eq!(app.scroll_offset(), 100.0);
    }

    #[test]
    fn apply_page_scroll_applies_both_when_both_flags_set() {
        let mut app = ViewerApp::new();
        app.scroll_offset = 100.0;
        app.page_down = true;
        app.page_up = true;
        app.apply_page_scroll(500.0);
        // page down then page up in the same frame cancel out
        assert_eq!(app.scroll_offset(), 100.0);
    }
}
