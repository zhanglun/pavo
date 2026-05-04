use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use crate::{
  config, daily_update_thread, rotation_thread, scheduler, services::AsyncProcessMessage,
};

pub struct Background {}

impl Background {
  pub async fn new(
    receiver: Arc<Mutex<mpsc::Receiver<AsyncProcessMessage>>>,
    app: AppHandle,
  ) -> Self {
    scheduler::SCHEDULER.lock().await.setup_list().await;

    let mut daily_update_worker = daily_update_thread::DailyUpdateWorker::new();
    let mut rotation_worker = rotation_thread::RotationWorker::new();

    let cfg = config::PavoConfig::get_config();

    if cfg.auto_daily_update {
      daily_update_worker
        .start_daily_update(app.clone())
        .await;
    }

    if cfg.auto_rotate {
      rotation_worker
        .start(
          app.clone(),
          cfg.rotate_interval_minutes,
          cfg.rotate_mode,
        )
        .await;
    }

    let mut rotation_active = cfg.auto_rotate;
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
      loop {
        let message = receiver.lock().await.recv().await;

        if let Some(message) = message {
          println!("output: {:?}", message);

          match message {
            AsyncProcessMessage::StartDailyUpdate => {
              daily_update_worker
                .start_daily_update(app_clone.clone())
                .await;
            }
            AsyncProcessMessage::StopDailyUpdate => {
              daily_update_worker.stop_daily_update();
            }
            AsyncProcessMessage::StartRotation => {
              rotation_active = true;
              let cfg = config::PavoConfig::get_config();
              rotation_worker
                .start(
                  app_clone.clone(),
                  cfg.rotate_interval_minutes,
                  cfg.rotate_mode,
                )
                .await;
            }
            AsyncProcessMessage::StopRotation => {
              rotation_active = false;
              rotation_worker.stop();
            }
            AsyncProcessMessage::UpdateRotationConfig {
              interval_minutes,
              mode,
            } => {
              if rotation_active {
                rotation_worker.stop();
                rotation_worker
                  .start(app_clone.clone(), interval_minutes, mode)
                  .await;
              }
            }
            AsyncProcessMessage::PreviousPhoto => {
              let mut scheduler = scheduler::SCHEDULER.lock().await;
              let _ = scheduler.previous_photo_with_event(&app_clone).await;
            }
            AsyncProcessMessage::NextPhoto => {
              let mut scheduler = scheduler::SCHEDULER.lock().await;
              let _ = scheduler.next_photo_with_event(&app_clone).await;
            }
          };
        };
      }
    });

    let mut interval = time::interval(time::Duration::from_secs(scheduler::BING_EXPIRE_TIME as u64));
    let app_timer = app.clone();

    tauri::async_runtime::spawn(async move {
      loop {
        interval.tick().await;
        scheduler::SCHEDULER.lock().await.setup_list().await;
        let _ = app_timer.emit("wallpapers:cache-refreshed", ());
        log::info!("A Bright New Day!");
      }
    });

    Self {}
  }
}
