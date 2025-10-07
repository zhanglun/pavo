use crate::scheduler::Scheduler;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;

use crate::config;

pub struct ShuffleThread {
  thread: Option<tauri::async_runtime::JoinHandle<()>>,
}

impl ShuffleThread {
  pub fn new() -> Self {
    Self { thread: None }
  }

  pub async fn start_shuffle(&mut self, scheduler: Arc<Mutex<Scheduler>>) {
    if let Some(thread) = self.thread.take() {
      println!("shuffle thread abort, restart now");
      thread.abort();
    }

    let shuffle_interval = config::PavoConfig::get_interval();
    let mut interval = time::interval(time::Duration::from_secs(shuffle_interval * 60));

    let thread = tauri::async_runtime::spawn(async move {
      loop {
        log::info!(
          "WAITTING! Wallpaper will switch in {:?} mins \n",
          shuffle_interval
        );

        interval.tick().await;

        let mut scheduler = match scheduler.try_lock() {
          Ok(scheduler) => scheduler,
          Err(_) => {
            log::warn!("Failed to acquire scheduler lock, skipping this cycle");
            continue;
          }
        };

        match scheduler.next_photo().await {
          Ok(_) => {
            log::info!("Successfully switched wallpaper");
          }
          Err(e) => {
            log::error!("Failed to switch wallpaper: {}", e);

            // 简单恢复：前进到下一张图片
            scheduler.current_idx = (scheduler.current_idx + 1) % scheduler.cache_list.len().max(1);
            log::info!("Skipped to next wallpaper index: {}", scheduler.current_idx);
          }
        }
      }
    });

    log::info!(
      "shuffle thread started thread id: {:?}",
      thread.inner().id()
    );

    self.thread = Some(thread);
  }

  pub fn stop_shuffle(&mut self) {
    if let Some(thread) = self.thread.take() {
      println!(
        "shuffle thread abort, thread id: {:?} ",
        thread.inner().id()
      );
      thread.abort();
    }
  }
}
