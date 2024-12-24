use tauri::{
  menu::{Menu, MenuItem},
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle,
};
use tauri::image::Image;

pub fn create_tray(app: &AppHandle) -> Result<(), tauri::Error> {
  let show = MenuItem::with_id(app, "show".to_string(), "Show", true, None::<&str>)?;
  let hide = MenuItem::with_id(app, "hide".to_string(), "Hide", true, None::<&str>)?;
  let quit = MenuItem::with_id(app, "quit".to_string(), "Quit", true, None::<&str>)?;

  let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
  let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;

  // let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

  let _ = TrayIconBuilder::new()
    .menu(&menu)
    .icon_as_template(true)
    .icon(Image::from_path("../icons/tray.png").unwrap())
    .on_tray_icon_event(|tray, event| match event {
      TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } => {
        println!("left click pressed and released");
        // in this example, let's show and focus the main window when the tray is clicked
        // let app = tray.app_handle();
        // if let Some(window) = app.get_webview_window("main") {
        //   let _ = window.show();
        //   let _ = window.set_focus();
        // }
      }
      _ => {
        println!("unhandled event {event:?}");
      }
    })
    .on_menu_event(|app, event| match event.id.as_ref() {
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
