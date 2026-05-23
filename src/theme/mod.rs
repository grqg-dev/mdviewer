mod default;
mod fonts;
pub mod glow_latte;

use eframe::egui::Color32;
use egui_commonmark::CommonMarkCache;

use crate::config::Style;

pub use default::COLUMN_MAX_WIDTH;

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub muted: Color32,
    pub link: Color32,
}

impl Palette {
    pub fn for_style(style: Style) -> Self {
        match style {
            Style::Default => Self {
                bg: default::BG,
                muted: default::MUTED,
                link: default::LINK,
            },
            Style::GlowLatte => Self {
                bg: glow_latte::BG,
                muted: glow_latte::MUTED,
                link: glow_latte::LINK,
            },
        }
    }
}

pub fn setup(ctx: &egui::Context, style: Style) {
    match style {
        Style::Default => default::setup(ctx),
        Style::GlowLatte => glow_latte::setup(ctx),
    }
}

pub fn column_max_width(style: Style) -> f32 {
    match style {
        Style::Default => default::COLUMN_MAX_WIDTH,
        Style::GlowLatte => glow_latte::COLUMN_MAX_WIDTH,
    }
}

pub fn show_markdown(
    ui: &mut egui::Ui,
    cache: &mut CommonMarkCache,
    markdown: &str,
    width: f32,
    style: Style,
) {
    match style {
        Style::Default => {
            default::markdown_viewer()
                .max_image_width(Some(width as usize))
                .show(ui, cache, markdown);
        }
        Style::GlowLatte => {
            crate::glow::show_glow_markdown(ui, cache, markdown, width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fonts::try_add_font;
    use eframe::egui::FontDefinitions;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn try_add_font_returns_none_for_missing_path() {
        let mut fonts = FontDefinitions::default();
        assert!(try_add_font(&mut fonts, "missing", "/no/such/font.ttf").is_none());
    }

    #[test]
    fn try_add_font_loads_bytes_from_existing_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"font-bytes").unwrap();

        let mut fonts = FontDefinitions::default();
        let name = try_add_font(&mut fonts, "test-font", file.path()).unwrap();

        assert_eq!(name, "test-font");
        assert!(fonts.font_data.contains_key("test-font"));
    }
}
