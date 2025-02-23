use crate::{scheduler, services::AsyncProcessMessage};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct Background {
  scheduler: scheduler::Scheduler,
  scheduler_thread: tauri::async_runtime::JoinHandle<()>,
}

impl Background {
  pub async fn new(receiver: Arc<Mutex<mpsc::Receiver<AsyncProcessMessage>>>) -> Self {
    let mut scheduler = scheduler::Scheduler::new();

    // scheduler.setup_list(None).await;
    // scheduler.shuffle_photo().await;

    println!("init output start");

    let scheduler_clone = scheduler.clone();

    let scheduler_thread = tauri::async_runtime::spawn(async move {
      // loop {
      //   scheduler.shuffle_photo().await;
      // }
    });
    tauri::async_runtime::spawn(async move {
      loop {
        let message = receiver.lock().await.recv().await;

        if let Some(message) = message {
          println!("output: {:?}", message);

          match message {
            AsyncProcessMessage::StartShuffle => {
              println!("init output start 2 {:?}", message);
              scheduler.start_shuffle_photo().await;
            }
            AsyncProcessMessage::StopShuffle => {
              println!("init output stop 2 {:?}", message);
              scheduler.stop_shuffle_photo();
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
      scheduler: scheduler_clone,
      scheduler_thread,
    }
  }
}
