use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, task, time};

use crate::services::bing;

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

pub fn timer_wrap<F: Fn() + Send + 'static>(f: F) {
  thread::spawn(move || loop {
    thread::sleep(time::Duration::from_secs(5));
    f();
    println!("thread spawn 2");
  });
}

pub async fn rotate_photo() {
  let json1 = bing::Wallpaper::new(0, 8).await;
  let json2 = bing::Wallpaper::new(1, 8).await;

  let mut list = json1.unwrap().json.images;
  let mut list2 = json2.unwrap().json.images;

  list.append(&mut list2);

  let mut interval = time::interval(time::Duration::from_secs(10));

  for item in list {
    interval.tick().await;

    println!("{:?} item =-==>", item.url());
    println!("{:?} item.name =-==>", item.filename());

    bing::Wallpaper::set_wallpaper(&item.url()).await;
  }
}

pub fn stop_rotate_photo() {
  println!("stop rotate photo");
}

pub struct Scheduler {}

impl Scheduler {
  pub async fn rotate_photo() {
    let json1 = bing::Wallpaper::new(0, 8).await;
    let json2 = bing::Wallpaper::new(8, 8).await;

    let mut list = json1.unwrap().json.images;
    let mut list2 = json2.unwrap().json.images;
    let mut list = list.into_iter().chain(list2.clone().into_iter()).collect::<Vec<bing::Images>>();
    let mut cache = list.clone();
    let mut interval = time::interval(time::Duration::from_secs(5));

    while list.len() > 0 {
      let item = list.pop().unwrap();

      println!("{:?}", item.title);

      interval.tick().await;
      bing::Wallpaper::set_wallpaper(&item.url()).await;

      if list.len() == 0 {
        list = cache.clone();
      }
    }
  }

  pub async fn stop_rotate_photo() {
    // TODO: how to cancel inerval ?
  }
}
