#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod scheduler;
mod services;

use cmd::AsyncProcInputTx;
use services::AsyncProcessMessage;
use tauri::{
  AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent,
  SystemTrayMenu, SystemTrayMenuItem, WindowEvent, Wry,
};

fn create_tray() -> SystemTray {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let auto_rotate = CustomMenuItem::new("auto_rotate".to_string(), "Auto Rotate");
    let previous_photo = CustomMenuItem::new("previous_photo".to_string(), "Previous Photo");
    let next_photo = CustomMenuItem::new("next_photo".to_string(), "Next Photo");

  let tray_menu = SystemTrayMenu::new()
    .add_item(auto_rotate)
    .add_item(previous_photo)
    .add_item(next_photo)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(show)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let tray = SystemTray::new().with_menu(tray_menu);

  tray
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::DoubleClick {
      tray_id,
      position,
      size,
      ..
    } => {
      let window = app.get_window("main").unwrap();
      window.show().unwrap();
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
    "previous_photo" => {

            },
    "next_photo" => {

            },
      "show" => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      "hide" => {
        let window = app.get_window("main").unwrap();
        window.hide().unwrap();
      }
      "quit" => {
        std::process::exit(0);
      }
      _ => {}
    },
    _ => {}
  };
}

fn handle_window_event(event: GlobalWindowEvent<Wry>) {
  let window = event.window();
  let app = window.app_handle();

  match event.event() {
    WindowEvent::CloseRequested { api, .. } => {
      let window = window.clone();
      api.prevent_close();
      window.hide().unwrap();
      // ask the user if he wants to quit
      // ask(
      //   Some(&window),
      //   "Tauri API",
      //   "Are you sure that you want to close this window?",
      //   |answer| {
      //     if answer {
      //       // .close() cannot be called on the main thread
      //       std::thread::spawn(move || {
      //         window.close().unwrap();
      //       });
      //     }
      //   },
      // );
    }
    _ => {}
  }
}

use tokio::sync::{mpsc, Mutex};

#[tokio::main]
async fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  let (async_process_input_tx, async_process_input_rx) =
    mpsc::channel::<AsyncProcessMessage>(32);

  tauri::Builder::default()
    .manage(AsyncProcInputTx {
      sender: Mutex::new(async_process_input_tx),
    })
    .setup(|app| {
      // let app_handle = app.handle();

      scheduler::Scheduler::init(async_process_input_rx);

      Ok(())
    })
    .system_tray(create_tray())
    .on_system_tray_event(handle_tray_event)
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::view_photo,
      cmd::get_bing_wallpaper_list,
      cmd::get_bing_daily,
      cmd::get_pexels_curated_photos,
      cmd::get_config,
      cmd::set_auto_rotate,
      cmd::set_interval,
      cmd::set_randomly,
      cmd::set_rotate_source,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
