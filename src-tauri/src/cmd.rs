use std::path::Path;

use crate::scheduler;
use crate::services::{bing, AsyncProcessMessage, PhotoService};
use crate::{config, services};

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
pub async fn set_auto_daily_update(
  enabled: bool,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  let pavo_config = config::PavoConfig::get_config();
  pavo_config.set_auto_daily_update(enabled);

  let sender = state.sender.lock().await;
  sender
    .send(if enabled {
      AsyncProcessMessage::StartDailyUpdate
    } else {
      AsyncProcessMessage::StopDailyUpdate
    })
    .await
    .map_err(|_| ())
}

#[tauri::command]
pub async fn set_history_range_days(days: u8) {
  let pavo_config = config::PavoConfig::get_config();
  pavo_config.set_history_range_days(days);
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

  showfile::show_path_in_file_manager(&file_path);
}

#[tauri::command]
pub async fn view_photo(handle: tauri::AppHandle, href: String) {
  services::view_photo(handle, href);
}

#[tauri::command]
pub async fn get_today_wallpaper() -> Option<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.ok()?;
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::pick_today(&list, &today)
}

#[tauri::command]
pub async fn get_recent_wallpapers(days: u8) -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.unwrap_or_default();
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::filter_recent_days(&list, days, &today)
}

#[tauri::command]
pub async fn list_favorites() -> Vec<config::FavoriteItem> {
  config::PavoConfig::get_config().favorites
}

#[tauri::command]
pub async fn add_favorite(item: config::FavoriteItem) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().add_favorite(item)).unwrap()
}

#[tauri::command]
pub async fn remove_favorite(filename: String) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().remove_favorite_by_filename(&filename))
    .unwrap()
}
