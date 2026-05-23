use anyhow::Result;
use mdviewer::ViewerApp;

fn main() -> Result<()> {
    let options = mdviewer::files::launch_options();

    if mdviewer::ipc::deliver_to_running_instance(options.path.as_deref())? {
        return Ok(());
    }

    let ipc_rx = match mdviewer::ipc::spawn_listener() {
        Ok(rx) => rx,
        Err(_) => {
            std::thread::sleep(std::time::Duration::from_millis(200));
            if mdviewer::ipc::deliver_to_running_instance(options.path.as_deref())? {
                return Ok(());
            }
            anyhow::bail!("failed to start IPC listener");
        }
    };

    let style = mdviewer::config::resolve_style(options.style.as_deref());

    eframe::run_native(
        "mdviewer",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_title("mdviewer")
                .with_inner_size([960.0, 720.0])
                .with_min_inner_size([480.0, 320.0]),
            ..Default::default()
        },
        Box::new(move |cc| {
            mdviewer::theme::setup(&cc.egui_ctx, style);
            cc.egui_ctx.set_embed_viewports(false);
            Ok(Box::new(ViewerApp::new(options.path, ipc_rx, style)))
        }),
    )
    .map_err(|err| anyhow::anyhow!("{err}"))
}
