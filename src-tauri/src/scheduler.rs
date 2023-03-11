use chrono::Local;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::{self, runtime::Runtime, sync::mpsc, task, time};

use crate::services::bing::{self, Images};
use crate::services::AsyncProcessMessage;
use crate::{cache, config};

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerPhoto {
  url: String,
  title: String,
}

pub fn test_timer() {
  let rt = Runtime::new().unwrap();
  let _guard = rt.enter();

  task::spawn(async {
    println!("task start ==>");
    time::sleep(time::Duration::from_secs(5)).await;
    println!("task over: {}", now());
  });

  thread::spawn(|| loop {
    thread::sleep(time::Duration::from_secs(10));
    println!("thread spawn");
  });
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scheduler {
  pub interval: u64,
  pub auto_rotate: bool,
  pub randomly: bool,
  pub list: Vec<SchedulerPhoto>,

  pub rotating: bool,
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
    }
  }

  pub async fn setup_list(&mut self) {
    let user_config = config::PavoConfig::get_config();
    println!("user_config: {:?}", user_config);
    let mut cache = cache::CACHE.lock().await;
    let bing_list = cache.get_pexels_list().await;
    let list = bing_list.into_iter().map(|i| SchedulerPhoto {
      url: i.url,
      title: i.alt,
    });

    if user_config.rotate_source.contains(&String::from("pexels")) {
      let pexels_list = cache.get_bing_list().await;
      self.list = list.chain(pexels_list.into_iter().map(|p| SchedulerPhoto {
        url: p.url,
        title: p.title,
      })).collect();
    } else {
      self.list = list.collect();
    }
  }

  pub fn push_list(&mut self, image: SchedulerPhoto) {
    self.list.push(image);
  }

  pub async fn rotate_photo(&mut self) {
    if self.rotating == false {
      ()
    }

    let mut list = self.list.clone();
    let cache = list.clone();

    let rotate_interval = config::PavoConfig::get_interval();
    let mut interval = time::interval(time::Duration::from_secs(rotate_interval * 60));

    let mut cfg = config::PavoConfig::get_config();

    while list.len() > 0 && cfg.auto_rotate {
      interval.tick().await;

      cfg = config::PavoConfig::get_config();

      let mut item = list[0].clone();

      if cfg.randomly {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, list.len());

        item = list[idx].clone();
        list.remove(idx);
      } else {
        item = list.pop().unwrap();
      }

      println!("{:?}", item.title);

      bing::Wallpaper::set_wallpaper(&item.url).await.unwrap();

      if list.len() == 0 {
        list = cache.clone();
      }
    }
  }

  pub async fn start_rotate_photo(&mut self) {
    self.rotating = true;
    self.rotate_photo().await;
  }

  pub fn stop_rotate_photo(&mut self) {
    self.rotating = false
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
