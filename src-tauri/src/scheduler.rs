use chrono::offset::Utc;
use chrono::Local;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::events::WallpaperEvent;
use crate::services::bing;
use tauri::Emitter;

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

pub const BING_EXPIRE_TIME: i64 = 60 * 60 * 12;

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

  pub fn should_refresh(&mut self) -> bool {
    let now = Utc::now().timestamp();

    if now - self.last_load_time < BING_EXPIRE_TIME {
      false
    } else {
      self.last_load_time = now;
      true
    }
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

    let region_codes = [
      "zh-CN", "en-US", "fr-FR", "de-DE", "ja-JP", "en-CA", "en-GB", "en-IN", "it-IT",
    ];

    let mut handles = vec![];

    for region_code in region_codes {
      let region = region_code.to_string();
      let mut scheduler = self.clone();
      let handle = tokio::spawn(async move { scheduler.fetch_list_with_region(region).await });
      handles.push(handle);
    }

    let mut res: Vec<SchedulerPhoto> = vec![];
    for handle in handles {
      if let Ok(Ok(mut photos)) = handle.await {
        res.append(&mut photos);
      }
    }

    let mut formatted_list = vec![];

    for i in res {
      let unique_name = i.filename.clone().split("_").collect::<Vec<_>>()[0].to_string();

      let idx = formatted_list.iter().position(|x: &SchedulerPhoto| {
        x.filename.clone().split("_").collect::<Vec<_>>()[0] == unique_name
      });

      match idx {
        Some(idx) => {
          let item = &mut formatted_list[idx];

          item.regions.append(&mut i.clone().regions);
          item.urls.append(&mut i.clone().urls);
          item.titles.append(&mut i.clone().titles);
          item.startdates.append(&mut i.clone().startdates);
          item.copyrights.append(&mut i.clone().copyrights);
          item.copyrightlinks.append(&mut i.clone().copyrightlinks);
        }
        None => {
          let item = i.clone();
          formatted_list.push(item);
        }
      }
    }

    self.cache_list = formatted_list.clone();

    Ok(formatted_list)
  }

  pub async fn fetch_list_with_region(
    &mut self,
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
        let sd = photo
          .startdates
          .first()
          .map(String::as_str)
          .unwrap_or_default();
        within_recent_days(sd, today, days)
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

pub static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));

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
}
