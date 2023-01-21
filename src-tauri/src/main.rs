#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod services;

fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::get_bing_wallpaper_list,
      cmd::get_pexels_curated_photos,
    ])
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
