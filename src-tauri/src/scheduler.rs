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
      println!("task start ===>");
        time::sleep(time::Duration::from_secs(5)).await;
        println!("task over: {}", now());
    });

  thread::spawn(|| {
    loop {
      thread::sleep(time::Duration::from_secs(10));
      println!("thread spawn");
    }
  });

}
