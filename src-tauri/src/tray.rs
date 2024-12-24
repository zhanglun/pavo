use tauri::{
Manager,
  App,
  menu::{Menu, MenuItem},
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri::image::Image;

pub fn create_tray(app: &mut App) -> Result<(), tauri::Error> {

  let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
  let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
  let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
  // we could opt handle an error case better than calling unwrap
  let menu = MenuBuilder::new(app)
    .items(&[&show, &hide, &quit])
    .build()
    .unwrap();

  let icon_path = app.path().resolve("icons/tray.png", tauri::path::BaseDirectory::Resource)?;

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
        let app = tray.app_handle();

        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
      _ => {
        println!("unhandled event {event:?}");
      }
    })
    .on_menu_event(|app, event| match event.id.as_ref() {
     "show" => {
       let window = app.get_webview_window("main").unwrap();
       window.show().unwrap();
     }
     "hide" => {
       let window = app.get_webview_window("main").unwrap();
       window.hide().unwrap();
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
