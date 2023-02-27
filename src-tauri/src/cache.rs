use crate::config;
use crate::services;
use crate::services::bing;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
  pub bing_daily: bing::Images,
  pub bing_list: Vec<bing::Images>,
}

impl Cache {
  /// update cache list
  pub fn update_cache_list(&mut self) {
    let cfg = config::PavoConfig::get_config();
  }

  /// get cached list, save request
  pub fn get_cache_list() {}

  /// get current photo which setted as wallpaper
  pub fn get_current_photo() {}

  /// get photo list rotating
  pub fn get_rotate_list() {}

  pub fn rotate_to_next() {}

  pub fn rotate_to_previous() {}

  // cache service data

  pub async fn get_bing_daily(&mut self) -> bing::Images {
    let bing = services::bing::Wallpaper::new(0, 1).await.unwrap();

    self.bing_daily = bing.json.images[0].clone();

    self.bing_daily.clone()
  }
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::default()));
