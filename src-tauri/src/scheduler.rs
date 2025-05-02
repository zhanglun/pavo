use chrono::offset::Utc;
use chrono::Local;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::{self, sync::Mutex};

use crate::config;
use crate::services::bing;
use crate::services::download_file;

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

const BING_EXPIRE_TIME: i64 = 60 * 60 * 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerPhoto {
  filename: String,
  regions: Vec<String>,
  urls: Vec<String>,
  titles: Vec<String>,
  startdates: Vec<String>,
  copyrights: Vec<String>,
  copyrightlinks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Scheduler {
  pub last_load_time: i64,
  pub cache_list: Vec<SchedulerPhoto>,
  pub current_lang: String,
  pub current_idx: usize,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      last_load_time: Utc::now().timestamp(),
      cache_list: vec![],
      current_lang: String::from("zh-cn"),
      current_idx: 0,
    }
  }

  pub fn should_refresh(&mut self) -> bool {
    let now = Utc::now().timestamp();

    if now - self.last_load_time < BING_EXPIRE_TIME {
      return false;
    } else {
      self.last_load_time = now;
      return true;
    }
  }

  pub async fn batch_fetch(&mut self) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
    if !self.should_refresh() && !self.cache_list.is_empty() {
      return Ok(self.cache_list.clone());
    }

    let region_codes = [
      "zh-CN", "en-US", "fr-FR", "de-DE", "ja-JP", "en-CA", "en-GB", "en-IN", "it-IT",
    ];

    let mut handles = vec![];

    for region_code in region_codes {
      let region = region_code.to_string();
      let mut scheduler = self.clone();
      let handle = tokio::spawn(async move {
        scheduler.fetch_list_with_region(region).await
      });
      handles.push(handle);
    }

    let mut res: Vec<SchedulerPhoto> = vec![];
    for handle in handles {
      if let Ok(Ok(mut photos)) = handle.await {
        res.append(&mut photos);
      }
    }

    let mut formatted_list = vec![];

    for i in res {
      let unique_name = i.filename.clone().split("_").collect::<Vec<_>>()[0].to_string();

      let idx = formatted_list.iter().position(|x: &SchedulerPhoto| x.filename.clone().split("_").collect::<Vec<_>>()[0] == unique_name);

      match idx {
        Some(idx) => {
          let item = &mut formatted_list[idx];

          item.regions.append(&mut i.clone().regions);
          item.urls.append(&mut i.clone().urls);
          item.titles.append(&mut i.clone().titles);
          item.startdates.append(&mut i.clone().startdates);
          item.copyrights.append(&mut i.clone().copyrights);
          item.copyrightlinks.append(&mut i.clone().copyrightlinks);
        }
        None => {
          let item = i.clone();
          formatted_list.push(item);
        }
      }
    }

    self.cache_list = formatted_list.clone();

    Ok(formatted_list)
  }

  pub async fn fetch_list_with_region(&mut self, region: String) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
    let res1 = bing::Wallpaper::new(0, 8, Some(region.clone()))
      .await
      .unwrap();
    let res2 = bing::Wallpaper::new(7, 8, Some(region.clone())).await.unwrap();

    let images1 = res1.json.images;
    let images2 = res2.json.images;

    let mut res: Vec<SchedulerPhoto> = images1
      .into_iter()
      .chain(images2.into_iter())
      .into_iter()
      .map(|i| SchedulerPhoto {
        filename: bing::Images::get_filename(&i.url).to_string(),
        urls: vec![["https://www.bing.com", &i.url].concat()],
        regions: vec![region.clone()],
        titles: vec![i.clone().title],
        startdates: vec![i.clone().startdate],
        copyrights: vec![i.clone().copyright],
        copyrightlinks: vec![i.clone().copyrightlink],
      })
      .collect();

    res.dedup_by(|a, b| a.filename == b.filename);

    Ok(res)
  }

  pub async fn setup_list(&mut self) -> Vec<SchedulerPhoto> {
    let list = self.batch_fetch().await.unwrap();

    list
  }

  pub async fn save_wallpaper(url: &str, filename: &str) -> Result<String, String> {
    let app_folder = config::PavoConfig::get_app_folder().unwrap();
    let path = Path::new(&app_folder).join(&*filename);
    let res = download_file(&Client::new(), &url, path.clone().to_str().unwrap())
      .await
      .unwrap();

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
    let list = self.cache_list.clone();

    if self.current_idx <= 0 {
      self.current_idx = list.len() - 1;
    } else {
      self.current_idx -= 1;
    }

    let item = list[self.current_idx].clone();

    Self::set_wallpaper(&item.urls[0], &item.filename)
      .await
      .unwrap();
  }

  pub async fn next_photo(&mut self) {
    let list = self.cache_list.clone();

    if self.current_idx >= list.len() - 1 {
      self.current_idx = 0;
    } else {
      self.current_idx += 1;
    }

    let item = list[self.current_idx].clone();

    Self::set_wallpaper(&item.urls[0], &item.filename)
      .await
      .unwrap();
  }
}

pub static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));
