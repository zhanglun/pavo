use std::path::Path;

use crate::config::RotateMode;
use crate::scheduler;
use crate::services::{bing, AsyncProcessMessage, PhotoService};
use crate::{config, services};

use tauri::{AppHandle, Runtime};
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
  {
    let scheduler = scheduler::SCHEDULER.lock().await;
    if scheduler.is_cache_valid() {
      return scheduler.cache_list.clone();
    }
  }
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  scheduler.batch_fetch().await.unwrap_or_default()
}

#[tauri::command]
pub async fn get_config() -> serde_json::Value {
  let pavo_config = config::PavoConfig::get_config();

  serde_json::to_value(pavo_config).unwrap_or_default()
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
pub async fn reveal_log_file() {
  match config::PavoConfig::get_app_folder() {
    Ok(folder_dir) => {
      let file_path = Path::new(&folder_dir).join("logs/Pavo.log");
      showfile::show_path_in_file_manager(&file_path);
    }
    Err(e) => log::error!("failed to get app folder: {:?}", e),
  }
}

#[tauri::command]
pub async fn view_photo(handle: tauri::AppHandle, href: String) {
  services::view_photo(handle, href);
}

#[tauri::command]
pub async fn get_today_wallpaper() -> Option<scheduler::SchedulerPhoto> {
  {
    let scheduler = scheduler::SCHEDULER.lock().await;
    if scheduler.is_cache_valid() {
      let today = chrono::Local::now().format("%Y%m%d").to_string();
      return scheduler::Scheduler::pick_today(&scheduler.cache_list, &today);
    }
  }
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.ok()?;
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::pick_today(&list, &today)
}

#[tauri::command]
pub async fn get_recent_wallpapers(days: u8) -> Vec<scheduler::SchedulerPhoto> {
  {
    let scheduler = scheduler::SCHEDULER.lock().await;
    if scheduler.is_cache_valid() {
      let today = chrono::Local::now().format("%Y%m%d").to_string();
      return scheduler::Scheduler::filter_recent_days(&scheduler.cache_list, days, &today);
    }
  }
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.unwrap_or_default();
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::filter_recent_days(&list, days, &today)
}

#[tauri::command]
pub async fn get_today_collection() -> Vec<scheduler::SchedulerPhoto> {
  {
    let scheduler = scheduler::SCHEDULER.lock().await;
    if scheduler.is_cache_valid() {
      let today = chrono::Local::now().format("%Y%m%d").to_string();
      return scheduler::Scheduler::filter_today(&scheduler.cache_list, &today);
    }
  }
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.unwrap_or_default();
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::filter_today(&list, &today)
}

#[tauri::command]
pub async fn list_favorites() -> Vec<config::FavoriteItem> {
  config::PavoConfig::get_config().favorites
}

#[tauri::command]
pub async fn add_favorite(item: config::FavoriteItem) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().add_favorite(item)).unwrap_or_default()
}

#[tauri::command]
pub async fn remove_favorite(filename: String) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().remove_favorite_by_filename(&filename))
    .unwrap_or_default()
}

#[tauri::command]
pub async fn set_auto_rotate(
  enabled: bool,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  config::PavoConfig::get_config().set_auto_rotate(enabled);

  let sender = state.sender.lock().await;
  sender
    .send(if enabled {
      AsyncProcessMessage::StartRotation
    } else {
      AsyncProcessMessage::StopRotation
    })
    .await
    .map_err(|_| ())
}

#[tauri::command]
pub async fn set_rotate_interval(
  minutes: u16,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  let cfg = config::PavoConfig::get_config().set_rotate_interval(minutes);

  let sender = state.sender.lock().await;
  sender
    .send(AsyncProcessMessage::UpdateRotationConfig {
      interval_minutes: cfg.rotate_interval_minutes,
      mode: cfg.rotate_mode,
    })
    .await
    .map_err(|_| ())
}

#[tauri::command]
pub async fn set_rotate_mode(
  mode: RotateMode,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  let cfg = config::PavoConfig::get_config().set_rotate_mode(mode);

  let sender = state.sender.lock().await;
  sender
    .send(AsyncProcessMessage::UpdateRotationConfig {
      interval_minutes: cfg.rotate_interval_minutes,
      mode: cfg.rotate_mode,
    })
    .await
    .map_err(|_| ())
}
