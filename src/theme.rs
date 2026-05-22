use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Stroke, Vec2, Visuals};
use egui_commonmark::CommonMarkViewer;

// GitHub / Cursor markdown preview palette
pub const BG: Color32 = Color32::from_rgb(255, 255, 255);
pub const TEXT: Color32 = Color32::from_rgb(31, 35, 40);
pub const MUTED: Color32 = Color32::from_rgb(101, 109, 118);
pub const LINK: Color32 = Color32::from_rgb(9, 105, 218);
pub const CODE_BG: Color32 = Color32::from_rgb(246, 248, 250);
pub const INLINE_CODE_BG: Color32 = Color32::from_rgb(239, 241, 243);
pub const BORDER: Color32 = Color32::from_rgb(208, 215, 222);

pub const COLUMN_MAX_WIDTH: f32 = 820.0;

pub fn setup(ctx: &egui::Context) {
    setup_fonts(ctx);
    setup_visuals(ctx);
}

pub fn markdown_viewer() -> CommonMarkViewer<'static> {
    CommonMarkViewer::new()
        .syntax_theme_light("InspiredGitHub")
        .indentation_spaces(2)
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    #[cfg(target_os = "macos")]
    {
        if let Some(name) = try_add_font(&mut fonts, "text", "/System/Library/Fonts/SFNS.ttf") {
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, name);
        }

        let mono_paths = [
            "/System/Library/Fonts/SFNSMono.ttf",
            "/System/Library/Fonts/SFMono.ttf",
        ];
        for path in mono_paths {
            if let Some(name) = try_add_font(&mut fonts, "mono", path) {
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, name);
                break;
            }
        }

        if let Some(home) = std::env::var_os("HOME") {
            let fira = std::path::PathBuf::from(home).join("Library/Fonts/FiraCode-Retina.ttf");
            if let Some(name) = try_add_font(&mut fonts, "fira-code", &fira) {
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, name);
            }
        }
    }

    ctx.set_fonts(fonts);
}

fn try_add_font(fonts: &mut FontDefinitions, key: &str, path: impl AsRef<std::path::Path>) -> Option<String> {
    let bytes = std::fs::read(path.as_ref()).ok()?;
    let name = key.to_owned();
    fonts.font_data.insert(name.clone(), FontData::from_owned(bytes).into());
    Some(name)
}

fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    visuals.window_fill = BG;
    visuals.panel_fill = BG;
    visuals.extreme_bg_color = CODE_BG;
    visuals.faint_bg_color = INLINE_CODE_BG;
    visuals.code_bg_color = INLINE_CODE_BG;
    visuals.hyperlink_color = LINK;
    visuals.override_text_color = Some(TEXT);
    visuals.weak_text_color = Some(MUTED);
    visuals.weak_text_alpha = 1.0;

    let text_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.noninteractive.fg_stroke = text_stroke;
    visuals.widgets.inactive.fg_stroke = text_stroke;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, TEXT);
    visuals.widgets.open.fg_stroke = text_stroke;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);

    ctx.set_visuals(visuals);

    let mut style = (*ctx.global_style()).clone();
    style.spacing.item_spacing = Vec2::new(0.0, 10.0);
    style.spacing.indent = 16.0;
    style.spacing.button_padding = Vec2::new(6.0, 3.0);
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(16.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(32.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(14.0, FontFamily::Monospace),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(13.0, FontFamily::Monospace),
    );
    ctx.set_global_style(style);
}
