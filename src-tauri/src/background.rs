use std::sync::Arc;
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use crate::{config, scheduler, services::AsyncProcessMessage, shuffle_thread};

const BING_EXPIRE_TIME: u64 = 60 * 60 * 12;

pub struct Background {}

impl Background {
  pub async fn new(receiver: Arc<Mutex<mpsc::Receiver<AsyncProcessMessage>>>) -> Self {
    let mut scheduler = scheduler::Scheduler::new();
    scheduler.setup_list().await;
    let mut shuffle_thread = shuffle_thread::ShuffleThread::new();
    let mut scheduler_clone = scheduler.clone();

    let cfg = config::PavoConfig::get_config();

    if cfg.auto_shuffle {
      shuffle_thread
        .start_shuffle(Arc::new(Mutex::new(scheduler.clone())))
        .await;
    }

    tauri::async_runtime::spawn(async move {
      loop {
        let message = receiver.lock().await.recv().await;

        if let Some(message) = message {
          println!("output: {:?}", message);

          match message {
            AsyncProcessMessage::StartShuffle => {
              println!("init output start 2 {:?}", message);
              shuffle_thread
                .start_shuffle(Arc::new(Mutex::new(scheduler.clone())))
                .await;
            }
            AsyncProcessMessage::StopShuffle => {
              println!("init output stop 2 {:?}", message);
              shuffle_thread.stop_shuffle();
            }
            AsyncProcessMessage::PreviousPhoto => {
              println!("PreviousPhoto {:?}", message);
              scheduler.previous_photo().await;
            }
            AsyncProcessMessage::NextPhoto => {
              println!("NextPhoto {:?}", message);
              scheduler.next_photo().await;
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
