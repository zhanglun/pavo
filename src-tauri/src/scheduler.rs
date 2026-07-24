use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::sync::Mutex;

use crate::events::WallpaperEvent;
use crate::services::bing;
use tauri::Emitter;

pub const BING_EXPIRE_TIME: i64 = 60 * 60 * 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerPhoto {
  pub filename: String,
  pub regions: Vec<String>,
  pub urls: Vec<String>,
  pub titles: Vec<String>,
  pub startdates: Vec<String>,
  pub copyrights: Vec<String>,
  pub copyrightlinks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Scheduler {
  pub last_load_time: i64,
  pub cache_list: Vec<SchedulerPhoto>,
  pub current_idx: usize,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      last_load_time: Utc::now().timestamp(),
      cache_list: vec![],
      current_idx: 0,
    }
  }

  pub fn should_refresh(&self) -> bool {
    let now = Utc::now().timestamp();
    now - self.last_load_time >= BING_EXPIRE_TIME
  }

  pub fn is_cache_valid(&self) -> bool {
    let now = Utc::now().timestamp();
    now - self.last_load_time < BING_EXPIRE_TIME && !self.cache_list.is_empty()
  }

  pub async fn batch_fetch(
    &mut self,
  ) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
    if !self.should_refresh() && !self.cache_list.is_empty() {
      return Ok(self.cache_list.clone());
    }
    self.fetch_from_api().await
  }

  pub async fn force_fetch(
    &mut self,
  ) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
    self.fetch_from_api().await
  }

  async fn fetch_from_api(
    &mut self,
  ) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
    let region_codes = [
      "zh-CN", "en-US", "fr-FR", "de-DE", "ja-JP", "en-CA", "en-GB", "en-IN", "it-IT",
    ];

    let mut handles = vec![];

    for region_code in region_codes {
      let region = region_code.to_string();
      let handle = tokio::spawn(async move { fetch_list_with_region(region).await });
      handles.push(handle);
    }

    let mut res: Vec<SchedulerPhoto> = vec![];
    for handle in handles {
      if let Ok(Ok(mut photos)) = handle.await {
        res.append(&mut photos);
      }
    }

    let formatted_list = merge_scheduler_photos(res);

    self.cache_list = formatted_list.clone();
    self.last_load_time = Utc::now().timestamp();

    Ok(formatted_list)
  }

  pub async fn setup_list(&mut self) -> Vec<SchedulerPhoto> {
    self.batch_fetch().await.unwrap_or_default()
  }

  pub async fn previous_photo(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let list = self.cache_list.clone();

    if list.is_empty() {
      return Err("No wallpapers available in cache".into());
    }

    if self.current_idx == 0 {
      self.current_idx = list.len() - 1;
    } else {
      self.current_idx -= 1;
    }

    let item = &list[self.current_idx];

    bing::Wallpaper::set_wallpaper(&item.urls[0]).await?;

    Ok(())
  }

  pub async fn next_photo(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let list = self.cache_list.clone();

    if list.is_empty() {
      return Err("No wallpapers available in cache".into());
    }

    if self.current_idx >= list.len() - 1 {
      self.current_idx = 0;
    } else {
      self.current_idx += 1;
    }

    let item = &list[self.current_idx];

    bing::Wallpaper::set_wallpaper(&item.urls[0]).await?;

    Ok(())
  }

  pub async fn next_photo_with_event<R: tauri::Runtime>(
    &mut self,
    app: &tauri::AppHandle<R>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    self.next_photo().await?;
    self.emit_wallpaper_event(app).await?;
    Ok(())
  }

  pub async fn previous_photo_with_event<R: tauri::Runtime>(
    &mut self,
    app: &tauri::AppHandle<R>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    self.previous_photo().await?;
    self.emit_wallpaper_event(app).await?;
    Ok(())
  }

  async fn emit_wallpaper_event<R: tauri::Runtime>(
    &self,
    app: &tauri::AppHandle<R>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let item = &self.cache_list[self.current_idx];
    let event = WallpaperEvent {
      title: item.titles.first().cloned().unwrap_or_default(),
      copyright: item.copyrights.first().cloned().unwrap_or_default(),
      url: item.urls.first().cloned().unwrap_or_default(),
      startdate: item.startdates.first().cloned().unwrap_or_default(),
    };
    app.emit("wallpaper:changed", event)?;
    Ok(())
  }

  pub async fn emit_wallpaper_event_by_url<R: tauri::Runtime>(
    &self,
    app: &tauri::AppHandle<R>,
    url: &str,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let item = self
      .cache_list
      .iter()
      .find(|photo| photo.urls.contains(&url.to_string()));

    if let Some(item) = item {
      let event = WallpaperEvent {
        title: item.titles.first().cloned().unwrap_or_default(),
        copyright: item.copyrights.first().cloned().unwrap_or_default(),
        url: url.to_string(),
        startdate: item.startdates.first().cloned().unwrap_or_default(),
      };
      app.emit("wallpaper:changed", event)?;
    }

    Ok(())
  }

  pub fn filter_recent_days(list: &[SchedulerPhoto], days: u8, today: &str) -> Vec<SchedulerPhoto> {
    list
      .iter()
      .filter(|photo| {
        photo
          .startdates
          .iter()
          .any(|startdate| within_recent_days(startdate, today, days))
      })
      .cloned()
      .collect()
  }

  pub fn pick_today(list: &[SchedulerPhoto], today: &str) -> Option<SchedulerPhoto> {
    list
      .iter()
      .find(|photo| photo.startdates.first().map(String::as_str) == Some(today))
      .cloned()
      .or_else(|| {
        list
          .iter()
          .max_by(|a, b| {
            let a_sd = a.startdates.first().map(String::as_str).unwrap_or("");
            let b_sd = b.startdates.first().map(String::as_str).unwrap_or("");
            a_sd.cmp(b_sd)
          })
          .cloned()
      })
  }

  /// 从列表中筛选 startdate 包含指定日期的所有 SchedulerPhoto
  pub fn filter_today(list: &[SchedulerPhoto], today: &str) -> Vec<SchedulerPhoto> {
    list
      .iter()
      .filter(|photo| photo.startdates.iter().any(|sd| sd.as_str() == today))
      .cloned()
      .collect()
  }
}

