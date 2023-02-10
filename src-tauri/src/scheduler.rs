use chrono::Local;
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::{self, runtime::Runtime, task, time};
use tokio::sync::{mpsc, Mutex};

use crate::services::AsyncProcessMessage;
use crate::services::bing::{self, Images};
use crate::config;

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
}

impl Scheduler {
  pub fn new() -> Self {
    let cfg = config::PavoConfig::get_config();

    Self {
      interval: cfg.interval,
      auto_rotate: cfg.auto_rotate,
      randomly: cfg.randomly,
      list: vec![],
    }
  }

  pub async fn setup_list(&mut self) {
    let json1 = bing::Wallpaper::new(0, 8).await;
    let json2 = bing::Wallpaper::new(8, 8).await;

    let list = json1.unwrap().json.images;
    let list2 = json2.unwrap().json.images;
    let list = list.into_iter().chain(list2.clone().into_iter()).collect::<Vec<bing::Images>>();

    self.list = list;
  }

  pub async fn push_list(&mut self, image: Images) {
    self.list.push(image);
  }

  pub async fn rotate_photo(&self) {
    let mut list = self.list.clone();
    let cache = list.clone();

    let rotate_interval = config::PavoConfig::get_interval();
    let mut interval = time::interval(time::Duration::from_secs(rotate_interval));

    let mut setting_auto_rotate = config::PavoConfig::get_config().auto_rotate;

    while list.len() > 0 && setting_auto_rotate {
      let item = list.pop().unwrap();

      interval.tick().await;

      println!("{:?}", item.title);
      bing::Wallpaper::set_wallpaper(&item.url()).await.unwrap();

      setting_auto_rotate = config::PavoConfig::get_config().auto_rotate;

      if list.len() == 0 {
        list = cache.clone();
      }
    }
  }

  pub async fn stop_rotate_photo(&self) {

  }

  pub async fn rotate_photo_randomly(&mut self) {

  }

  pub async fn create_interval () {
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
              scheduler.rotate_photo().await;
              println!("init output start 2 {:?}", output);
            }
            AsyncProcessMessage::StopRotate => {
              println!("init output stop 2 {:?}", output);
            }
          }
        }
      }
    });
  }
}
