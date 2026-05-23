use eframe::egui::{self, FontDefinitions, FontFamily, Stroke, Vec2, Visuals};

use super::catppuccin::GlowPalette;

pub const COLUMN_MAX_WIDTH: f32 = 820.0;
pub const FONT_SIZE: f32 = 14.0;

pub fn setup(ctx: &egui::Context, palette: GlowPalette) {
    setup_fonts(ctx);
    setup_visuals(ctx, palette);
}

fn ghostty_font_family() -> Option<String> {
    let home = std::env::var_os("HOME")?;
    let config = std::path::PathBuf::from(home)
        .join("Library/Application Support/com.mitchellh.ghostty/config");
    let content = std::fs::read_to_string(config).ok()?;
    for line in content.lines() {
        let line = line.split('#').next()?.trim();
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "font-family" {
                return Some(value.trim().to_string());
            }
        }
    }
    None
}

fn font_paths_for_family(family: &str) -> Vec<std::path::PathBuf> {
    let home = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_default();
    let fonts = home.join("Library/Fonts");

    match family.to_ascii_lowercase().as_str() {
        "fira code" => vec![
            fonts.join("FiraCode-Retina.ttf"),
            fonts.join("FiraCode-Regular.ttf"),
            fonts.join("FiraCode-Medium.ttf"),
            fonts.join("FiraCode-Bold.ttf"),
        ],
        "jetbrains mono" => vec![
            fonts.join("JetBrainsMono-Regular.ttf"),
            fonts.join("JetBrainsMono-Medium.ttf"),
        ],
        "sf mono" | "sfmono" => vec![
            std::path::PathBuf::from("/System/Library/Fonts/SFNSMono.ttf"),
            std::path::PathBuf::from("/System/Library/Fonts/SFMono.ttf"),
        ],
        _ => vec![fonts.join(format!("{family}.ttf"))],
    }
}

fn fallback_font_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if let Some(family) = ghostty_font_family() {
        paths.extend(font_paths_for_family(&family));
    }
    paths.extend(font_paths_for_family("Fira Code"));
    paths.extend(font_paths_for_family("JetBrains Mono"));
    paths.extend(font_paths_for_family("SF Mono"));
    paths
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let mut loaded_name = None;
    for (i, path) in fallback_font_paths().into_iter().enumerate() {
        if let Some(name) = super::fonts::try_add_font(&mut fonts, &format!("terminal-{i}"), &path)
        {
            loaded_name = Some(name);
            break;
        }
    }

    if let Some(name) = loaded_name {
        fonts
            .families
            .insert(FontFamily::Monospace, vec![name.clone()]);
        fonts.families.insert(FontFamily::Proportional, vec![name]);
    }

    ctx.set_fonts(fonts);
}

fn setup_visuals(ctx: &egui::Context, palette: GlowPalette) {
    let mut visuals = if palette.is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    visuals.window_fill = palette.base;
    visuals.panel_fill = palette.base;
    visuals.extreme_bg_color = palette.surface0;
    visuals.faint_bg_color = palette.surface0;
    visuals.code_bg_color = palette.surface0;
    visuals.hyperlink_color = palette.blue;
    visuals.override_text_color = Some(palette.text);
    visuals.weak_text_color = Some(palette.overlay0);
    visuals.weak_text_alpha = 1.0;

    let text_stroke = Stroke::new(1.0, palette.text);
    visuals.widgets.noninteractive.fg_stroke = text_stroke;
    visuals.widgets.inactive.fg_stroke = text_stroke;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, palette.text);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, palette.text);
    visuals.widgets.open.fg_stroke = text_stroke;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, palette.border());

    ctx.set_visuals(visuals);

    let mono = |size| egui::FontId::new(size, FontFamily::Monospace);
    let mut style = (*ctx.global_style()).clone();
    style.spacing.item_spacing = Vec2::new(0.0, 8.0);
    style.spacing.indent = 16.0;
    style.spacing.button_padding = Vec2::new(6.0, 3.0);
    style.text_styles.insert(egui::TextStyle::Body, mono(FONT_SIZE));
    style.text_styles.insert(egui::TextStyle::Heading, mono(FONT_SIZE));
    style.text_styles.insert(egui::TextStyle::Monospace, mono(FONT_SIZE));
    style.text_styles.insert(egui::TextStyle::Small, mono(FONT_SIZE - 1.0));
    ctx.set_global_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ghostty_font_family_parses_config() {
        let family = ghostty_font_family();
        assert_eq!(family.as_deref(), Some("Fira Code"));
    }
}
