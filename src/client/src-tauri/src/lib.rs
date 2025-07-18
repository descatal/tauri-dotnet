use tauri::{generate_handler, plugin::TauriPlugin, Runtime};

mod sidecar;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init());

    // Setup
    builder = builder.setup(|app| {
        if cfg!(debug_assertions) {
            app.handle().plugin(build_log_plugin())?;
        }

        let app_handle_copy = app.handle().clone();

        tauri::async_runtime::spawn(async move {
            sidecar::initialize_service(app_handle_copy);
        });

        Ok(())
    });

    builder = builder.invoke_handler(generate_handler![
        // add commands to be invoked in frontend code
        // start_listening,
        // stop_listening,
        // get_listener
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Debug)
        .max_file_size(50_000 /* bytes */)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("logs".to_string()),
            },
        ))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Webview,
        ))
        .build()
}
