use chrono::Local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::{self, sync::mpsc, time};

use crate::services::bing::Images;
use crate::services::{download_file, AsyncProcessMessage};
use crate::{cache, config};

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerPhoto {
  url: String,
  title: String,
  filename: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scheduler {
  pub interval: u64,
  pub auto_shuffle: bool,
  pub randomly: bool,
  pub list: Vec<SchedulerPhoto>,

  pub rotating: bool,
  pub current_idx: usize,
}

impl Scheduler {
  pub fn new() -> Self {
    let cfg = config::PavoConfig::get_config();

    Self {
      interval: cfg.interval,
      auto_shuffle: cfg.auto_shuffle,
      randomly: cfg.randomly,
      list: vec![],
      rotating: false,
      current_idx: 0,
    }
  }

  pub async fn setup_list(&mut self) {
    let user_config = config::PavoConfig::get_config();
    let mut cache = cache::CACHE.lock().await;
    let bing_list = cache.get_bing_list().await;
    let list = bing_list.into_iter().map(|p| SchedulerPhoto {
      url: p.url.clone(),
      title: p.title,
      filename: Images::get_filename(&p.url).to_string(),
    });

    println!("user_config.shuffle_source {:?}", user_config.shuffle_source);


    self.list = list.collect();

    println!("self.list.len {:?}", self.list.len());

    // FIXME: update cache templatly
    cache.update_cache_list(self.list.clone());
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

  pub async fn shuffle_photo(&mut self) {
    if self.rotating == false {
      ()
    }

    tauri::async_runtime::spawn(async move {
      let shuffle_interval = config::PavoConfig::get_interval();
      let mut interval = time::interval(time::Duration::from_secs(shuffle_interval * 60));

      loop {
        print!("WAITTING!\n");

        interval.tick().await;

        let mut cfg = config::PavoConfig::get_config();
        let mut cache = cache::CACHE.lock().await;

        if cache.cache_list.len() > 0 && cfg.auto_shuffle {
          cfg = config::PavoConfig::get_config();

          if cfg.randomly {
            let item = cache.get_random_photo();
            println!("CHANGE TO {:?} \n", &item);

            Self::set_wallpaper(&item.url, &item.filename)
              .await
              .unwrap();
          } else {
            let item = cache.shuffle_to_next();
            println!("CHANGE TO {:?} \n", &item);

            Self::set_wallpaper(&item.url, &item.filename)
              .await
              .unwrap();
          }
        }
      }
    });
  }

  pub async fn start_shuffle_photo(&mut self) {
    self.rotating = true;
    // self.shuffle_photo().await;
  }

  pub fn stop_shuffle_photo(&mut self) {
    self.rotating = false
  }

  pub async fn previous_photo(&mut self) {
    let mut cache = cache::CACHE.lock().await;
    let item = cache.shuffle_to_previous();
    println!("CHANGE TO {:?} \n", &item);

    Self::set_wallpaper(&item.url, &item.filename)
      .await
      .unwrap();
  }

  pub async fn next_photo(&mut self) {
    let mut cache = cache::CACHE.lock().await;
    let item = cache.shuffle_to_next();

    println!("CHANGE TO {:?} \n", &item);

    Self::set_wallpaper(&item.url, &item.filename)
      .await
      .unwrap();
  }
}
