use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "mkd", "txt"];

pub fn cli_path_from(mut args: impl Iterator<Item = String>) -> Option<PathBuf> {
    args.next();
    args.next().map(PathBuf::from)
}

pub fn cli_path() -> Option<PathBuf> {
    cli_path_from(env::args())
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
    fn cli_path_from_uses_first_arg_only() {
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
    fn markdown_extensions_include_common_suffixes() {
        assert!(MARKDOWN_EXTENSIONS.contains(&"md"));
        assert!(MARKDOWN_EXTENSIONS.contains(&"markdown"));
        assert!(MARKDOWN_EXTENSIONS.contains(&"txt"));
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
