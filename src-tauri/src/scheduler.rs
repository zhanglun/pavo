use chrono::Local;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::{self, runtime::Runtime, sync::mpsc, task, time};

use crate::config;
use crate::services::bing::{self, Images};
use crate::services::AsyncProcessMessage;

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

pub fn test_timer() {
  let rt = Runtime::new().unwrap();
  let _guard = rt.enter();
  task::spawn(async {
    println!("task start ===>");
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
  pub list: Vec<Images>,

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
    // TODO: initialize data according the user's config settings
    let json1 = bing::Wallpaper::new(0, 8).await;
    let json2 = bing::Wallpaper::new(8, 8).await;

    let list = json1.unwrap().json.images;
    let list2 = json2.unwrap().json.images;
    let list = list
      .into_iter()
      .chain(list2.clone().into_iter())
      .collect::<Vec<bing::Images>>();

    self.list = list;
  }

  pub fn push_list(&mut self, image: Images) {
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

      bing::Wallpaper::set_wallpaper(&item.url()).await.unwrap();

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
