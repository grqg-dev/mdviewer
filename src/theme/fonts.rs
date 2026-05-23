use eframe::egui::{FontData, FontDefinitions};

pub(crate) fn try_add_font(
    fonts: &mut FontDefinitions,
    key: &str,
    path: impl AsRef<std::path::Path>,
) -> Option<String> {
    let bytes = std::fs::read(path.as_ref()).ok()?;
    let name = key.to_owned();
    fonts.font_data.insert(name.clone(), FontData::from_owned(bytes).into());
    Some(name)
}
