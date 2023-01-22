use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Error, ErrorKind};
use tauri;

pub struct PavoConfig {

}

impl PavoConfig {
  pub fn create_app_folder () -> Result<String, Error> {
    let home_dir = tauri::api::path::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".pavo");

        match fs::create_dir_all(app_config_dir.clone()) {
          Ok(_) => {
            Ok(app_config_dir.clone().to_str().unwrap().to_string())
          },
          Err(e) => Err(e)
        }
      }
      None => {
       Err(Error::new(ErrorKind::NotFound, "home dir is not fount"))
      }
    }
  }

  pub fn get_app_folder() -> Result<String, (usize, String)> {
    let home_dir = tauri::api::path::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".pavo");

        if app_config_dir.exists() {
          Ok(app_config_dir.clone().to_str().unwrap().to_string())
        } else {
          Ok(Self::create_app_folder().unwrap())
        }
      }
      None => {
        Err((2, "no home dir".to_string()))
      }
    }
  }
}
