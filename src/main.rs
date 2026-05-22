use anyhow::Result;
use mdviewer::ViewerApp;

fn main() -> Result<()> {
    eframe::run_native(
        "mdviewer",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_title("mdviewer")
                .with_inner_size([960.0, 720.0])
                .with_min_inner_size([480.0, 320.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            mdviewer::theme::setup(&cc.egui_ctx);
            Ok(Box::new(ViewerApp::new()))
        }),
    )
    .map_err(|err| anyhow::anyhow!("{err}"))
}
