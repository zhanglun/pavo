use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::download_file;
use crate::config;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::sync::Arc;
use std::sync::LazyLock;

static GLOBAL_CLIENT: LazyLock<Arc<Client>> = LazyLock::new(|| Arc::new(Client::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltips {
  pub loading: String,
  pub next: String,
  pub previous: String,
  pub walle: String,
  pub walls: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Images {
  pub bot: usize,
  pub copyright: String,
  pub copyrightlink: String,
  pub drk: usize,
  pub enddate: String,
  pub fullstartdate: String,
  pub hs: Vec<String>,
  pub hsh: String,
  pub quiz: String,
  pub startdate: String,
  pub title: String,
  pub top: usize,
  pub url: String,
  pub urlbase: String,
  pub wp: bool,
}

impl Images {
  pub fn get_filename(url: &str) -> Result<String> {
    let s = url
      .find("OHR.")
      .ok_or_else(|| format!("url missing OHR segment: {url}"))?;
    let e = url
      .find("&rf=")
      .ok_or_else(|| format!("url missing &rf= segment: {url}"))?;

    Ok(url[s..e].to_string())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperRes {
  pub images: Vec<Images>,
  pub tooltips: Tooltips,
}

impl WallpaperRes {
  pub async fn new(index: u8, number: u8, mkt: Option<String>) -> Result<WallpaperRes> {
    let client = GLOBAL_CLIENT.clone();
    let url = get_url(index, number, mkt);
    let res = client.get(url).send().await?;

    Ok(res.json::<WallpaperRes>().await?)
  }
}

const BING_URL: &str =
  "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160";

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallpaper {
  index: u8,
  number: u8,
  files: Vec<String>,
  pub json: WallpaperRes,
}

impl Wallpaper {
  pub async fn new(index: u8, number: u8, mkt: Option<String>) -> Result<Wallpaper> {
    let json = WallpaperRes::new(index, number, mkt).await?;
    Ok(Wallpaper {
      index,
      number,
      files: vec![],
      json,
    })
  }

  pub async fn save_wallpaper(url: &str, filename: Option<&str>) -> Result<String> {
    let filename_owned;
    let filename = match filename {
      Some(filename) => filename,
      None => {
        filename_owned = Images::get_filename(url)?;
        filename_owned.as_str()
      }
    };
    let app_folder = config::PavoConfig::get_app_folder().map_err(
      |e| -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(
          std::io::ErrorKind::Other,
          e,
        ))
      },
    )?;
    let path = Path::new(&app_folder).join(filename);
    // 缓存检查：文件已存在且非空则跳过下载
    if path.exists() && path.metadata().map_or(false, |m| m.len() > 0) {
      log::info!("Wallpaper already cached: {}", filename);
      return Ok(path.to_string_lossy().to_string());
    }
    let client = GLOBAL_CLIENT.clone();
    let path_str = path.to_string_lossy().to_string();
    let res = download_file(&client, url, &path_str).await;

    log::debug!("save_wallpaper result: {:?}", res);

    res
  }

  pub async fn set_wallpaper_from_local(path: &str) -> Result<String> {
    let path_owned = path.to_string();
    tokio::task::spawn_blocking(move || {
      wallpaper::set_from_path(&path_owned).map_err(|e| e.to_string())?;

      if cfg!(not(target_os = "macos")) {
        wallpaper::set_mode(wallpaper::Mode::Crop).map_err(|e| e.to_string())?;
      }

      Ok(path_owned)
    })
    .await
    .map_err(|e| format!("Failed to join wallpaper task: {e}"))?
  }

  pub async fn set_wallpaper(url: &str) -> Result<String> {
    let file_path = Wallpaper::save_wallpaper(url, None).await?;

    Self::set_wallpaper_from_local(&file_path)
      .await
      .map_err(|e| format!("Failed to set wallpaper from local file: {}", e))?;

    Ok(String::from("Ok"))
  }
}

fn get_url(index: u8, number: u8, mkt: Option<String>) -> String {
  let num = number.to_string();
  let idx = index.to_string();
  let mut url = format!("{}&idx={}&n={}", BING_URL, idx, num);

  if let Some(mkt_val) = mkt {
    url.push_str("&mkt=");
    url.push_str(mkt_val.as_str());
  }

  log::debug!("url: {:?}", url);

  url
}

/// 清理超过保留天数的壁纸缓存文件
pub fn clean_cache(retention_days: u32) -> Result<()> {
  let app_folder = config::PavoConfig::get_app_folder().map_err(
    |e| -> Box<dyn std::error::Error + Send + Sync> {
      Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        e,
      ))
    },
  )?;

  let entries = std::fs::read_dir(&app_folder)?;
  let now = std::time::SystemTime::now();
  let retention = std::time::Duration::from_secs(retention_days as u64 * 24 * 60 * 60);

  for entry in entries {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() {
      continue;
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !matches!(ext, "jpg" | "jpeg" | "png" | "bmp" | "webp") {
      continue;
    }

    if let Ok(metadata) = path.metadata() {
      if let Ok(modified) = metadata.modified() {
        if let Ok(age) = now.duration_since(modified) {
          if age > retention {
            match std::fs::remove_file(&path) {
              Ok(_) => log::info!("Cleaned cache: {:?}", path),
              Err(e) => log::warn!("Failed to clean cache {:?}: {}", path, e),
            }
          }
        }
      }
    }
  }

  Ok(())
}
