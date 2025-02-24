use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::{config, scheduler, services::AsyncProcessMessage, shuffle_thread};

pub struct Background {
}

impl Background {
  pub async fn new(receiver: Arc<Mutex<mpsc::Receiver<AsyncProcessMessage>>>) -> Self {
    let mut scheduler = scheduler::Scheduler::new();
    let mut shuffle_thread = shuffle_thread::ShuffleThread::new();

    scheduler.setup_list(None).await;

    let mut cfg = config::PavoConfig::get_config();

    if cfg.auto_shuffle {
      shuffle_thread
        .start_shuffle(Arc::new(Mutex::new(scheduler.clone())))
        .await;
    }

    let scheduler_clone = scheduler.clone();

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

    Self {
    }
  }
}
