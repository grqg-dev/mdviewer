use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;
use eframe::egui::{self, Key, RichText, ScrollArea, Vec2, ViewportCommand, ViewportId};
use egui_commonmark::CommonMarkCache;

use crate::config::Style;
use crate::files::{self, scroll_after_page_down, scroll_after_page_up};
use crate::theme::{self, Palette};

static NEXT_VIEWPORT_ID: AtomicU64 = AtomicU64::new(1);

pub struct DocumentWindow {
    viewport_id: ViewportId,
    markdown: Option<String>,
    title: String,
    cache: CommonMarkCache,
    scroll_offset: f32,
    pending_title: Option<String>,
    close_requested: bool,
    style: Style,
    palette: Palette,
}

impl DocumentWindow {
    pub fn empty(style: Style) -> Self {
        Self {
            viewport_id: next_viewport_id(),
            markdown: None,
            title: "mdviewer".to_owned(),
            cache: CommonMarkCache::default(),
            scroll_offset: 0.0,
            pending_title: None,
            close_requested: false,
            style,
            palette: Palette::for_style(style),
        }
    }

    pub fn from_path(path: PathBuf, style: Style) -> Self {
        let mut window = Self::empty(style);
        if window.open(path).is_ok() {
            window.pending_title = Some(window.title.clone());
        }
        window
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

    fn viewport_builder(&self) -> egui::ViewportBuilder {
        egui::ViewportBuilder::default()
            .with_title(&self.title)
            .with_inner_size([960.0, 720.0])
            .with_min_inner_size([480.0, 320.0])
            .with_active(true)
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
                ui.vertical_centered(|ui| {
                    let width = ui.available_width().min(theme::column_max_width(self.style));
                    ui.set_max_width(width);
                    theme::show_markdown(ui, &mut self.cache, markdown, width, self.style);
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
                            self.palette.link
                        } else {
                            self.palette.muted
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

    fn logic(&mut self, ctx: &egui::Context) {
        if let Some(title) = self.pending_title.take() {
            ctx.send_viewport_cmd(ViewportCommand::Title(title));
        }

        self.handle_drops(ctx);

        if ctx.input(|input| input.key_pressed(Key::Escape) || input.key_pressed(Key::Q)) {
            self.close_requested = true;
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        if ctx.input(|input| input.viewport().close_requested()) {
            self.close_requested = true;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().url_in_tooltip = true;

        let drag_hover = ui.ctx().input(|input| !input.raw.hovered_files.is_empty());

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(self.palette.bg)
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

pub struct ViewerApp {
    // The first document is rendered directly in the root eframe window so the
    // root window is always a real, full-size NSWindow. Extra documents get their
    // own deferred viewports. This prevents the old 1×1 invisible root window
    // from appearing in the macOS Cmd+~ window cycle and crashing on focus.
    main_doc: DocumentWindow,
    extra_docs: Vec<Arc<Mutex<DocumentWindow>>>,
    ipc_rx: mpsc::Receiver<Option<PathBuf>>,
    style: Style,
}

impl ViewerApp {
    pub fn new(initial_path: Option<PathBuf>, ipc_rx: mpsc::Receiver<Option<PathBuf>>, style: Style) -> Self {
        let main_doc = match initial_path {
            Some(path) => DocumentWindow::from_path(path, style),
            None => DocumentWindow::empty(style),
        };
        Self {
            main_doc,
            extra_docs: vec![],
            ipc_rx,
            style,
        }
    }

    fn drain_ipc(&mut self) {
        while let Ok(path) = self.ipc_rx.try_recv() {
            let doc = match path {
                Some(p) => DocumentWindow::from_path(p, self.style),
                None => DocumentWindow::empty(self.style),
            };
            self.extra_docs.push(Arc::new(Mutex::new(doc)));
        }
    }

    fn remove_closed_extras(&mut self) {
        self.extra_docs
            .retain(|w| !w.lock().unwrap().close_requested);
    }

    fn show_extra_viewports(&self, ui: &mut egui::Ui) {
        for window in &self.extra_docs {
            let window = Arc::clone(window);
            let doc = window.lock().unwrap();
            if doc.close_requested {
                continue;
            }
            let viewport_id = doc.viewport_id;
            let builder = doc.viewport_builder();
            drop(doc);

            ui.ctx().show_viewport_deferred(viewport_id, builder, move |ui, class| {
                if class == egui::ViewportClass::EmbeddedWindow {
                    ui.label("mdviewer requires native windows.");
                    return;
                }
                let Ok(mut doc) = window.lock() else { return };
                doc.logic(ui.ctx());
                doc.ui(ui);
            });
        }
    }
}

impl eframe::App for ViewerApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_ipc();
        self.remove_closed_extras();
        self.main_doc.logic(ctx);

        if self.main_doc.close_requested {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.main_doc.ui(ui);
        self.show_extra_viewports(ui);
    }
}

fn next_viewport_id() -> ViewportId {
    ViewportId::from_hash_of(NEXT_VIEWPORT_ID.fetch_add(1, Ordering::Relaxed))
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

        let mut window = DocumentWindow::empty(Style::Default);
        window.scroll_offset = 42.0;
        window.open(file.path().to_path_buf()).unwrap();

        assert_eq!(window.markdown.as_deref(), Some("# Hello\n"));
        assert_eq!(window.title, file.path().file_name().unwrap().to_str().unwrap());
        assert_eq!(window.scroll_offset, 0.0);
    }

    #[test]
    fn apply_page_scroll_updates_offset() {
        let mut window = DocumentWindow::empty(Style::Default);
        window.scroll_offset = 100.0;
        window.apply_page_scroll(500.0, true, false);
        assert_eq!(window.scroll_offset, 550.0);

        window.apply_page_scroll(500.0, false, true);
        assert_eq!(window.scroll_offset, 100.0);
    }

    #[test]
    fn apply_page_scroll_applies_both_when_both_flags_set() {
        let mut window = DocumentWindow::empty(Style::Default);
        window.scroll_offset = 100.0;
        window.apply_page_scroll(500.0, true, true);
        assert_eq!(window.scroll_offset, 100.0);
    }
}
