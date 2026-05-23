use anyhow::Result;
use mdviewer::ViewerApp;

fn main() -> Result<()> {
    let cli_path = mdviewer::files::cli_path();

    if mdviewer::ipc::deliver_to_running_instance(cli_path.as_deref())? {
        return Ok(());
    }

    let ipc_rx = match mdviewer::ipc::spawn_listener() {
        Ok(rx) => rx,
        Err(_) => {
            // Race condition: another instance bound the socket between our delivery
            // attempt and our bind attempt. Give it a moment then retry delivery.
            std::thread::sleep(std::time::Duration::from_millis(200));
            if mdviewer::ipc::deliver_to_running_instance(cli_path.as_deref())? {
                return Ok(());
            }
            anyhow::bail!("failed to start IPC listener");
        }
    };

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
            mdviewer::theme::setup(&cc.egui_ctx);
            cc.egui_ctx.set_embed_viewports(false);
            Ok(Box::new(ViewerApp::new(cli_path, ipc_rx)))
        }),
    )
    .map_err(|err| anyhow::anyhow!("{err}"))
}
