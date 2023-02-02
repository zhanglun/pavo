use std::thread;
use chrono::Local;
use tokio::{self, runtime::Runtime, time, task};

#[allow(dead_code)]
fn now() -> String {
  Local::now().format("%F %T").to_string()
}

pub fn test_timer() {
  let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    task::spawn(async {
        time::sleep(time::Duration::from_secs(30)).await;
        println!("task over: {}", now());
    });

    thread::sleep(time::Duration::from_secs(4));
}
