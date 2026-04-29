use std::sync::Arc;
use tauri::AppHandle;
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use crate::{
  config, daily_update_thread, rotation_thread, scheduler, services::AsyncProcessMessage,
};

const BING_EXPIRE_TIME: u64 = 60 * 60 * 12;

pub struct Background {}

impl Background {
  pub async fn new(
    receiver: Arc<Mutex<mpsc::Receiver<AsyncProcessMessage>>>,
    app: AppHandle,
  ) -> Self {
    let mut scheduler = scheduler::Scheduler::new();
    scheduler.setup_list().await;
    let mut daily_update_worker = daily_update_thread::DailyUpdateWorker::new();
    let mut rotation_worker = rotation_thread::RotationWorker::new();
    let mut scheduler_clone = scheduler.clone();

    let cfg = config::PavoConfig::get_config();

    if cfg.auto_daily_update {
      daily_update_worker
        .start_daily_update(app.clone(), Arc::new(Mutex::new(scheduler.clone())))
        .await;
    }

    if cfg.auto_rotate {
      rotation_worker
        .start(
          app.clone(),
          Arc::new(Mutex::new(scheduler.clone())),
          cfg.rotate_interval_minutes,
          cfg.rotate_mode,
        )
        .await;
    }

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
      loop {
        let message = receiver.lock().await.recv().await;

        if let Some(message) = message {
          println!("output: {:?}", message);

          match message {
            AsyncProcessMessage::StartDailyUpdate => {
              daily_update_worker
                .start_daily_update(app_clone.clone(), Arc::new(Mutex::new(scheduler.clone())))
                .await;
            }
            AsyncProcessMessage::StopDailyUpdate => {
              daily_update_worker.stop_daily_update();
            }
            AsyncProcessMessage::PreviousPhoto => {
              let _ = scheduler.previous_photo_with_event(&app_clone).await;
            }
            AsyncProcessMessage::NextPhoto => {
              let _ = scheduler.next_photo_with_event(&app_clone).await;
            }
          };
        };
      }
    });

    let mut interval = time::interval(time::Duration::from_secs(BING_EXPIRE_TIME));

    tauri::async_runtime::spawn(async move {
      loop {
        interval.tick().await;
        scheduler_clone.setup_list().await;
        log::info!("A Bright New Day!");
      }
    });

    Self {}
  }
}
