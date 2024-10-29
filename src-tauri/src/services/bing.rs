use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::download_file;
use crate::config;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlParams {
  pub index: u8,
  pub number: u8,
  pub mkt: Option<String>,
  pub hdr: Option<String>,
}

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

#[warn(dead_code)]
impl Images {
  pub fn url(&self) -> String {
    ["https://www.bing.com", &self.url].concat()
  }

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

const BING_URL: &str = "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160";

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
    let filename = match filename {
      Some(filename) => filename,
      None => Images::get_filename(url),
    };
    let app_folder = config::PavoConfig::get_app_folder().unwrap();
    let path = Path::new(&app_folder).join(&*filename);
    let res = download_file(&Client::new(), &url, path.clone().to_str().unwrap())
      .await
      .unwrap();

    println!("{:?}", res);

    Ok(res)
  }

  /// set wallpaper from local file
  pub async fn set_wallpaper_from_local(a: String) -> String {
    wallpaper::set_from_path(a.as_str()).unwrap();

    if cfg!(not(target_os = "macos")) {
      wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    }

    a
  }

  pub async fn set_wallpaper(url: &str) -> Result<String> {
    let a = Wallpaper::save_wallpaper(url, None).await;
    match a {
      Ok(a) => {
        Self::set_wallpaper_from_local(a).await;

        Ok(String::from("OK"))
      }
      Err(e) => Err(e.to_string().into()),
    }
  }
}

fn get_url(index: u8, number: u8, mkt: Option<String>) -> String {
  let num = number.to_string();
  let idx = index.to_string();
  let mut url_parts = vec![BING_URL, "&idx=", &idx, "&n=", &num];
  let mut mkt_str = String::new();

  if let Some(mkt_val) = mkt {
    mkt_str = format!("&mkt={}", mkt_val);
    url_parts.push(&mkt_str);
  }

  let url = url_parts.concat();

  println!("url: {:?}", url);

  url
}
