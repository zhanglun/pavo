use tauri::image::Image;
use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, Emitter, Manager,
};

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

  let previous_wallpaper = MenuItemBuilder::new("Previous Wallpaper")
    .id("previous_wallpaper")
    .build(app)
    .unwrap();
  let next_wallpaper = MenuItemBuilder::new("Next Wallpaper")
    .id("next_wallpaper")
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
    .items(&[&previous_wallpaper, &next_wallpaper])
    .separator()
    .items(&[&show, &hide])
    .separator()
    .items(&[&check_for_update, &settings])
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
      let app = tray.app_handle();

      match event {
        TrayIconEvent::Click {
          button: MouseButton::Left,
          button_state: MouseButtonState::Up,
          ..
        } => {
          if let Some(window) = app.get_webview_window("main") {
            if window.is_minimized().unwrap() {
              let _ = window.unminimize().unwrap();
              let _ = window.show();
              let _ = window.set_focus();
            } else if window.is_visible().unwrap() {
              let _ = window.hide();
            } else {
              let _ = window.show();
              let _ = window.set_focus();
            }
          }
        }
        _ => {
          log::trace!("unhandled event {event:?}");
        }
      }
    })
    .show_menu_on_left_click(false)
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
      "previous_wallpaper" => {
        let tx = sender.clone();
        tokio::spawn(async move {
          tx.send(AsyncProcessMessage::PreviousPhoto).await.unwrap();
        });
      }
      "next_wallpaper" => {
        let tx = sender.clone();
        tokio::spawn(async move {
          tx.send(AsyncProcessMessage::NextPhoto).await.unwrap();
        });
      }
      "settings" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = app.emit("go-to-settings", ());
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      "check_for_updates" => {
        let _ = app.emit("check-for-updates", ());
      }
      "quit" => {
        app.exit(0);
      }
      _ => {}
    })
    .build(app);
  Ok(())
}