fn merge_scheduler_photos(photos: Vec<SchedulerPhoto>) -> Vec<SchedulerPhoto> {
  let mut merged: Vec<SchedulerPhoto> = vec![];

  for photo in photos {
    let unique_name = photo.filename.split('_').next().unwrap_or(&photo.filename);
    // 合并键必须含日期：Bing 会在不同日期复用同一图名（OHR.XXX），
    // 只按前缀合并会把跨日期的同名图混并，导致同一天的不同地区图丢失。
    // 同一天同一张图的多地区才应合并。
    let photo_date = photo.startdates.first().cloned().unwrap_or_default();
    let Some(existing) = merged.iter_mut().find(|item| {
      item.filename.split('_').next().unwrap_or(&item.filename) == unique_name
        && item.startdates.first().map(String::as_str) == Some(photo_date.as_str())
    }) else {
      merged.push(photo);
      continue;
    };

    for index in 0..photo.regions.len() {
      let Some(region) = photo.regions.get(index) else {
        continue;
      };
      let Some(url) = photo.urls.get(index) else {
        continue;
      };
      let Some(startdate) = photo.startdates.get(index) else {
        continue;
      };
      let duplicate =
        existing
          .regions
          .iter()
          .enumerate()
          .any(|(existing_index, existing_region)| {
            existing_region == region
              && existing.urls.get(existing_index) == Some(url)
              && existing.startdates.get(existing_index) == Some(startdate)
          });
      if duplicate {
        continue;
      }

      existing.regions.push(region.clone());
      existing.urls.push(url.clone());
      existing
        .titles
        .push(photo.titles.get(index).cloned().unwrap_or_default());
      existing.startdates.push(startdate.clone());
      existing
        .copyrights
        .push(photo.copyrights.get(index).cloned().unwrap_or_default());
      existing
        .copyrightlinks
        .push(photo.copyrightlinks.get(index).cloned().unwrap_or_default());
    }
  }

  merged
}

