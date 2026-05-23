use eframe::egui::{self, FontId, RichText, Ui};

use crate::theme::glow_latte as palette;

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
    pub fn to_richtext(&self, text: &str) -> RichText {
        let mut rt = RichText::new(text).color(palette::TEXT);

        if self.quote {
            rt = rt.italics().color(palette::TEXT);
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
                .font(FontId::monospace(palette::FONT_SIZE))
                .color(palette::RED)
                .background_color(palette::SURFACE0);
        }

        if self.link {
            rt = rt.color(palette::BLUE).strong().underline();
        }

        rt
    }
}

pub fn render_hr(ui: &mut Ui) {
    ui.label(
        RichText::new("────────")
            .color(palette::OVERLAY0)
            .size(palette::FONT_SIZE),
    );
    egui_commonmark_backend::elements::newline(ui);
}

pub fn render_h1(ui: &mut Ui, text: &str) {
    egui::Frame::new()
        .fill(palette::LAVENDER)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text.trim())
                    .color(palette::WHITE)
                    .strong()
                    .size(palette::FONT_SIZE),
            );
        });
    egui_commonmark_backend::elements::newline(ui);
}

pub fn render_heading(ui: &mut Ui, level: u8, text: &str) {
    let prefix = format!("{} ", "#".repeat(level as usize));
    ui.horizontal(|ui| {
        ui.label(RichText::new(prefix).color(palette::BLUE).size(palette::FONT_SIZE));
        let mut rt = RichText::new(text.trim())
            .color(palette::BLUE)
            .size(palette::FONT_SIZE);
        if level < 6 {
            rt = rt.strong();
        }
        ui.label(rt);
    });
    egui_commonmark_backend::elements::newline(ui);
}
