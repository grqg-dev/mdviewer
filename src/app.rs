use std::path::PathBuf;

use anyhow::Result;
use eframe::egui::{self, Key, RichText, ScrollArea, Vec2, ViewportCommand};
use egui_commonmark::CommonMarkCache;

use crate::files::{self, scroll_after_page_down, scroll_after_page_up};
use crate::theme;

pub struct ViewerApp {
    markdown: Option<String>,
    title: String,
    cache: CommonMarkCache,
    scroll_offset: f32,
    pending_title: Option<String>,
}

impl ViewerApp {
    pub fn new() -> Self {
        let mut app = Self {
            markdown: None,
            title: "mdviewer".to_owned(),
            cache: CommonMarkCache::default(),
            scroll_offset: 0.0,
            pending_title: None,
        };

        if let Some(path) = files::cli_path() {
            if app.open(path).is_ok() {
                app.pending_title = Some(app.title.clone());
            }
        }

        app
    }

    pub fn open(&mut self, path: PathBuf) -> Result<()> {
        let (markdown, title) = files::read_markdown(&path)?;
        self.title = title;
        self.markdown = Some(markdown);
        self.scroll_offset = 0.0;
        self.cache = CommonMarkCache::default();
        Ok(())
    }

    pub fn apply_page_scroll(&mut self, viewport_height: f32, page_down: bool, page_up: bool) {
        if page_down {
            self.scroll_offset = scroll_after_page_down(self.scroll_offset, viewport_height);
        }
        if page_up {
            self.scroll_offset = scroll_after_page_up(self.scroll_offset, viewport_height);
        }
    }

    fn open_with_title(&mut self, path: PathBuf, ctx: &egui::Context) {
        if self.open(path).is_ok() {
            ctx.send_viewport_cmd(ViewportCommand::Title(self.title.clone()));
        }
    }

    fn handle_drops(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|input| input.raw.dropped_files.clone());
        if let Some(path) = dropped.into_iter().find_map(|f| f.path) {
            self.open_with_title(path, ctx);
        }
        if ctx.input(|input| !input.raw.hovered_files.is_empty()) {
            ctx.request_repaint();
        }
    }

    fn page_scroll_keys(ctx: &egui::Context) -> (bool, bool) {
        ctx.input(|input| {
            let cmd = input.modifiers.command;
            (
                input.key_pressed(Key::PageDown)
                    || input.key_pressed(Key::Space)
                    || (input.key_pressed(Key::ArrowDown) && cmd),
                input.key_pressed(Key::PageUp)
                    || (input.key_pressed(Key::ArrowUp) && cmd),
            )
        })
    }

    fn show_markdown(&mut self, ui: &mut egui::Ui) {
        let (page_down, page_up) = Self::page_scroll_keys(ui.ctx());
        self.apply_page_scroll(ui.available_height(), page_down, page_up);

        let markdown = self.markdown.as_ref().unwrap();
        let output = ScrollArea::vertical()
            .auto_shrink([false; 2])
            .scroll_offset(Vec2::new(0.0, self.scroll_offset))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    let width = ui.available_width().min(theme::COLUMN_MAX_WIDTH);
                    ui.set_width(width);
                    theme::markdown_viewer()
                        .max_image_width(Some(width as usize))
                        .show(ui, &mut self.cache, markdown);
                });
            });

        self.scroll_offset = output.state.offset.y;
    }

    fn show_empty(&mut self, ui: &mut egui::Ui, drag_hover: bool) {
        let ctx = ui.ctx().clone();
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(files::empty_state_prompt(drag_hover))
                        .color(if drag_hover {
                            theme::LINK
                        } else {
                            theme::MUTED
                        })
                        .size(20.0),
                );
                ui.add_space(16.0);
                if ui.button("Open file…").clicked() {
                    if let Some(path) = files::pick_markdown_file() {
                        self.open_with_title(path, &ctx);
                    }
                }
            });
        });
    }
}

impl eframe::App for ViewerApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(title) = self.pending_title.take() {
            ctx.send_viewport_cmd(ViewportCommand::Title(title));
        }

        self.handle_drops(ctx);

        if ctx.input(|input| input.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
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
                    self.show_empty(ui, drag_hover);
                } else {
                    self.show_markdown(ui);
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn open_reads_content_and_resets_scroll() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# Hello").unwrap();

        let mut app = ViewerApp::new();
        app.scroll_offset = 42.0;
        app.open(file.path().to_path_buf()).unwrap();

        assert_eq!(app.markdown.as_deref(), Some("# Hello\n"));
        assert_eq!(app.title, file.path().file_name().unwrap().to_str().unwrap());
        assert_eq!(app.scroll_offset, 0.0);
    }

    #[test]
    fn apply_page_scroll_updates_offset() {
        let mut app = ViewerApp::new();
        app.scroll_offset = 100.0;
        app.apply_page_scroll(500.0, true, false);
        assert_eq!(app.scroll_offset, 550.0);

        app.apply_page_scroll(500.0, false, true);
        assert_eq!(app.scroll_offset, 100.0);
    }

    #[test]
    fn apply_page_scroll_applies_both_when_both_flags_set() {
        let mut app = ViewerApp::new();
        app.scroll_offset = 100.0;
        app.apply_page_scroll(500.0, true, true);
        assert_eq!(app.scroll_offset, 100.0);
    }
}