async fn fetch_list_with_region(
  region: String,
) -> Result<Vec<SchedulerPhoto>, Box<dyn std::error::Error + Send + Sync>> {
  let res1 = bing::Wallpaper::new(0, 8, Some(region.clone())).await?;
  let res2 = bing::Wallpaper::new(7, 8, Some(region.clone())).await?;
  let res3 = bing::Wallpaper::new(14, 8, Some(region.clone())).await?;

  let images1 = res1.json.images;
  let images2 = res2.json.images;
  let images3 = res3.json.images;

  let mut res: Vec<SchedulerPhoto> = images1
    .into_iter()
    .chain(images2)
    .chain(images3)
    .map(
      |i| -> Result<SchedulerPhoto, Box<dyn std::error::Error + Send + Sync>> {
        let filename = bing::Images::get_filename(&i.url)?;
        Ok(SchedulerPhoto {
          filename,
          urls: vec![["https://www.bing.com", &i.url].concat()],
          regions: vec![region.clone()],
          titles: vec![i.clone().title],
          startdates: vec![i.clone().startdate],
          copyrights: vec![i.clone().copyright],
          copyrightlinks: vec![i.clone().copyrightlink],
        })
      },
    )
    .collect::<Result<_, _>>()?;

  res.dedup_by(|a, b| a.filename == b.filename);

  Ok(res)
}

fn within_recent_days(startdate: &str, today: &str, days: u8) -> bool {
  let Ok(start) = chrono::NaiveDate::parse_from_str(startdate, "%Y%m%d") else {
    return false;
  };
  let Ok(ref_day) = chrono::NaiveDate::parse_from_str(today, "%Y%m%d") else {
    return false;
  };
  let diff = ref_day.signed_duration_since(start).num_days();
  diff >= 0 && diff <= days as i64
}

pub static SCHEDULER: LazyLock<Mutex<Scheduler>> = LazyLock::new(|| Mutex::new(Scheduler::new()));

#[cfg(test)]
mod scheduler_tests {
  use super::*;

  fn photo(filename: &str, startdate: &str) -> SchedulerPhoto {
    SchedulerPhoto {
      filename: filename.into(),
      regions: vec!["zh-CN".into()],
      urls: vec![format!("https://example.com/{filename}")],
      titles: vec![filename.into()],
      startdates: vec![startdate.into()],
      copyrights: vec!["Copyright".into()],
      copyrightlinks: vec!["https://www.bing.com".into()],
    }
  }

  #[test]
  fn filter_recent_days_limits_result_count() {
    let list = vec![
      photo("a", "20260427"),
      photo("b", "20260426"),
      photo("c", "20260420"),
      photo("d", "20260410"),
    ];

    let recent = Scheduler::filter_recent_days(&list, 7, "20260427");
    // 20260427 (day 0), 20260426 (day 1), 20260420 (day 7) → all within 7 days
    // 20260410 (day 17) → excluded
    assert_eq!(recent.len(), 3);
  }

  #[test]
  fn filter_recent_days_14_includes_more() {
    let list = vec![
      photo("a", "20260427"),
      photo("b", "20260420"),
      photo("c", "20260414"),
      photo("d", "20260410"),
    ];

    let recent = Scheduler::filter_recent_days(&list, 14, "20260427");
    // 20260427 (day 0), 20260420 (day 7), 20260414 (day 13) → within 14 days
    // 20260410 (day 17) → excluded
    assert_eq!(recent.len(), 3);
  }

  #[test]
  fn filter_recent_days_keeps_group_when_any_region_is_recent() {
    let mut mixed = photo("mixed", "20260410");
    mixed.regions.push("en-US".into());
    mixed.urls.push("https://example.com/mixed-us".into());
    mixed.titles.push("Mixed US".into());
    mixed.startdates.push("20260427".into());
    mixed.copyrights.push("Copyright US".into());
    mixed.copyrightlinks.push("https://www.bing.com/us".into());

    let recent = Scheduler::filter_recent_days(&[mixed], 7, "20260427");
    assert_eq!(recent.len(), 1);
  }

