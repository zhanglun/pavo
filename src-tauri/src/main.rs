#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod background;
mod cmd;
mod config;
mod scheduler;
mod plugins;
mod services;
mod shuffle_thread;
mod tray;

use cmd::AsyncProcInputTx;
use plugins::register_plugins;
use services::AsyncProcessMessage;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
  match event {
    tauri::WindowEvent::CloseRequested { api, .. } => {
      let window = window.clone();
      api.prevent_close();
      window.hide().unwrap();
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

  builder.manage(AsyncProcInputTx {
      sender: Mutex::new(async_process_input_tx),
    })
    .setup(move |app| {
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      let sender = tx.clone();
      let _ = tray::create_tray(app, sender);

      use pavo::update;

      let handle = app.handle().clone();

      tauri::async_runtime::spawn(async move {
        println!("background start");
        background::Background::new(Arc::new(Mutex::new(async_process_input_rx))).await;

        update(handle).await.unwrap();
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::view_photo,
      cmd::get_bing_wallpaper_list,
      cmd::get_config,
      cmd::set_auto_shuffle,
      cmd::set_interval,
      cmd::set_randomly,
      cmd::set_auto_save,
      cmd::reveal_log_file,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
