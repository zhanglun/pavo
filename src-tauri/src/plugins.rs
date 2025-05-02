use crate::config;
use tauri::{Builder, Manager, Runtime};

pub fn register_plugins<R: Runtime>(builder: Builder<R>) -> Builder<R> {
  builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
      let _ = app
        .get_webview_window("main")
        .expect("no main window")
        .set_focus();
    }))
    .plugin(
      tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
          tauri_plugin_log::TargetKind::Folder {
            path: std::path::PathBuf::from(config::PavoConfig::get_app_folder().unwrap() + "/logs"),
            file_name: None,
          },
        ))
        .level(log::LevelFilter::Info)
        .build(),
    )
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
      let _ = app
        .get_webview_window("main")
        .expect("no main window")
        .set_focus();
    }))
}
