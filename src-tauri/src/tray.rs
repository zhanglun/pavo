use std::sync::{LazyLock, Mutex};

use tauri::image::Image;
use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Emitter, Manager, Runtime, WebviewWindow,
};

use crate::services::AsyncProcessMessage;
use tokio::sync::mpsc;

/// Cached tray icon position and size from tray events.
/// Used as fallback when `tray.rect()` is unavailable (e.g., Linux).
static TRAY_RECT_CACHE: LazyLock<
  Mutex<Option<(tauri::PhysicalPosition<f64>, tauri::PhysicalSize<f64>)>>,
> = LazyLock::new(|| Mutex::new(None));

/// Cache tray rect from any tray icon event that carries position info.
fn cache_tray_rect(event: &TrayIconEvent) {
  let rect = match event {
    TrayIconEvent::Click { rect, .. }
    | TrayIconEvent::Enter { rect, .. }
    | TrayIconEvent::Leave { rect, .. }
    | TrayIconEvent::Move { rect, .. } => rect,
    _ => return,
  };

  let size = rect.size.to_physical(1.0);
  if size.width > 0.0 {
    let pos = rect.position.to_physical(1.0);
    *TRAY_RECT_CACHE.lock().unwrap() = Some((pos, size));
  }
}

/// Position the main window near the tray icon.
///
/// Strategy: `tray.rect()` → cached event rect → screen center.
/// Constrains the window to stay within the current monitor bounds.
fn position_window_near_tray<R: Runtime>(app: &AppHandle<R>, window: &WebviewWindow<R>) {
  let rect = app
    .tray_by_id("main-tray")
    .and_then(|tray| tray.rect().ok().flatten());

  let (tray_x, tray_y, tray_w, tray_h) = if let Some(r) = rect {
    let pos = r.position.to_physical(1.0);
    let size = r.size.to_physical(1.0);
    (pos.x, pos.y, size.width, size.height)
  } else if let Some((pos, size)) = TRAY_RECT_CACHE.lock().unwrap().clone() {
    (pos.x, pos.y, size.width, size.height)
  } else {
    let _ = window.center();
    return;
  };

  let Ok(win_size) = window.outer_size() else {
    return;
  };
  let win_w = win_size.width as f64;
  let win_h = win_size.height as f64;

  let x = tray_x + tray_w / 2.0 - win_w / 2.0;

  // macOS/Linux: tray is at top → window below tray
  // Windows: tray is at bottom → window above tray
  #[cfg(target_os = "windows")]
  let y = tray_y - win_h - 4.0;
  #[cfg(not(target_os = "windows"))]
  let y = tray_y + tray_h + 4.0;

  // Constrain to monitor bounds
  let (final_x, final_y) = if let Some(monitor) = window.current_monitor().ok().flatten() {
    let screen = monitor.size();
    let origin = monitor.position();
    let sw = screen.width as f64;
    let sh = screen.height as f64;
    let sx = origin.x as f64;
    let sy = origin.y as f64;

    (
      x.max(sx).min(sx + sw - win_w),
      y.max(sy).min(sy + sh - win_h),
    )
  } else {
    (x, y)
  };

  let _ = window.set_position(tauri::Position::Physical(
    tauri::PhysicalPosition::new(final_x as i32, final_y as i32),
  ));
}

pub fn create_tray(
  app: &mut App,
  sender: mpsc::Sender<AsyncProcessMessage>,
) -> Result<(), tauri::Error> {
  let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
  let show = MenuItemBuilder::new("Show").id("show").build(app)?;
  let hide = MenuItemBuilder::new("Hide").id("hide").build(app)?;

  let previous_wallpaper = MenuItemBuilder::new("Previous Wallpaper")
    .id("previous_wallpaper")
    .build(app)?;
  let next_wallpaper = MenuItemBuilder::new("Next Wallpaper")
    .id("next_wallpaper")
    .build(app)?;

  let check_for_update = MenuItemBuilder::new("Check for Updates")
    .id("check_for_updates")
    .build(app)?;
  let settings = MenuItemBuilder::new("Settings...")
    .id("settings")
    .build(app)?;

  let menu = MenuBuilder::new(app)
    .items(&[&previous_wallpaper, &next_wallpaper])
    .separator()
    .items(&[&show, &hide])
    .separator()
    .items(&[&check_for_update, &settings])
    .separator()
    .items(&[&quit])
    .build()?;

  let icon_path = app
    .path()
    .resolve("icons/tray.png", tauri::path::BaseDirectory::Resource)?;

  let _ = TrayIconBuilder::with_id("main-tray")
    .menu(&menu)
    .icon_as_template(true)
    .icon(Image::from_path(icon_path)?)
    .on_tray_icon_event(|tray, event| {
      let app = tray.app_handle();

      cache_tray_rect(&event);

      match event {
        TrayIconEvent::Click {
          button: MouseButton::Left,
          button_state: MouseButtonState::Up,
          ..
        } => {
          if let Some(window) = app.get_webview_window("main") {
            if window.is_minimized().unwrap_or(false) {
              let _ = window.unminimize();
              position_window_near_tray(&app, &window);
              let _ = window.show();
              let _ = window.set_focus();
              let _ = app.emit("window:shown", ());
            } else if window.is_visible().unwrap_or(false) {
              let _ = window.hide();
            } else {
              position_window_near_tray(&app, &window);
              let _ = window.show();
              let _ = window.set_focus();
              let _ = app.emit("window:shown", ());
            }
          }
        }
        _ => {
          log::trace!("unhandled tray event {event:?}");
        }
      }
    })
    .show_menu_on_left_click(false)
    .on_menu_event(move |app, event| match event.id.as_ref() {
      "show" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          position_window_near_tray(&app, &window);
          let _ = window.show();
          let _ = window.set_focus();
          let _ = app.emit("window:shown", ());
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
          if let Err(e) = tx.send(AsyncProcessMessage::PreviousPhoto).await {
            log::error!("failed to send PreviousPhoto: {}", e);
          }
        });
      }
      "next_wallpaper" => {
        let tx = sender.clone();
        tokio::spawn(async move {
          if let Err(e) = tx.send(AsyncProcessMessage::NextPhoto).await {
            log::error!("failed to send NextPhoto: {}", e);
          }
        });
      }
      "settings" => {
        let app = app.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = app.emit("go-to-settings", ());
          position_window_near_tray(&app, &window);
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
