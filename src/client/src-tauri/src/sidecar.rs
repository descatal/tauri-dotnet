use log::{debug, info, warn};
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

pub(crate) async fn spawn(app_handle: &AppHandle) {
    info!("Spawning sidecar process");
    let command = match app_handle.shell().sidecar("service") {
        Ok(cmd) => cmd,
        Err(e) => {
            warn!("Could not get sidecar executable: {e}");
            return; // No-op if sidecar not found
        }
    };

    let (mut rx, mut _child) = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            warn!(
                "Failed to spawn sidecar: {e}. \
Failed to spawn sidecar, the files in the binaries folder are stubs that are there for building purposes. \
If you are currently in debugging mode please run your own instance of the backend service"
            );
            return; // No-op if spawn fails
        }
    };

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    debug!("{}", String::from_utf8_lossy(&bytes).trim());
                }
                _ => {}
            }
        }
    });
}
