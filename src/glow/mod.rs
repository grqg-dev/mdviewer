mod list;
mod render;
mod style;

use eframe::egui;
use egui_commonmark::CommonMarkCache;
use egui_commonmark_backend::misc::{prepare_show, CommonMarkOptions};

use crate::theme::GlowPalette;

pub fn show_glow_markdown(
    ui: &mut egui::Ui,
    cache: &mut CommonMarkCache,
    markdown: &str,
    max_width: f32,
    palette: GlowPalette,
) -> egui::InnerResponse<()> {
    prepare_show(cache, ui.ctx());

    let options = CommonMarkOptions {
        indentation_spaces: 2,
        max_image_width: Some(max_width as usize),
        theme_light: "base16-ocean.light".into(),
        theme_dark: "base16-ocean.dark".into(),
        ..Default::default()
    };

    render::GlowRenderer::new(palette).show(ui, cache, &options, markdown, max_width)
}
