use crate::scheduler::Scheduler;
use crate::services::bing;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::Mutex;
use tokio::time;

use crate::events::WallpaperEvent;

pub struct DailyUpdateWorker {
  app: Option<AppHandle>,
  thread: Option<tauri::async_runtime::JoinHandle<()>>,
}

const DAILY_UPDATE_INTERVAL_SECS: u64 = 60 * 60 * 12;

impl DailyUpdateWorker {
  pub fn new() -> Self {
    Self {
      app: None,
      thread: None,
    }
  }

  pub async fn start_daily_update(&mut self, app: AppHandle, scheduler: Arc<Mutex<Scheduler>>) {
    self.app = Some(app.clone());

    if let Some(thread) = self.thread.take() {
      thread.abort();
    }

    let mut interval = time::interval(time::Duration::from_secs(DAILY_UPDATE_INTERVAL_SECS));

    let thread = tauri::async_runtime::spawn(async move {
      loop {
        interval.tick().await;

        let mut scheduler = match scheduler.try_lock() {
          Ok(scheduler) => scheduler,
          Err(_) => {
            log::warn!("Failed to acquire scheduler lock, skipping this cycle");
            continue;
          }
        };

        let today = chrono::Local::now().format("%Y%m%d").to_string();

        let list = match scheduler.batch_fetch().await {
          Ok(list) => list,
          Err(e) => {
            log::error!("Failed to fetch wallpaper list for daily update: {}", e);
            continue;
          }
        };

        let today_wallpaper = Scheduler::pick_today(&list, &today);

        match today_wallpaper {
          Some(photo) => {
            let url = match photo.urls.first() {
              Some(u) => u.clone(),
              None => {
                log::error!("Today wallpaper has no URL");
                continue;
              }
            };

            match bing::Wallpaper::set_wallpaper(&url).await {
              Ok(_) => {
                if let Some(idx) = list.iter().position(|p| p.filename == photo.filename) {
                  scheduler.current_idx = idx;
                }
                let event = WallpaperEvent {
                  title: photo.titles.first().cloned().unwrap_or_default(),
                  copyright: photo.copyrights.first().cloned().unwrap_or_default(),
                  url: url.clone(),
                  startdate: photo.startdates.first().cloned().unwrap_or_default(),
                };
                let _ = app.emit("wallpaper:changed", event);
                log::info!("Daily wallpaper updated to today's Bing wallpaper");
              }
              Err(e) => {
                log::error!("Failed to set daily wallpaper: {}", e);
              }
            }
          }
          None => {
            log::warn!("No wallpaper found for today, skipping daily update");
          }
        }
      }
    });

    log::info!("daily update thread started: {:?}", thread.inner().id());

    self.thread = Some(thread);
  }

  pub fn stop_daily_update(&mut self) {
    if let Some(thread) = self.thread.take() {
      log::info!("daily update thread stopped: {:?}", thread.inner().id());
      thread.abort();
    }
  }
}
