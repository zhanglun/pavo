#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod background;
mod cmd;
mod config;
mod daily_update_thread;
mod desktop_layer;
mod events;
mod plugins;
mod rotation_thread;
mod scheduler;
mod services;
mod tray;

use cmd::AsyncProcInputTx;
use plugins::register_plugins;
use services::AsyncProcessMessage;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;
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

      let sender = tx.clone();
      let _ = tray::create_tray(app, sender);

      use pavo::update;

      let handle = app.handle().clone();

      tauri::async_runtime::spawn(async move {
        println!("background start");
        background::Background::new(Arc::new(Mutex::new(async_process_input_rx)), handle.clone())
          .await;

        update(handle).await.unwrap();
      });

      let app = app.app_handle();

      let clock = app.get_webview_window("underlayer").unwrap();

      let app_clone = app.clone();
      let clock_clone = clock.clone();
      tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

        if let Some(monitor) = app_clone.primary_monitor().ok().flatten() {
          let scale_factor = monitor.scale_factor();
          let screen_size = monitor.size();
          let window_size = clock_clone.outer_size().unwrap();
          let margin_right = 40;
          let margin_bottom = 40;

          let x = screen_size.width as i32 - window_size.width as i32 - margin_right;
          let y = screen_size.height as i32 - window_size.height as i32 - margin_bottom;

          println!(
            "Screen size: {:?}, scale factor: {}",
            screen_size, scale_factor
          );
          println!("Window size: {:?}", window_size);
          println!("Calculated physical position: x={}, y={}", x, y);

          let _ =
            clock_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));

          println!("After set_position");

          tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

          let _ = clock_clone.show();

          println!("After show");

          tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

          println!("Setting position again after show...");

          let _ =
            clock_clone.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));

          println!("After second set_position");
        } else {
          println!("Failed to get primary monitor");
        }
      });

      let cfg = config::PavoConfig::get_config();

      if cfg.show_layer {
        clock.set_desktop_underlay(true)?;
        clock.show()?;
      } else {
        clock.set_desktop_underlay(false)?;
        clock.hide()?;
      }

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
      cmd::set_show_layer,
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
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
