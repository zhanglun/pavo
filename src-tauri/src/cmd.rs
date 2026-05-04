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
    PhotoService::Bing => {
      let filename = bing::Images::get_filename(url).map_err(|e| e.to_string())?;
      let downloads_dir = dirs::download_dir()
        .ok_or("无法获取下载目录".to_string())?
        .join("Pavo");
      std::fs::create_dir_all(&downloads_dir).map_err(|e| e.to_string())?;
      let dest = downloads_dir.join(&filename);
      let dest_str = dest.to_string_lossy().to_string();

      let app_folder = config::PavoConfig::get_app_folder().map_err(|e| e.to_string())?;
      let cache_path = Path::new(&app_folder).join(&filename);

      // 如果缓存中有，直接复制；否则先下载到缓存再复制
      if cache_path.exists() {
        std::fs::copy(&cache_path, &dest).map_err(|e| e.to_string())?;
        Ok(dest_str)
      } else {
        let _cached = bing::Wallpaper::save_wallpaper(url, None)
          .await
          .map_err(|e| e.to_string())?;
        std::fs::copy(&cache_path, &dest).map_err(|e| e.to_string())?;
        Ok(dest_str)
      }
    }
  }
}

#[tauri::command]
pub async fn get_bing_wallpaper_list() -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  if scheduler.is_cache_valid() {
    return scheduler.cache_list.clone();
  }
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
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = if scheduler.is_cache_valid() {
    scheduler.cache_list.clone()
  } else {
    scheduler.batch_fetch().await.ok()?
  };
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::pick_today(&list, &today)
}

#[tauri::command]
pub async fn get_recent_wallpapers(days: u8) -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = if scheduler.is_cache_valid() {
    scheduler.cache_list.clone()
  } else {
    scheduler.batch_fetch().await.unwrap_or_default()
  };
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::filter_recent_days(&list, days, &today)
}

#[tauri::command]
pub async fn get_today_collection() -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = if scheduler.is_cache_valid() {
    scheduler.cache_list.clone()
  } else {
    scheduler.batch_fetch().await.unwrap_or_default()
  };
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

#[tauri::command]
pub async fn set_auto_start(enabled: bool, app: tauri::AppHandle) -> Result<(), String> {
  use tauri_plugin_autostart::ManagerExt;
  let manager = app.autolaunch();
  if enabled {
    manager.enable().map_err(|e| e.to_string())?;
  } else {
    manager.disable().map_err(|e| e.to_string())?;
  }
  config::PavoConfig::get_config().set_auto_start(enabled);
  Ok(())
}

#[tauri::command]
pub async fn set_cache_retention_days(days: u32) {
  config::PavoConfig::get_config().set_cache_retention_days(days);
}
