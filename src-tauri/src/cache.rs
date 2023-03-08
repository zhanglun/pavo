use crate::services;
use crate::{
  config,
  services::{bing, pexels},
};
use chrono::offset::Utc;

use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

const BING_EXPIRE_TIME: i64 = 60;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
  pub bing_daily: bing::Images,
  pub bing_list: Vec<bing::Images>,
  pub timestamp: i64,
  pub pexels_list: Vec<pexels::Photo>,
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

    if !self.bing_daily.url.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
      return self.bing_daily.clone();
    }

    let bing = services::bing::Wallpaper::new(0, 1).await.unwrap();

    self.bing_daily = bing.json.images[0].clone();

    self.timestamp = now;

    self.bing_daily.clone()
  }

  /// get bing photo list. return cached data if not expired.
  pub async fn get_bing_list(&mut self) -> Vec<bing::Images> {
    let now = Utc::now().timestamp();

    if !self.bing_list.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
      return self.bing_list.clone();
    }

    let res1 = services::bing::Wallpaper::new(0, 8).await.unwrap();
    let res2 = services::bing::Wallpaper::new(8, 8).await.unwrap();

    let images1 = res1.json.images;
    let images2 = res2.json.images;

    self.bing_list = images1.into_iter().chain(images2.into_iter()).collect();

    self.timestamp = Utc::now().timestamp();

    println!("timestamp: {:?}", self.timestamp);

    self.bing_list.clone()
  }

  pub async fn get_pexels_list(&mut self) -> Vec<pexels::Photo> {
    let now = Utc::now().timestamp();

    if !self.pexels_list.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
      return self.pexels_list.clone();
    }

    let pexels_client =
      pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
    let res: pexels::PexlesJSON = pexels_client.get_photo_curated(30, 1).await;

    self.pexels_list = res.photos;

    self.timestamp = Utc::now().timestamp();

    self.pexels_list.clone()
  }

  /// update the time of last request to bing if 24 hours pasted
  pub fn update_timestamp_if_need(&mut self) -> i64 {
    let now = get_now_timestamp();

    if now - self.timestamp > 24 * 60 * 60 {
      self.update_timestamp();
    }

    self.timestamp
  }

  /// update the time of last request to bing
  pub fn update_timestamp(&mut self) -> i64 {
    let now = get_now_timestamp();

    self.timestamp = now;

    now
  }
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
  Mutex::new(Cache {
    bing_daily: bing::Images::default(),
    bing_list: vec![],
    timestamp: Utc::now().timestamp(),
    pexels_list: vec![],
  })
});
