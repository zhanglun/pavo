use std::sync::Arc;

use crate::services::{bing, AsyncProcessMessage, PhotoService};
use crate::{cache, config, services};

use tokio::sync::{mpsc, Mutex};

pub struct AsyncProcInputTx {
  pub sender: Mutex<mpsc::Sender<AsyncProcessMessage>>,
}

#[tauri::command]
pub async fn set_as_desktop(url: &str, service: PhotoService) -> Result<String, String> {
  println!("set as {:?}", url);

  match service {
    PhotoService::Bing => Ok(bing::Wallpaper::set_wallpaper(url).await.unwrap()),
  }
}

#[tauri::command]
pub async fn download(url: &str, service: PhotoService) -> Result<String, String> {
  match service {
    PhotoService::Bing => Ok(bing::Wallpaper::save_wallpaper(url, None).await.unwrap()),
  }
}

#[tauri::command]
pub async fn get_bing_wallpaper_list(_page: u8) -> Vec<bing::Images> {
  let mut cache = cache::CACHE.lock().await;
  let res = cache.get_bing_list().await;

  res
}

#[tauri::command]
pub async fn get_bing_daily() -> bing::Images {
  let mut bing_daily = cache::CACHE.lock().await;
  let res = bing_daily.get_bing_daily().await;

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
pub async fn set_randomly(randomly: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_randomly(randomly);
}

#[tauri::command]
pub async fn set_shuffle_source(source: String, checked: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_shuffle_source(source, checked);
}

#[tauri::command]
pub async fn view_photo(handle: tauri::AppHandle, href: String) {
  services::view_photo(handle, href);
}
