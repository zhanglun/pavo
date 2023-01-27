#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod services;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

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
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
