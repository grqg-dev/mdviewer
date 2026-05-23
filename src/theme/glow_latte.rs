use eframe::egui::{self, Color32, FontDefinitions, FontFamily, Stroke, Vec2, Visuals};

pub const BASE: Color32 = Color32::from_rgb(0xef, 0xf1, 0xf5);
pub const TEXT: Color32 = Color32::from_rgb(0x4c, 0x4f, 0x69);
pub const OVERLAY0: Color32 = Color32::from_rgb(0x9c, 0xa0, 0xb0);
pub const SURFACE0: Color32 = Color32::from_rgb(0xdc, 0xe0, 0xe8);
pub const BLUE: Color32 = Color32::from_rgb(0x1e, 0x66, 0xf5);
pub const LAVENDER: Color32 = Color32::from_rgb(0x72, 0x87, 0xfd);
pub const RED: Color32 = Color32::from_rgb(0xd2, 0x0f, 0x39);
pub const WHITE: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);

pub const BG: Color32 = BASE;
pub const MUTED: Color32 = OVERLAY0;
pub const LINK: Color32 = BLUE;
pub const BORDER: Color32 = OVERLAY0;

pub const COLUMN_MAX_WIDTH: f32 = 820.0;
pub const FONT_SIZE: f32 = 14.0;

pub fn setup(ctx: &egui::Context) {
    setup_fonts(ctx);
    setup_visuals(ctx);
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

fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    visuals.window_fill = BASE;
    visuals.panel_fill = BASE;
    visuals.extreme_bg_color = SURFACE0;
    visuals.faint_bg_color = SURFACE0;
    visuals.code_bg_color = SURFACE0;
    visuals.hyperlink_color = BLUE;
    visuals.override_text_color = Some(TEXT);
    visuals.weak_text_color = Some(OVERLAY0);
    visuals.weak_text_alpha = 1.0;

    let text_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.noninteractive.fg_stroke = text_stroke;
    visuals.widgets.inactive.fg_stroke = text_stroke;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, TEXT);
    visuals.widgets.open.fg_stroke = text_stroke;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);

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
