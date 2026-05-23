use std::env;
use std::fs;
use std::path::PathBuf;

use crate::theme::Flavor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Style {
    #[default]
    Default,
    Glow(Flavor),
}

impl Style {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "default" | "main" => Some(Self::Default),
            _ => Flavor::parse(raw).map(Self::Glow),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Glow(flavor) => flavor.as_str(),
        }
    }

    pub fn is_glow(self) -> bool {
        matches!(self, Self::Glow(_))
    }

    pub fn flavor(self) -> Option<Flavor> {
        match self {
            Self::Glow(flavor) => Some(flavor),
            Self::Default => None,
        }
    }
}

pub fn config_path() -> PathBuf {
    if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(dir).join("mdviewer").join("config.toml");
    }
    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".config").join("mdviewer").join("config.toml")
}

pub fn load_style_from_file() -> Option<Style> {
    let content = fs::read_to_string(config_path()).ok()?;
    parse_style_from_toml(&content)
}

fn parse_style_from_toml(content: &str) -> Option<Style> {
    for line in content.lines() {
        let line = line.split('#').next()?.trim();
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "style" {
                let value = value.trim().trim_matches('"').trim_matches('\'');
                return Style::parse(value);
            }
        }
    }
    None
}

/// CLI `--style` > `MDVIEWER_STYLE` env > config file > `default`.
pub fn resolve_style(cli_override: Option<&str>) -> Style {
    if let Some(raw) = cli_override {
        if let Some(style) = Style::parse(raw) {
            return style;
        }
    }
    if let Ok(raw) = env::var("MDVIEWER_STYLE") {
        if let Some(style) = Style::parse(&raw) {
            return style;
        }
    }
    load_style_from_file().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Flavor;

    #[test]
    fn parses_style_names() {
        assert_eq!(Style::parse("default"), Some(Style::Default));
        assert_eq!(Style::parse("main"), Some(Style::Default));
        assert_eq!(Style::parse("glow-latte"), Some(Style::Glow(Flavor::Latte)));
        assert_eq!(Style::parse("glow"), Some(Style::Glow(Flavor::Latte)));
        assert_eq!(Style::parse("glow-frappe"), Some(Style::Glow(Flavor::Frappe)));
        assert_eq!(Style::parse("glow-macchiato"), Some(Style::Glow(Flavor::Macchiato)));
        assert_eq!(Style::parse("glow-mocha"), Some(Style::Glow(Flavor::Mocha)));
        assert!(Style::parse("unknown").is_none());
    }

    #[test]
    fn parses_config_toml() {
        let toml = r#"
# comment
style = "glow-latte"
"#;
        assert_eq!(parse_style_from_toml(toml), Some(Style::Glow(Flavor::Latte)));
    }

    #[test]
    fn cli_override_wins() {
        assert_eq!(
            resolve_style(Some("glow-latte")),
            Style::Glow(Flavor::Latte)
        );
    }
}
