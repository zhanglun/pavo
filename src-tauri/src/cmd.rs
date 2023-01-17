#[tauri::command]
pub fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn set_as_desktop(url: &str) -> String {
  println!("set as desktop : {:?}", url);

  String::from(url)
}
