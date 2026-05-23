use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "mkd", "txt"];

pub struct LaunchOptions {
    pub path: Option<PathBuf>,
    pub style: Option<String>,
}

pub fn parse_launch_options_from(mut args: impl Iterator<Item = String>) -> LaunchOptions {
    args.next(); // program name
    let mut path = None;
    let mut style = None;

    while let Some(arg) = args.next() {
        if arg == "--style" || arg == "-s" {
            style = args.next();
        } else if !arg.starts_with('-') {
            path = Some(PathBuf::from(arg));
            break;
        }
    }

    LaunchOptions { path, style }
}

pub fn launch_options() -> LaunchOptions {
    parse_launch_options_from(env::args())
}

pub fn cli_path_from(args: impl Iterator<Item = String>) -> Option<PathBuf> {
    parse_launch_options_from(args).path
}

pub fn cli_path() -> Option<PathBuf> {
    launch_options().path
}

pub fn title_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mdviewer")
        .to_owned()
}

pub fn pick_markdown_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Markdown", MARKDOWN_EXTENSIONS)
        .set_title("Open Markdown")
        .pick_file()
}

pub fn read_markdown(path: &Path) -> Result<(String, String)> {
    let markdown = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    Ok((markdown, title_from_path(path)))
}

pub fn scroll_after_page_down(scroll_offset: f32, viewport_height: f32) -> f32 {
    scroll_offset + viewport_height * 0.9
}

pub fn scroll_after_page_up(scroll_offset: f32, viewport_height: f32) -> f32 {
    (scroll_offset - viewport_height * 0.9).max(0.0)
}

pub fn empty_state_prompt(drag_hover: bool) -> &'static str {
    if drag_hover {
        "Drop to open"
    } else {
        "Drop a .md file here"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn cli_path_from_no_args_returns_none() {
        assert_eq!(cli_path_from(["mdviewer".to_owned()].into_iter()), None);
    }

    #[test]
    fn cli_path_from_one_arg_returns_path() {
        assert_eq!(
            cli_path_from(["mdviewer".to_owned(), "notes.md".to_owned()].into_iter()),
            Some(PathBuf::from("notes.md"))
        );
    }

    #[test]
    fn cli_path_from_style_flag_before_path() {
        let opts = parse_launch_options_from(
            [
                "mdviewer".to_owned(),
                "--style".to_owned(),
                "glow-latte".to_owned(),
                "notes.md".to_owned(),
            ]
            .into_iter(),
        );
        assert_eq!(opts.style.as_deref(), Some("glow-latte"));
        assert_eq!(opts.path.as_deref(), Some(Path::new("notes.md")));
    }

    #[test]
    fn cli_path_from_uses_first_non_flag_arg() {
        assert_eq!(
            cli_path_from(
                [
                    "mdviewer".to_owned(),
                    "first.md".to_owned(),
                    "second.md".to_owned(),
                ]
                .into_iter()
            ),
            Some(PathBuf::from("first.md"))
        );
    }

    #[test]
    fn title_from_path_uses_file_name() {
        assert_eq!(
            title_from_path(Path::new("/tmp/docs/readme.md")),
            "readme.md"
        );
    }

    #[test]
    fn title_from_path_falls_back_for_empty_name() {
        assert_eq!(title_from_path(Path::new("")), "mdviewer");
    }

    #[test]
    fn scroll_after_page_down_advances_by_ninety_percent() {
        assert_eq!(scroll_after_page_down(100.0, 500.0), 550.0);
    }

    #[test]
    fn scroll_after_page_up_retreats_by_ninety_percent() {
        assert_eq!(scroll_after_page_up(550.0, 500.0), 100.0);
    }

    #[test]
    fn scroll_after_page_up_never_goes_below_zero() {
        assert_eq!(scroll_after_page_up(50.0, 500.0), 0.0);
    }

    #[test]
    fn empty_state_prompt_reflects_drag_hover() {
        assert_eq!(empty_state_prompt(false), "Drop a .md file here");
        assert_eq!(empty_state_prompt(true), "Drop to open");
    }

    #[test]
    fn read_markdown_loads_file_content() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# Hello").unwrap();

        let (markdown, title) = read_markdown(file.path()).unwrap();
        assert_eq!(markdown, "# Hello\n");
        assert_eq!(title, file.path().file_name().unwrap().to_str().unwrap());
    }

    #[test]
    fn read_markdown_errors_for_missing_path() {
        let err = read_markdown(Path::new("/no/such/file.md")).unwrap_err();
        assert!(err.to_string().contains("failed to read"));
        assert!(err.to_string().contains("file.md"));
    }
}
