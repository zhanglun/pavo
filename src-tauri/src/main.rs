#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod services;

use tauri::{api::dialog::ask, CustomMenuItem, GlobalWindowEvent, Manager, MenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, WindowEvent, Wry}
;

fn create_tray () -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  let tray = SystemTray::new().with_menu(tray_menu);

  tray
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
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::get_bing_wallpaper_list,
      cmd::get_pexels_curated_photos,
    ])
    .on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
