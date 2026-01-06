use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::download_file;
use crate::config;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use once_cell::sync::Lazy;
use std::sync::Arc;

static GLOBAL_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
    Arc::new(Client::new())
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltips {
  pub loading: String,
  pub next: String,
  pub previous: String,
  pub walle: String,
  pub walls: String,
}

#[derive(Serialize, Deserialize)]
pub struct WallpaperMeta {
  pub title: String,
  pub date: String,
  pub copyright: String,
  pub copyrightlink: String,
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
  pub fn get_filename(url: &str) -> &str {
    let s = url.find("OHR.").ok_or(0).unwrap();
    let e = url.find("&rf=").ok_or(0).unwrap();

    &url[s..e]
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperRes {
  pub images: Vec<Images>,
  pub tooltips: Tooltips,
}

impl WallpaperRes {
  pub async fn new(index: u8, number: u8, mkt: Option<String>) -> Result<WallpaperRes> {
    Ok(
      reqwest::get(get_url(index, number, mkt).as_str())
        .await?
        .json::<WallpaperRes>()
        .await?,
    )
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
      json
    })
  }

  pub async fn save_wallpaper(
    url: &str,
    filename: Option<&str>,
  ) -> Result<String> {
    let filename = match filename {
      Some(filename) => filename,
      None => Images::get_filename(url),
    };
    let app_folder = config::PavoConfig::get_app_folder()
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Error code: {}, message: {}", e.0, e.1)))
        })?;
    let path = Path::new(&app_folder).join(&*filename);
    let client = GLOBAL_CLIENT.clone();
    let res = download_file(&client, &url, path.clone().to_str().unwrap()).await;

    println!("{:?}", res);

    return res;
  }

  /// set wallpaper from local file
  // pub async fn set_wallpaper_from_local(a: String) -> String {
  //   wallpaper::set_from_path(a.as_str()).unwrap();

  //   if cfg!(not(target_os = "macos")) {
  //     wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
  //   }

  //   a
  // }

  pub async fn set_wallpaper_from_local(path: &str) -> Result<&str> {
    wallpaper::set_from_path(path).map_err(|e| e.to_string())?;

    if cfg!(not(target_os = "macos")) {
      wallpaper::set_mode(wallpaper::Mode::Crop).map_err(|e| e.to_string())?;
    }

    Ok(path)
  }

  pub async fn set_wallpaper(url: &str) -> Result<String> {
    let file_path = Wallpaper::save_wallpaper(url, None).await?;

    Self::set_wallpaper_from_local(&file_path).await
      .map_err(|e| format!("Failed to set wallpaper from local file: {}", e))?;

    Ok(String::from("Ok"))
  }
}

fn get_url(index: u8, number: u8, mkt: Option<String>) -> String {
  let num = number.to_string();
  let idx = index.to_string();
  let mut url = format!("{}&idx={}&n={}", BING_URL, &idx, &num);

  if let Some(mkt_val) = mkt.clone() {
    let v = mkt_val.clone();
    url.push_str("&mkt=");
    url.push_str(v.clone().as_str());
  }

  println!("url: {:?}", url);

  url
}
