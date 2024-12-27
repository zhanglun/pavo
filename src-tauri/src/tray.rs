use tauri::image::Image;
use tauri::{
  menu::{Menu, MenuItem},
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, Manager,
};

use crate::cmd::AsyncProcInputTx;
use crate::services::AsyncProcessMessage;
use tokio::sync::{mpsc, Mutex};

pub fn create_tray(app: &mut App, sender: mpsc::Sender<AsyncProcessMessage>) -> Result<(), tauri::Error> {
  let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
  let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
  let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
  // we could opt handle an error case better than calling unwrap

  let previous_photo = MenuItemBuilder::new("Previous photo")
    .id("previous_photo")
    .build(app)
    .unwrap();
  let next_photo = MenuItemBuilder::new("Next photo")
    .id("next_photo")
    .build(app)
    .unwrap();

  let menu = MenuBuilder::new(app)
    .items(&[&previous_photo, &next_photo])
    .separator()
    .items(&[&show, &hide, &quit])
    .build()
    .unwrap();

  let icon_path = app
    .path()
    .resolve("icons/tray.png", tauri::path::BaseDirectory::Resource)?;

  let _ = TrayIconBuilder::new()
    .menu(&menu)
    .icon_as_template(true)
    .icon(Image::from_path(icon_path).unwrap())
    .on_tray_icon_event(|tray, event| match event {
      TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } => {
        println!("left click pressed and released");
      }
      // A double click happened on the tray icon. Windows Only
      TrayIconEvent::DoubleClick {
        id,
        position,
        rect,
        button,
      } => {
        println!("double click");
        let app = tray.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      _ => {
        // println!("unhandled event {event:?}");
      }
    })
    .on_menu_event(move |app, event| match event.id.as_ref() {
      "show" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      "hide" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.hide();
        }
      }
      "previous_photo" => {
        let tx = sender.clone();
        tokio::spawn(async move {
          tx.send(AsyncProcessMessage::PreviousPhoto).await.unwrap();
        });
      }
      "next_photo" => {
        let tx = sender.clone();
        tokio::spawn(async move {
          tx.send(AsyncProcessMessage::NextPhoto).await.unwrap();
          println!("send");
        });
      }
      "quit" => {
        println!("quit menu item was clicked");
        app.exit(0);
      }
      "toggle" => {
        println!("toggle clicked");
      }
      _ => {
        println!("menu item {:?} not handled", event.id);
      }
    })
    .build(app);
  Ok(())
}
