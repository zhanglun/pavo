#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cache;
mod cmd;
mod config;
mod scheduler;
mod services;

use cmd::AsyncProcInputTx;
use services::AsyncProcessMessage;
use tauri::{AppHandle, Manager, tray::TrayIconBuilder, WindowEvent};
use tokio::sync::{mpsc, Mutex};

//fn create_tray() -> SystemTray {
//  let show = CustomMenuItem::new("show".to_string(), "Show");
//  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
//  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//
//  // let auto_shuffle = CustomMenuItem::new("auto_shuffle".to_string(), "Auto Shuffle");
//  let previous_photo = CustomMenuItem::new("previous_photo".to_string(), "Previous Photo");
//  let next_photo = CustomMenuItem::new("next_photo".to_string(), "Next Photo");
//
//  let tray_menu = SystemTrayMenu::new()
//    // .add_item(auto_shuffle)
//    .add_item(previous_photo)
//    .add_item(next_photo)
//    .add_native_item(SystemTrayMenuItem::Separator)
//    .add_item(show)
//    .add_item(hide)
//    .add_native_item(SystemTrayMenuItem::Separator)
//    .add_item(quit);
//
//  SystemTray::new().with_menu(tray_menu)
//}
//
//fn handle_tray_event(
//  app: &AppHandle,
//  event: SystemTrayEvent,
//  sender: mpsc::Sender<AsyncProcessMessage>,
//) {
//  match event {
//    SystemTrayEvent::DoubleClick {
//      tray_id,
//      position,
//      size,
//      ..
//    } => {
//      let window = app.get_webview_window("main").unwrap();
//      window.show().unwrap();
//    }
//    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
//      "previous_photo" => {
//        let tx = sender.clone();
//        tokio::spawn(async move {
//          tx.send(AsyncProcessMessage::PreviousPhoto).await.unwrap();
//        });
//      }
//      "next_photo" => {
//        let tx = sender.clone();
//        tokio::spawn(async move {
//          tx.send(AsyncProcessMessage::NextPhoto).await.unwrap();
//          println!("send");
//        });
//      }
//      "show" => {
//        let window = app.get_webview_window("main").unwrap();
//        window.show().unwrap();
//      }
//      "hide" => {
//        let window = app.get_webview_window("main").unwrap();
//        window.hide().unwrap();
//      }
//      "quit" => {
//        std::process::exit(0);
//      }
//      _ => {}
//    },
//    _ => {}
//  };
//}
//
//fn handle_window_event(event: GlobalWindowEvent<Wry>) {
//  let window = event.window();
//  let app = window.app_handle();
//
//  match event.event() {
//    WindowEvent::CloseRequested { api, .. } => {
//      let window = window.clone();
//      api.prevent_close();
//      window.hide().unwrap();
//      // ask the user if he wants to quit
//      // ask(
//      //   Some(&window),
//      //   "Tauri API",
//      //   "Are you sure that you want to close this window?",
//      //   |answer| {
//      //     if answer {
//      //       // .close() cannot be called on the main thread
//      //       std::thread::spawn(move || {
//      //         window.close().unwrap();
//      //       });
//      //     }
//      //   },
//      // );
//    }
//    WindowEvent::Focused(_) => {
//      println!("window focused!");
//
//      // tauri::async_runtime::spawn(async move {
//      //   let mut g_cache = cache::CACHE.lock().await;
//      //   g_cache.update_timestamp();
//      // });
//    }
//    _ => {}
//  }
//}

#[tokio::main]
async fn main() {
  config::PavoConfig::create_app_folder().expect("create app folder failed!");

  tauri::async_runtime::spawn(async move {
    let mut g_cache = cache::CACHE.lock().await;
    g_cache.update_timestamp_if_need();
  });

  let (async_process_input_tx, mut async_process_input_rx) =
    mpsc::channel::<AsyncProcessMessage>(32);
  let tx = async_process_input_tx.clone();

  let mut scheduler = scheduler::Scheduler::new();

  scheduler.setup_list(None).await;
  scheduler.shuffle_photo().await;

  tauri::async_runtime::spawn(async move {
    loop {
      if let Some(message) = async_process_input_rx.recv().await {
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
        }
      }
    }
  });

  tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .manage(AsyncProcInputTx {
      sender: Mutex::new(async_process_input_tx),
    })
    .setup(|app| {
      // let app_handle = app.handle();

      // scheduler::Scheduler::init(async_process_input_rx);
      #[cfg(target_os = "windows")]
      {
        //app
        //  .tray_handle()
        //  .set_icon(tauri::Icon::Raw(
        //    include_bytes!("../icons/win-icon.png").to_vec(),
        //  ))
        //  .unwrap();
      }
      let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

      Ok(())
    })
    //.system_tray(create_tray())
    //.on_system_tray_event(move |app, event| {
    //  let sender = tx.clone();
    //  handle_tray_event(app, event, sender)
    //})
    .invoke_handler(tauri::generate_handler![
      cmd::set_as_desktop,
      cmd::download,
      cmd::view_photo,
      cmd::get_bing_wallpaper_list,
      cmd::get_bing_daily,
      cmd::get_config,
      cmd::set_auto_shuffle,
      cmd::set_interval,
      cmd::set_randomly,
      cmd::set_shuffle_source,
    ])
    //.on_window_event(handle_window_event)
    .run(tauri::generate_context!())
    .expect("error while running Pavo");
}
