use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, task, time};

use crate::services::bing;
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

pub struct Scheduler {}

impl Scheduler {
  pub async fn rotate_photo() {
    let json1 = bing::Wallpaper::new(0, 8).await;
    let json2 = bing::Wallpaper::new(8, 8).await;

    let list = json1.unwrap().json.images;
    let list2 = json2.unwrap().json.images;
    let mut list = list.into_iter().chain(list2.clone().into_iter()).collect::<Vec<bing::Images>>();
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

    pub async fn create_interval () {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _guard = rt.enter();

        task::spawn(async {
            time::sleep(time::Duration::from_secs(3)).await;
            println!("task over: {}", now());
        });

        thread::sleep(time::Duration::from_secs(4));
    }
}
