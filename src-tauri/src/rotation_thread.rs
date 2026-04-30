use crate::config::RotateMode;
use crate::events::WallpaperEvent;
use crate::scheduler;
use crate::services::bing;
use rand::Rng;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::time;

pub struct RotationWorker {
  thread: Option<tauri::async_runtime::JoinHandle<()>>,
}

impl RotationWorker {
  pub fn new() -> Self {
    Self { thread: None }
  }

  pub async fn start(
    &mut self,
    app: AppHandle,
    interval_minutes: u16,
    mode: RotateMode,
  ) {
    if let Some(thread) = self.thread.take() {
      thread.abort();
    }

    let mode_clone = mode.clone();
    let mut interval = time::interval(time::Duration::from_secs(interval_minutes as u64 * 60));

    let thread = tauri::async_runtime::spawn(async move {
      loop {
        interval.tick().await;

        let mut scheduler = match scheduler::SCHEDULER.try_lock() {
          Ok(s) => s,
          Err(_) => {
            log::warn!("rotation: failed to acquire scheduler lock, skipping");
            continue;
          }
        };

        let list = match scheduler.batch_fetch().await {
          Ok(l) => l,
          Err(e) => {
            log::error!("rotation: failed to fetch list: {}", e);
            continue;
          }
        };

        if list.is_empty() {
          log::warn!("rotation: wallpaper list is empty, skipping");
          continue;
        }

        let idx = match mode {
          RotateMode::Sequential => {
            let idx = scheduler.current_idx;
            scheduler.current_idx = (idx + 1) % list.len();
            idx
          }
          RotateMode::Random => rand::rng().random_range(0..list.len()),
        };

        let photo = &list[idx];
        let url = match photo.urls.first() {
          Some(u) => u.clone(),
          None => continue,
        };

        match bing::Wallpaper::set_wallpaper(&url).await {
          Ok(_) => {
            let event = WallpaperEvent {
              title: photo.titles.first().cloned().unwrap_or_default(),
              copyright: photo.copyrights.first().cloned().unwrap_or_default(),
              url: url.clone(),
              startdate: photo.startdates.first().cloned().unwrap_or_default(),
            };
            let _ = app.emit("wallpaper:changed", event);
            log::info!("rotation: wallpaper changed (idx={})", idx);
          }
          Err(e) => {
            log::error!("rotation: failed to set wallpaper: {}", e);
          }
        }
      }
    });

    log::info!(
      "rotation worker started (interval={}min, mode={:?})",
      interval_minutes,
      mode_clone
    );
    self.thread = Some(thread);
  }

  #[allow(dead_code)]
  pub fn stop(&mut self) {
    if let Some(thread) = self.thread.take() {
      log::info!("rotation worker stopped");
      thread.abort();
    }
  }
}
