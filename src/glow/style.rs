use eframe::egui::{self, FontId, RichText, Ui};

use crate::theme::{FONT_SIZE, GlowPalette};

#[derive(Default, Clone)]
pub struct GlowTextStyle {
    pub strong: bool,
    pub emphasis: bool,
    pub strikethrough: bool,
    pub quote: bool,
    pub code: bool,
    pub link: bool,
}

impl GlowTextStyle {
    pub fn to_richtext(&self, text: &str, palette: GlowPalette) -> RichText {
        let mut rt = RichText::new(text).color(palette.text);

        if self.quote {
            rt = rt.italics().color(palette.text);
        }

        if self.strong {
            rt = rt.strong();
        }

        if self.emphasis {
            rt = rt.italics();
        }

        if self.strikethrough {
            rt = rt.strikethrough();
        }

        if self.code {
            rt = rt
                .font(FontId::monospace(FONT_SIZE))
                .color(palette.red)
                .background_color(palette.surface0);
        }

        if self.link {
            rt = rt.color(palette.blue).strong().underline();
        }

        rt
    }
}

pub fn render_hr(ui: &mut Ui, palette: GlowPalette) {
    ui.label(
        RichText::new("────────")
            .color(palette.overlay0)
            .size(FONT_SIZE),
    );
    egui_commonmark_backend::elements::newline(ui);
}

pub fn render_h1(ui: &mut Ui, text: &str, palette: GlowPalette) {
    egui::Frame::new()
        .fill(palette.lavender)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text.trim())
                    .color(palette.white)
                    .strong()
                    .size(FONT_SIZE),
            );
        });
    egui_commonmark_backend::elements::newline(ui);
}

pub fn render_heading(ui: &mut Ui, level: u8, text: &str, palette: GlowPalette) {
    let prefix = format!("{} ", "#".repeat(level as usize));
    ui.horizontal(|ui| {
        ui.label(RichText::new(prefix).color(palette.blue).size(FONT_SIZE));
        let mut rt = RichText::new(text.trim())
            .color(palette.blue)
            .size(FONT_SIZE);
        if level < 6 {
            rt = rt.strong();
        }
        ui.label(rt);
    });
    egui_commonmark_backend::elements::newline(ui);
}
