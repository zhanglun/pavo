use chrono::Local;
use rand::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{path::Path, thread};
use tokio::{self, runtime::Runtime, sync::mpsc, task, time};

use crate::services::bing::{Images};
use crate::services::pexels::Pexels;
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
  pub auto_rotate: bool,
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
      auto_rotate: cfg.auto_rotate,
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

    if user_config.rotate_source.contains(&String::from("pexels")) {
      let pexels_list = cache.get_pexels_list().await;
      self.list = list
        .chain(pexels_list.into_iter().map(|p| SchedulerPhoto {
          url: p.src.original.clone(),
          title: p.alt,
          filename: Pexels::get_filename(&p.src.original).to_string(),
        }))
        .collect();
    } else {
      self.list = list.collect();
    }
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

  pub async fn update_current_photo(&mut self) {
    let list = self.list.clone();
    let idx = self.current_idx;
    let item = &list[idx];

      Self::set_wallpaper(&item.url, &item.filename)
        .await
        .unwrap();
  }

  pub async fn rotate_photo(&mut self) {
    if self.rotating == false {
      ()
    }

    let list = self.list.clone();
    let rotate_interval = config::PavoConfig::get_interval();
    let mut interval = time::interval(time::Duration::from_secs(rotate_interval * 60));
    let mut cfg = config::PavoConfig::get_config();

    while list.len() > 0 && cfg.auto_rotate {
      interval.tick().await;

      cfg = config::PavoConfig::get_config();

      if cfg.randomly {
        let mut rng = rand::thread_rng();
        self.current_idx = rng.gen_range(0, list.len());
      } else {
        self.current_idx += 1;
      }

      self.update_current_photo().await;
    }
  }

  pub async fn start_rotate_photo(&mut self) {
    self.rotating = true;
    self.rotate_photo().await;
  }

  pub fn stop_rotate_photo(&mut self) {
    self.rotating = false
  }

  pub async fn previous_photo(&mut self) {
  }

  pub async fn next_photo(&mut self) {
  }

  pub async fn create_interval() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    task::spawn(async {
      time::sleep(time::Duration::from_secs(3)).await;
      println!("task over: {}", now());
    });

    thread::sleep(time::Duration::from_secs(4));
  }

  pub fn init(mut rx: mpsc::Receiver<AsyncProcessMessage>) {
    tokio::spawn(async move {
      let mut scheduler = Scheduler::new();

      scheduler.setup_list().await;
      scheduler.rotate_photo().await;

      loop {
        if let Some(output) = rx.recv().await {
          match output {
            AsyncProcessMessage::StartRotate => {
              scheduler.start_rotate_photo().await;
              println!("init output start 2 {:?}", output);
            }
            AsyncProcessMessage::StopRotate => {
              scheduler.stop_rotate_photo();
              println!("init output stop 2 {:?}", output);
            }
          }
        }
      }
    });
  }
}
