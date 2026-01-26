use std::path::Path;

use crate::scheduler;
use crate::services::{bing, AsyncProcessMessage, PhotoService};
use crate::{config, services};

use showfile;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_desktop_underlay::DesktopUnderlayExt;
use tokio::sync::{mpsc, Mutex};

pub struct AsyncProcInputTx {
  pub sender: Mutex<mpsc::Sender<AsyncProcessMessage>>,
}

#[tauri::command]
pub async fn set_as_desktop<R: Runtime>(
  app: AppHandle<R>,
  url: &str,
  service: PhotoService,
) -> Result<String, String> {
  println!("set as {:?}", url);

  let result = match service {
    PhotoService::Bing => bing::Wallpaper::set_wallpaper(url)
      .await
      .map_err(|e| e.to_string()),
  };

  if result.is_ok() {
    let scheduler = scheduler::SCHEDULER.lock().await;
    let _ = scheduler.emit_wallpaper_event_by_url(&app, url).await;
  }

  result
}

#[tauri::command]
pub async fn download(url: &str, service: PhotoService) -> Result<String, String> {
  match service {
    PhotoService::Bing => bing::Wallpaper::save_wallpaper(url, None)
      .await
      .map_err(|e| e.to_string()),
  }
}

#[tauri::command]
pub async fn get_bing_wallpaper_list() -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let res = scheduler.batch_fetch().await.unwrap();

  res
}

#[tauri::command]
pub async fn get_config() -> serde_json::Value {
  let pavo_config = config::PavoConfig::get_config();

  serde_json::to_value(pavo_config).unwrap()
}

#[tauri::command]
#[allow(unused)]
pub async fn set_auto_shuffle(
  shuffle: bool,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_auto_shuffle(shuffle);

  let async_proc_input_tx = state.sender.lock().await;

  if shuffle {
    async_proc_input_tx
      .send(AsyncProcessMessage::StartShuffle)
      .await
      .map_err(|e| e.to_string());
  } else {
    async_proc_input_tx
      .send(AsyncProcessMessage::StopShuffle)
      .await
      .map_err(|e| e.to_string());
  }

  Ok(())
}

#[tauri::command]
pub async fn set_interval(interval: u64) {
  let pavo_config = config::PavoConfig::get_config();

  println!("{:?}", interval);

  pavo_config.set_interval(interval);
}

#[tauri::command]
pub async fn set_show_layer<R: Runtime>(app_handler: AppHandle<R>, show_layer: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_show_layer(show_layer);

  if show_layer {
    print!("show layer");
    // app_handler.get_webview_window("underlayer").unwrap().show().unwrap();
    // app_handler.get_webview_window("underlayer").unwrap().set_desktop_underlay(true).unwrap();
    app_handler
      .get_webview_window("underlayer")
      .unwrap()
      .toggle_desktop_underlay()
      .unwrap();
    app_handler
      .get_webview_window("main")
      .unwrap()
      .set_focus()
      .unwrap();
  } else {
    // app_handler.get_webview_window("underlayer").unwrap().hide().unwrap();
    // app_handler.get_webview_window("underlayer").unwrap().set_desktop_underlay(false).unwrap();
    app_handler
      .get_webview_window("underlayer")
      .unwrap()
      .toggle_desktop_underlay()
      .unwrap();
    app_handler
      .get_webview_window("main")
      .unwrap()
      .set_focus()
      .unwrap();
  }
}

#[tauri::command]
pub async fn reveal_log_file() {
  let folder_dir = config::PavoConfig::get_app_folder().unwrap();
  let file_path = Path::new(&folder_dir).join("logs/Pavo.log");

  showfile::show_path_in_file_manager(file_path.to_path_buf());
}

#[tauri::command]
pub async fn set_randomly(randomly: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_randomly(randomly);
}

#[tauri::command]
pub async fn set_auto_save(auto_save: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_auto_save(auto_save);
}

#[tauri::command]
pub async fn view_photo(handle: tauri::AppHandle, href: String) {
  services::view_photo(handle, href);
}
