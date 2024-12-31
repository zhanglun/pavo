#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cache;
mod cmd;
mod config;
mod scheduler;
mod services;
mod tray;

use cmd::AsyncProcInputTx;
use log;
use services::AsyncProcessMessage;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::{mpsc, Mutex};

fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
  match event {
    tauri::WindowEvent::CloseRequested { api, .. } => {
      let window = window.clone();
      api.prevent_close();
      window.hide().unwrap();
    }
    tauri::WindowEvent::Focused(flag) => {
      println!("window focused! {:?}", flag);

      if !flag {
        let _ = window.hide().unwrap();
      }
    }
    _ => {}
  }
}

#[tokio::main]
async fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  tauri::async_runtime::spawn(async move {
    let mut g_cache = cache::CACHE.lock().await;
    g_cache.update_timestamp_if_need();
  });

  let (async_process_input_tx, mut async_process_input_rx) =
    mpsc::channel::<AsyncProcessMessage>(32);
  let tx = async_process_input_tx.clone();

  let mut scheduler = scheduler::Scheduler::new();

  scheduler.setup_list(None).await;
  scheduler.shuffle_photo().await;

  tauri::async_runtime::spawn(async move {
    loop {
      if let Some(message) = async_process_input_rx.recv().await {
        println!("output: {:?}", message);

        match message {
          AsyncProcessMessage::StartShuffle => {
            println!("init output start 2 {:?}", message);
            scheduler.start_shuffle_photo().await;
          }
          AsyncProcessMessage::StopShuffle => {
            println!("init output stop 2 {:?}", message);
            scheduler.stop_shuffle_photo();
          }
          AsyncProcessMessage::PreviousPhoto => {
            println!("PreviousPhoto {:?}", message);
            scheduler.previous_photo().await;
          }
          AsyncProcessMessage::NextPhoto => {
            println!("NextPhoto {:?}", message);
            scheduler.next_photo().await;
          }
        }
      }
    }
  });

  let mut app = tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_positioner::init())
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
    .plugin(tauri_plugin_positioner::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .manage(AsyncProcInputTx {
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
        update(handle).await.unwrap();
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::view_photo,
      cmd::get_bing_wallpaper_list,
      cmd::get_bing_daily,
      cmd::get_config,
      cmd::set_auto_shuffle,
      cmd::set_interval,
      cmd::set_randomly,
      cmd::set_shuffle_source,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
