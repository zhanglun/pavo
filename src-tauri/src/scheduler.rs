use chrono::offset::Utc;
use chrono::Local;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::{self, sync::Mutex };

use crate::services::bing;
use crate::services::download_file;
use crate::config;

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

const BING_EXPIRE_TIME: i64 = 60 * 60 * 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerPhoto {
  #[serde(flatten)]
  pub images: bing::Images,
  url: String,
  filename: String,
}

#[derive(Debug, Clone)]
pub struct Scheduler {
  pub last_load_time: i64,
  pub cache_list: HashMap<String, Vec<SchedulerPhoto>>,
  pub current_lang: String,
  pub current_idx: usize,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      last_load_time: Utc::now().timestamp(),
      cache_list: HashMap::new(),
      current_lang: String::from("zh-cn"),
      current_idx: 0,
    }
  }

  pub async fn get_list_from_remote(&mut self, country: Option<String>) -> Vec<SchedulerPhoto> {
    let now = Utc::now().timestamp();
    let mut lang = self.current_lang.clone();

    if let Some(country) = country.clone() {
      lang = country;
    }

    let mut list = vec![];

    if let Some(l) = self.cache_list.get(&lang) {
      list = l.clone();
    }

    if list.len() > 0 && now - self.last_load_time < BING_EXPIRE_TIME {
      return list.clone();
    }

    let res1 = bing::Wallpaper::new(0, 8, country.clone()).await.unwrap();
    let res2 = bing::Wallpaper::new(7, 8, country).await.unwrap();

    let images1 = res1.json.images;
    let images2 = res2.json.images;

    let mut res: Vec<SchedulerPhoto> = images1
      .into_iter()
      .chain(images2.into_iter())
      .into_iter()
      .map(|i| SchedulerPhoto {
        images: i.clone(),
        url: ["https://www.bing.com", &i.url].concat(),
        filename: bing::Images::get_filename(&i.url).to_string(),
      })
      .collect();

    res.dedup_by(|a, b| a.url == b.url);

    self.last_load_time = Utc::now().timestamp();

    println!("timestamp: {:?}", self.last_load_time);

    if self.cache_list.get(&lang).is_none() {
      self.cache_list.insert(lang.to_string(), res.clone());
    }

    res.clone()
  }

  pub async fn setup_list(&mut self, country: Option<String>) -> Vec<SchedulerPhoto> {
    let list = self.get_list_from_remote(country).await;

    list
  }

  pub async fn get_bing_daily(&mut self, country: Option<String>) -> SchedulerPhoto {
    let bing = bing::Wallpaper::new(0, 1, country).await.unwrap();
    let image = bing.json.images[0].clone();

    SchedulerPhoto {
      images: image.clone(),
      url: image.url(),
      filename: bing::Images::get_filename(&image.url).to_string(),
    }
  }

  pub async fn save_wallpaper(url: &str, filename: &str) -> Result<String, String> {
    let app_folder = config::PavoConfig::get_app_folder().unwrap();
    let path = Path::new(&app_folder).join(&*filename);
    let res = download_file(&Client::new(), &url, path.clone().to_str().unwrap())
      .await
      .unwrap();

    // 71行经常报错alled `Result::unwrap()` on an `Err` value: "Failed to GET from 'https://www.bing.com/th?id=OHR.SnowySvaneti_JA-JP2274619860_UHD.jpg&rf=LaDigue_UHD.jpg&pid=hp&w=3840&h=2160&rs=1&c=4'"
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    println!("{:?}", res);

    Ok(res)
  }

  pub async fn set_wallpaper_from_local(a: String) -> String {
    wallpaper::set_from_path(a.as_str()).unwrap();

    if cfg!(not(target_os = "macos")) {
      wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    }

    a
  }

  pub async fn set_wallpaper(url: &str, filename: &str) -> Result<String, String> {
    let a = Self::save_wallpaper(url, filename).await;

    match a {
      Ok(a) => {
        Self::set_wallpaper_from_local(a).await;

        Ok(String::from("OK"))
      }
      Err(e) => Err(e.to_string().into()),
    }
  }

  pub async fn previous_photo(&mut self) {
    let mut list = vec![];

    if let Some(l) = self.cache_list.get(&self.current_lang) {
      list = l.clone();
    }

    if self.current_idx <= 0 {
      self.current_idx = list.len() - 1;
    } else {
      self.current_idx -= 1;
    }

    let item = list[self.current_idx].clone();

    Self::set_wallpaper(&item.url, &item.filename)
      .await
      .unwrap();
  }

  pub async fn next_photo(&mut self) {
    let mut list = vec![];

    if let Some(l) = self.cache_list.get(&self.current_lang) {
      list = l.clone();
    }

    if self.current_idx >= list.len() - 1 {
      self.current_idx = 0;
    } else {
      self.current_idx += 1;
    }

    let item = list[self.current_idx].clone();

    Self::set_wallpaper(&item.url, &item.filename)
      .await
      .unwrap();
  }
}

pub static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));
