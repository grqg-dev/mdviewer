mod theme;

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use eframe::egui::{self, Key, RichText, ScrollArea, Vec2, ViewportCommand};
use egui_commonmark::CommonMarkCache;

fn main() -> Result<()> {
    eframe::run_native(
        "mdviewer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("mdviewer")
                .with_inner_size([960.0, 720.0])
                .with_min_inner_size([480.0, 320.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            theme::setup(&cc.egui_ctx);
            Ok(Box::new(ViewerApp::new()))
        }),
    )
    .map_err(|err| anyhow::anyhow!("{err}"))
}

struct ViewerApp {
    markdown: Option<String>,
    title: String,
    cache: CommonMarkCache,
    scroll_offset: f32,
    page_down: bool,
    page_up: bool,
    cli_checked: bool,
}

impl ViewerApp {
    fn new() -> Self {
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

    fn load_file(&mut self, path: PathBuf) -> Result<()> {
        let markdown = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        self.title = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("mdviewer")
            .to_owned();
        self.markdown = Some(markdown);
        self.scroll_offset = 0.0;
        self.cache = CommonMarkCache::default();
        Ok(())
    }

    fn try_load_cli_path(&mut self, ctx: &egui::Context) {
        if self.cli_checked || self.markdown.is_some() {
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
                if self.page_down {
                    self.scroll_offset += viewport_height * 0.9;
                }
                if self.page_up {
                    self.scroll_offset = (self.scroll_offset - viewport_height * 0.9).max(0.0);
                }

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
            let prompt = if drag_hover {
                "Drop to open"
            } else {
                "Drop a .md file here"
            };
            ui.label(
                RichText::new(prompt)
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

fn cli_path() -> Option<PathBuf> {
    env::args().nth(1).map(PathBuf::from)
}

fn pick_markdown_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd", "txt"])
        .set_title("Open Markdown")
        .pick_file()
}
