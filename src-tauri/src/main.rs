#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod background;
mod cmd;
mod config;
mod daily_update_thread;
mod events;
mod plugins;
mod rotation_thread;
mod scheduler;
mod services;
mod tray;

use cmd::AsyncProcInputTx;
use plugins::register_plugins;
use services::bing;
use services::AsyncProcessMessage;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::{mpsc, Mutex};

fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
  match event {
    tauri::WindowEvent::CloseRequested { api, .. } => {
      let window = window.clone();
      api.prevent_close();
      window.hide().unwrap();
    }
    tauri::WindowEvent::Destroyed => {
      println!("window destroyed");
      if window.label() == "main" {
        let app_handle = window.app_handle();

        for (label, win) in app_handle.webview_windows() {
          if label != "main" {
            win.close().unwrap();
          }
        }
      }
    }
    _ => {}
  }
}

#[tokio::main]
async fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  let (async_process_input_tx, async_process_input_rx) = mpsc::channel::<AsyncProcessMessage>(32);
  let tx = async_process_input_tx.clone();

  let builder = tauri::Builder::default();
  let builder = register_plugins(builder);

  builder
    .manage(AsyncProcInputTx {
      sender: Mutex::new(async_process_input_tx),
    })
    .setup(move |app| {
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      // 非自启动时显示主窗口（tauri.conf.json 默认 visible: false 防止闪烁）
      if !std::env::args().any(|arg| arg == "--autostart") {
        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
        }
      }

      let sender = tx.clone();

      {
        let retention_days = config::PavoConfig::get_config().cache_retention_days;
        let _ = bing::clean_cache(retention_days);
      }

      let _ = tray::create_tray(app, sender);

      use pavo::update;

      let handle = app.handle().clone();

      tauri::async_runtime::spawn(async move {
        println!("background start");
        background::Background::new(Arc::new(Mutex::new(async_process_input_rx)), handle.clone())
          .await;

        if let Err(e) = update(handle).await {
          log::error!("update check failed: {}", e);
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::view_photo,
      cmd::get_bing_wallpaper_list,
      cmd::get_config,
      cmd::set_auto_daily_update,
      cmd::set_history_range_days,
      cmd::reveal_log_file,
      cmd::get_today_wallpaper,
      cmd::get_recent_wallpapers,
      cmd::get_today_collection,
      cmd::list_favorites,
      cmd::add_favorite,
      cmd::remove_favorite,
      cmd::set_auto_rotate,
      cmd::set_rotate_interval,
      cmd::set_rotate_mode,
      cmd::set_auto_start,
      cmd::set_cache_retention_days,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
