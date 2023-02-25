use crate::services::{bing, pexels, PhotoService, AsyncProcessMessage};
use crate::{config, services};

use tokio::sync::{mpsc, Mutex};

pub struct AsyncProcInputTx {
  pub sender: Mutex<mpsc::Sender<AsyncProcessMessage>>,
}

#[tauri::command]
pub async fn set_as_desktop(url: &str, service: PhotoService) -> Result<String, String> {
  println!("set as {:?}", url);

  match service {
    PhotoService::Bing => Ok(bing::Wallpaper::set_wallpaper(url).await.unwrap()),
    PhotoService::Pexels => Ok(pexels::Pexels::set_wallpaper(url).await.unwrap()),
    PhotoService::Unsplash => {
      bing::Wallpaper::set_wallpaper(url).await.unwrap();
      Ok(String::from("asdf"))
    }
  }
}

#[tauri::command]
pub async fn download(url: &str, service: PhotoService) -> Result<String, String> {
  match service {
    PhotoService::Bing => Ok(bing::Wallpaper::save_wallpaper(url, None).await.unwrap()),
    PhotoService::Pexels => Ok(pexels::Pexels::save_photo(url).await.unwrap()),
    PhotoService::Unsplash => {
      bing::Wallpaper::set_wallpaper(url).await.unwrap();
      Ok(String::from("asdf"))
    }
  }
}

#[tauri::command]
pub async fn get_bing_wallpaper_list(page: u8) -> Result<bing::Wallpaper, String> {
  println!("page ===> {:?}", page);
  let idx = page * 8;
  let bing = services::bing::Wallpaper::new(idx, 8).await;

  match bing {
    Ok(bing) => Ok(bing),
    Err(_) => Err("".to_string()),
  }
}

#[tauri::command]
pub async fn get_bing_daily() -> Result<bing::Wallpaper, String> {
  let bing = services::bing::Wallpaper::new(0, 1).await;

  match bing {
    Ok(bing) => Ok(bing),
    Err(_) => Err("".to_string()),
  }
}


#[tauri::command]
pub async fn get_pexels_curated_photos(page: u8) -> serde_json::Value {
  let pexels_client =
    pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
  // let res = pexels_client.get_photo_search(20, page).await;
  let res = pexels_client.get_photo_curated(20, page).await;

  res
}

#[tauri::command]
pub async fn get_config() -> serde_json::Value {
  let pavo_config = config::PavoConfig::get_config();

  serde_json::to_value(pavo_config).unwrap()
}

#[tauri::command]
#[allow(unused)]
pub async fn set_auto_rotate(
  rotate: bool,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  println!("auto rotate {:?}", rotate);
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_auto_rotate(rotate);

  let async_proc_input_tx = state.sender.lock().await;

  if rotate {
    async_proc_input_tx
      .send(AsyncProcessMessage::StartRotate)
      .await
      .map_err(|e| e.to_string());
  } else {
    async_proc_input_tx
      .send(AsyncProcessMessage::StopRotate)
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
pub async fn set_rotate_source(source: String, checked: bool) {
  let pavo_config = config::PavoConfig::get_config();

  pavo_config.set_rotate_source(source, checked);
}

#[tauri::command]
pub async fn view_photo(handle: tauri::AppHandle, href: String) {
  services::view_photo(handle, href);
}
