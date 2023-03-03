use crate::config;
use crate::services;
use crate::services::bing;
use chrono::offset::Utc;

use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

const BING_EXPIRE_TIME: i64 = 60;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
  pub bing_daily: bing::Images,
  pub bing_list: Vec<bing::Images>,
  pub bing_timestamp: i64,
}

fn get_now_timestamp() -> i64 {
  Utc::now().timestamp()
}

impl Cache {
  /// update cache list
  pub fn update_cache_list(&mut self) {
    let cfg = config::PavoConfig::get_config();
  }

  /// get cached list, save request
  pub fn get_cache_list() {}

  /// get current photo which set as wallpaper
  pub fn get_current_photo() {}

  /// get photo list rotating
  pub fn get_rotate_list() {}

  pub fn rotate_to_next() {}

  pub fn rotate_to_previous() {}

  // cache service data

  pub async fn get_bing_daily(&mut self) -> bing::Images {
    let now = get_now_timestamp();

    if !self.bing_daily.url.is_empty() && self.bing_timestamp - now < BING_EXPIRE_TIME {
      return self.bing_daily.clone();
    }

    let bing = services::bing::Wallpaper::new(0, 1).await.unwrap();

    self.bing_daily = bing.json.images[0].clone();

    self.bing_daily.clone()
  }

  /// get bing photo list. return cached data if not expired.
  pub async fn get_bing_list(&mut self) -> Vec<bing::Images> {
    let now = Utc::now().timestamp();

    if !self.bing_list.is_empty() && self.bing_timestamp - now < BING_EXPIRE_TIME {
      println!("get list from cache");

      return self.bing_list.clone();
    }

    let res1 = services::bing::Wallpaper::new(0, 8).await.unwrap();
    let res2 = services::bing::Wallpaper::new(8, 8).await.unwrap();

    let images1 = res1.json.images;
    let images2 = res2.json.images;

    self.bing_list = images1.into_iter().chain(images2.into_iter()).collect();

    self.bing_timestamp = Utc::now().timestamp();

    println!("timestamp: {:?}", self.bing_timestamp);

    self.bing_list.clone()
  }

  /// update the time of last request to bing if 24 hours pasted
  pub fn update_bing_timestamp_if_need(&mut self) -> i64 {
    let now = get_now_timestamp();

    if now - self.bing_timestamp > 24 * 60 * 60 {
      self.update_bing_timestamp();
    }

    self.bing_timestamp
  }

  /// update the time of last request to bing
  pub fn update_bing_timestamp(&mut self) -> i64 {
    let now = get_now_timestamp();

    self.bing_timestamp = now;

    now
  }
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
  Mutex::new(Cache {
    bing_daily: bing::Images::default(),
    bing_list: vec![],
    bing_timestamp: Utc::now().timestamp(),
  })
});
