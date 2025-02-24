use tauri::image::Image;
use tauri::{
  menu,
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, Emitter, Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::services::AsyncProcessMessage;
use tokio::sync::mpsc;

pub fn create_tray(
  app: &mut App,
  sender: mpsc::Sender<AsyncProcessMessage>,
) -> Result<(), tauri::Error> {
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

  let about = MenuItemBuilder::new("About Pavo")
    .id("about")
    .build(app)
    .unwrap();
  let check_for_update = MenuItemBuilder::new("Check for Updates")
    .id("check_for_updates")
    .build(app)
    .unwrap();
  let settings = MenuItemBuilder::new("Settings...")
    .id("settings")
    .build(app)
    .unwrap();

  let menu = MenuBuilder::new(app)
    .items(&[&previous_photo, &next_photo])
    .separator()
    .items(&[&show, &hide])
    .separator()
    .items(&[&about, &check_for_update, &settings])
    .separator()
    .items(&[&quit])
    .build()
    .unwrap();

  let icon_path = app
    .path()
    .resolve("icons/tray.png", tauri::path::BaseDirectory::Resource)?;

  let _ = TrayIconBuilder::new()
    .menu(&menu)
    .icon_as_template(true)
    .icon(Image::from_path(icon_path).unwrap())
    .on_tray_icon_event(|tray, event| {
      tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

      let app = tray.app_handle();

      if let Some(window) = app.get_webview_window("main") {
        let _ = window.move_window(Position::TrayCenter);
      }

      match event {
        TrayIconEvent::Click {
          button: MouseButton::Left,
          button_state: MouseButtonState::Up,
          ..
        } => {
          if let Some(window) = app.get_webview_window("main") {
            if window.is_visible().unwrap() {
              let _ = window.hide();
            } else {
              // let current_position = window.outer_position().unwrap();
              // let offset_position = tauri::PhysicalPosition {
              //   x: current_position.x - 288,
              //     y: current_position.y - 12,
              // };
              // let _ = window.set_position(tauri::Position::Physical(offset_position));
              let _ = window.show();
              let _ = window.set_focus();
            }
          }
        }
        // A double click happened on the tray icon. Windows Only
        // TrayIconEvent::DoubleClick {
        //   id,
        //   position,
        //   rect,
        //   button,
        // } => {
        //   println!("double click");
        //   let app = tray.app_handle();

        //   if let Some(window) = app.get_webview_window("main") {
        //     let _ = window.move_window(Position::TrayCenter);
        //     let _ = window.show();
        //     let _ = window.set_focus();
        //   }
        // }
        _ => {
          log::trace!("unhandled event {event:?}");
        }
      }
    })
    .menu_on_left_click(false)
    .on_menu_event(move |app, event| match event.id.as_ref() {
      "show" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.move_window(Position::TrayCenter);
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
      "about" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = app.emit("go-to-about", ());
          let _ = window.move_window(Position::TrayCenter);
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      "settings" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = app.emit("go-to-settings", ());
          let _ = window.move_window(Position::TrayCenter);
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      "check_for_updates" => {
        let _ = app.emit("check-for-updates", ());
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
