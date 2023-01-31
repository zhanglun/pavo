#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod services;

use tauri::{
  Manager, CustomMenuItem,
  SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent,
  AppHandle, GlobalWindowEvent, WindowEvent, Wry
};

fn create_tray () -> SystemTray {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let tray = SystemTray::new().with_menu(tray_menu);

  tray
}

fn handle_tray_event (app: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::DoubleClick { tray_id, position, size, .. } => {
      let window = app.get_window("main").unwrap();
      window.show().unwrap();
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
      match id.as_str() {
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
      }
    }
    _ => {}
  };
}

fn handle_window_event (event: GlobalWindowEvent<Wry>) {
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

fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  tauri::Builder::default()
    .system_tray(create_tray())
    .on_system_tray_event(handle_tray_event)
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::get_bing_wallpaper_list,
      cmd::get_pexels_curated_photos,
      cmd::get_config,
      cmd::set_auto_rotate,
      cmd::set_randomly,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
