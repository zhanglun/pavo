use crate::{services};
use crate::services::{bing, pexels, PhotoService};

#[tauri::command]
pub async fn set_as_desktop(url: &str, service: PhotoService) -> Result<String, String> {
  println!("set as {:?}", url);

  match service {
    PhotoService::Bing => {
      Ok(bing::Wallpaper::set_wallpaper(url).await.unwrap())
    }
    PhotoService::Pexels => {
      Ok(pexels::Pexels::set_wallpaper(url).await.unwrap())
    }
    PhotoService::Unsplash => {
      bing::Wallpaper::set_wallpaper(url);
      Ok(String::from("asdf"))
    }
  }
}

#[tauri::command]
pub async fn download(url: &str, service: PhotoService) -> Result<String, String> {
  match service {
    PhotoService::Bing => {
      Ok(bing::Wallpaper::save_wallpaper(url).await.unwrap())
    }
    PhotoService::Pexels => {
      Ok(pexels::Pexels::save_photo(url).await.unwrap())
    }
    PhotoService::Unsplash => {
      bing::Wallpaper::set_wallpaper(url);
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
    Ok(bing) => {
      Ok(bing)
    }
    Err(_) => {
      Err("".to_string())
    }
  }
}

#[tauri::command]
pub async fn get_pexels_curated_photos(page: u8) -> serde_json::Value {
  let pexels_client = pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
  let res = pexels_client.get_photo_curated(20, page).await;

  res
}