  #[test]
  fn merge_scheduler_photos_ignores_exact_region_duplicates() {
    let first = photo("OHR.Sample_ZH-CN.jpg", "20260427");
    let duplicate = first.clone();
    let mut other_region = photo("OHR.Sample_EN-US.jpg", "20260427");
    other_region.regions[0] = "en-US".into();
    other_region.urls[0] = "https://example.com/sample-us".into();

    let merged = merge_scheduler_photos(vec![first, duplicate, other_region]);

    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].regions, vec!["zh-CN", "en-US"]);
    assert_eq!(merged[0].urls.len(), 2);
  }

  #[test]
  fn merge_scheduler_photos_keeps_same_named_images_on_different_dates_separate() {
    // Bing 会在不同日期复用同一图名（OHR.PinkDahlia），只按前缀合并
    // 会把跨日期的同名图混并，导致同一天的不同地区图丢失。
    let day_one_cn = photo("OHR.PinkDahlia_ZH-CN.jpg", "20260723");
    let day_two_us = photo("OHR.PinkDahlia_EN-US.jpg", "20260722");

    let merged = merge_scheduler_photos(vec![day_one_cn, day_two_us]);

    // 不同日期的同名图应各自独立，不合并
    assert_eq!(merged.len(), 2);
    assert_eq!(merged[0].startdates, vec!["20260723".to_string()]);
    assert_eq!(merged[1].startdates, vec!["20260722".to_string()]);
  }

  #[test]
  fn filter_recent_days_empty_list() {
    let list: Vec<SchedulerPhoto> = vec![];
    let recent = Scheduler::filter_recent_days(&list, 7, "20260427");
    assert!(recent.is_empty());
  }

  #[test]
  fn pick_today_returns_exact_match() {
    let list = vec![photo("a", "20260427"), photo("b", "20260426")];

    let today = Scheduler::pick_today(&list, "20260427");
    assert!(today.is_some());
    assert_eq!(today.unwrap().filename, "a");
  }

  #[test]
  fn pick_today_falls_back_to_newest() {
    let list = vec![
      photo("old", "20260420"),
      photo("newest", "20260426"),
      photo("mid", "20260423"),
    ];

    let today = Scheduler::pick_today(&list, "20260427");
    assert!(today.is_some());
    assert_eq!(today.unwrap().filename, "newest");
  }

  #[test]
  fn pick_today_returns_none_on_empty() {
    let list: Vec<SchedulerPhoto> = vec![];
    let today = Scheduler::pick_today(&list, "20260427");
    assert!(today.is_none());
  }

  #[test]
  fn filter_today_returns_all_matching_date() {
    let list = vec![
      photo("a", "20260427"),
      photo("b", "20260427"),
      photo("c", "20260426"),
      photo("d", "20260427"),
    ];

    let today_list = Scheduler::filter_today(&list, "20260427");
    assert_eq!(today_list.len(), 3);
    assert_eq!(
      today_list
        .iter()
        .map(|p| p.filename.clone())
        .collect::<Vec<_>>(),
      vec!["a", "b", "d"]
    );
  }

  #[test]
  fn filter_today_empty_when_no_match() {
    let list = vec![photo("a", "20260426")];
    let today_list = Scheduler::filter_today(&list, "20260427");
    assert!(today_list.is_empty());
  }

  #[test]
  fn should_refresh_returns_false_when_cache_is_fresh() {
    let scheduler = Scheduler::new();
    assert!(!scheduler.should_refresh());
  }

  #[test]
  fn should_refresh_returns_true_when_cache_is_expired() {
    let mut scheduler = Scheduler::new();
    scheduler.last_load_time = Utc::now().timestamp() - BING_EXPIRE_TIME - 1;
    assert!(scheduler.should_refresh());
  }

  #[test]
  fn should_refresh_has_no_side_effects() {
    let mut scheduler = Scheduler::new();
    scheduler.last_load_time = Utc::now().timestamp() - BING_EXPIRE_TIME - 1;
    let before = scheduler.last_load_time;
    let _ = scheduler.should_refresh();
    assert_eq!(scheduler.last_load_time, before);
  }

  #[test]
  fn is_cache_valid_returns_true_when_fresh_and_non_empty() {
    let mut scheduler = Scheduler::new();
    scheduler.cache_list = vec![photo("a", "20260427")];
    assert!(scheduler.is_cache_valid());
  }

  #[test]
  fn is_cache_valid_returns_false_when_expired() {
    let mut scheduler = Scheduler::new();
    scheduler.cache_list = vec![photo("a", "20260427")];
    scheduler.last_load_time = Utc::now().timestamp() - BING_EXPIRE_TIME - 1;
    assert!(!scheduler.is_cache_valid());
  }

  #[test]
  fn is_cache_valid_returns_false_when_empty() {
    let scheduler = Scheduler::new();
    assert!(!scheduler.is_cache_valid());
  }
}
