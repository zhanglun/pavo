use std::collections::HashMap;

use crate::scheduler::SchedulerPhoto;
use crate::services;
use crate::services::bing;
use chrono::offset::Utc;
use rand::{distributions::Uniform, Rng};

use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

const BING_EXPIRE_TIME: i64 = 60 * 60 * 12;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
  pub bing_daily: bing::Images,
  pub bing_list: HashMap<String, Vec<bing::Images>>,
  pub timestamp: i64,
  pub current_idx: usize,
  pub current_lang: String,
  pub cache_list: Vec<SchedulerPhoto>,
}

fn get_now_timestamp() -> i64 {
  Utc::now().timestamp()
}

impl Cache {
  /// update cache list
  pub fn update_cache_list(&mut self, list: Vec<SchedulerPhoto>) {
    self.cache_list = list;
  }

  /// get cached list, save request
  pub fn get_cache_list() {}

  /// get current photo which set as wallpaper
  pub fn get_current_photo() {}

  /// get photo list rotating
  pub fn get_shuffle_list(self) -> Vec<SchedulerPhoto> {
    self.cache_list
  }

  pub fn shuffle_to_next(&mut self) -> SchedulerPhoto {
    if self.current_idx >= self.cache_list.len() - 1 {
      self.current_idx = 0;
    } else {
      self.current_idx += 1;
    }

    self.cache_list[self.current_idx].clone()
  }

  pub fn shuffle_to_previous(&mut self) -> SchedulerPhoto {
    if self.current_idx <= 0 {
      self.current_idx = self.cache_list.len() - 1;
    } else {
      self.current_idx -= 1;
    }

    self.cache_list[self.current_idx].clone()
  }

  pub fn get_random_photo(&mut self) -> SchedulerPhoto {
    let mut rng = rand::thread_rng();

    // self.current_idx = rng.gen_range(0, self.cache_list.len());
    self.current_idx = rng.sample(Uniform::new_inclusive(0, self.cache_list.len() - 1));

    self.cache_list[self.current_idx].clone()
  }

  // cache service data

  pub async fn get_bing_daily(&mut self, country: Option<String>) -> bing::Images {
    let now = get_now_timestamp();

    if !self.bing_daily.url.is_empty() && now - self.timestamp < BING_EXPIRE_TIME {
      return self.bing_daily.clone();
    }

    let bing = services::bing::Wallpaper::new(0, 1, country).await.unwrap();

    self.bing_daily = bing.json.images[0].clone();

    self.timestamp = now;

    self.bing_daily.clone()
  }

  /// get bing photo list. return cached data if not expired.
  pub async fn get_bing_list(&mut self, country: Option<String>) -> Vec<bing::Images> {
    let now = Utc::now().timestamp();
    let mut lang = self.current_lang.clone();

    if let Some(country) = country.clone() {
      println!("country ===> {:?}", country);
      lang = country;
    }

    let mut list = vec![];

    if let Some(l) = self.bing_list.get(&lang) {
      list = l.clone();
    }

    if list.len() > 0 && now - self.timestamp < BING_EXPIRE_TIME {
      return list.clone();
    }

    let res1 = services::bing::Wallpaper::new(0, 8, country.clone())
      .await
      .unwrap();
    let res2 = services::bing::Wallpaper::new(7, 8, country).await.unwrap();

    let images1 = res1.json.images;
    let images2 = res2.json.images;

    let mut res: Vec<bing::Images> = images1
      .into_iter()
      .chain(images2.into_iter())
      .map(|mut i| {
        i.url = i.url();
        i
      })
      .collect();

    res.dedup_by(|a, b| a.url == b.url);

    self.timestamp = Utc::now().timestamp();

    println!("timestamp: {:?}", self.timestamp);

    if self.bing_list.get(&lang).is_none() {
      self.bing_list.insert(lang.to_string(), res.clone());
    }

    res.clone()
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
    bing_list: HashMap::new(),
    timestamp: Utc::now().timestamp(),
    current_idx: 0,
    cache_list: vec![],
    current_lang: "en-US".to_string(),
  })
});
