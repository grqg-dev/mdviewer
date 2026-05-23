use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Flavor {
    #[default]
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl Flavor {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "glow-latte" | "glow_latte" | "latte" | "glow" => Some(Self::Latte),
            "glow-frappe" | "glow_frappe" | "frappe" => Some(Self::Frappe),
            "glow-macchiato" | "glow_macchiato" | "macchiato" => Some(Self::Macchiato),
            "glow-mocha" | "glow_mocha" | "mocha" => Some(Self::Mocha),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Latte => "glow-latte",
            Self::Frappe => "glow-frappe",
            Self::Macchiato => "glow-macchiato",
            Self::Mocha => "glow-mocha",
        }
    }

    pub fn palette(self) -> GlowPalette {
        match self {
            Self::Latte => GlowPalette {
                base: rgb(0xef, 0xf1, 0xf5),
                text: rgb(0x4c, 0x4f, 0x69),
                overlay0: rgb(0x9c, 0xa0, 0xb0),
                surface0: rgb(0xdc, 0xe0, 0xe8),
                blue: rgb(0x1e, 0x66, 0xf5),
                lavender: rgb(0x72, 0x87, 0xfd),
                red: rgb(0xd2, 0x0f, 0x39),
                white: rgb(0xff, 0xff, 0xff),
                is_dark: false,
            },
            Self::Frappe => GlowPalette {
                base: rgb(0x30, 0x34, 0x46),
                text: rgb(0xc6, 0xd0, 0xf5),
                overlay0: rgb(0x73, 0x79, 0x94),
                surface0: rgb(0x41, 0x45, 0x59),
                blue: rgb(0x8c, 0xaa, 0xee),
                lavender: rgb(0xba, 0xbb, 0xf1),
                red: rgb(0xe7, 0x82, 0x84),
                white: rgb(0xff, 0xff, 0xff),
                is_dark: true,
            },
            Self::Macchiato => GlowPalette {
                base: rgb(0x24, 0x27, 0x3a),
                text: rgb(0xca, 0xd3, 0xf5),
                overlay0: rgb(0x6e, 0x73, 0x8d),
                surface0: rgb(0x36, 0x3a, 0x4f),
                blue: rgb(0x8a, 0xad, 0xf4),
                lavender: rgb(0xb7, 0xbd, 0xf8),
                red: rgb(0xed, 0x87, 0x96),
                white: rgb(0xff, 0xff, 0xff),
                is_dark: true,
            },
            Self::Mocha => GlowPalette {
                base: rgb(0x1e, 0x1e, 0x2e),
                text: rgb(0xcd, 0xd6, 0xf4),
                overlay0: rgb(0x6c, 0x70, 0x86),
                surface0: rgb(0x31, 0x32, 0x44),
                blue: rgb(0x89, 0xb4, 0xfa),
                lavender: rgb(0xb4, 0xbe, 0xfe),
                red: rgb(0xf3, 0x8b, 0xa8),
                white: rgb(0xff, 0xff, 0xff),
                is_dark: true,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct GlowPalette {
    pub base: Color32,
    pub text: Color32,
    pub overlay0: Color32,
    pub surface0: Color32,
    pub blue: Color32,
    pub lavender: Color32,
    pub red: Color32,
    pub white: Color32,
    pub is_dark: bool,
}

impl GlowPalette {
    pub fn bg(self) -> Color32 {
        self.base
    }

    pub fn muted(self) -> Color32 {
        self.overlay0
    }

    pub fn link(self) -> Color32 {
        self.blue
    }

    pub fn border(self) -> Color32 {
        self.overlay0
    }
}

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_flavors() {
        assert_eq!(Flavor::parse("glow-latte"), Some(Flavor::Latte));
        assert_eq!(Flavor::parse("glow"), Some(Flavor::Latte));
        assert_eq!(Flavor::parse("latte"), Some(Flavor::Latte));
        assert_eq!(Flavor::parse("glow-frappe"), Some(Flavor::Frappe));
        assert_eq!(Flavor::parse("frappe"), Some(Flavor::Frappe));
        assert_eq!(Flavor::parse("glow-macchiato"), Some(Flavor::Macchiato));
        assert_eq!(Flavor::parse("macchiato"), Some(Flavor::Macchiato));
        assert_eq!(Flavor::parse("glow-mocha"), Some(Flavor::Mocha));
        assert_eq!(Flavor::parse("mocha"), Some(Flavor::Mocha));
        assert!(Flavor::parse("unknown").is_none());
    }

    #[test]
    fn latte_is_light_others_are_dark() {
        assert!(!Flavor::Latte.palette().is_dark);
        assert!(Flavor::Frappe.palette().is_dark);
        assert!(Flavor::Macchiato.palette().is_dark);
        assert!(Flavor::Mocha.palette().is_dark);
    }
}
