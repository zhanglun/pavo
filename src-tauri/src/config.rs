use std::path::{Path};
use std::fs;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use tauri;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  pub auto_rotate: bool,
  pub rotate_source: Vec<String>,
  pub randomly: bool,
  pub interval: u64,
}

impl PavoConfig {
  pub fn new() -> Self {
    Self {
      auto_rotate: false,
      rotate_source: vec![],
      randomly: false,
      interval: 30,
    }
  }

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

  pub fn write_config(data: PavoConfig) {
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("pavo.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).expect("create config failed");
    }

    let content = toml::to_string(&data).unwrap();

    fs::write(file_path, content).expect("write file error");
  }

  pub fn get_config() -> Self {
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("pavo.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).expect("create config failed");
    }

    let content = match fs::read_to_string(&file_path) {
      Ok(content) => content,
      Err(_) => "".to_string(),
    };

    let data: PavoConfig = match toml::from_str(&content) {
      Ok(data) => PavoConfig { ..data },
      Err(_) => PavoConfig::new()
    };

    data
  }

  pub fn set_auto_rotate(&self, auto_rotate: bool) -> Self {
    let mut data = Self::get_config();

    data.auto_rotate = auto_rotate;

    Self::write_config(data.clone());

    data
  }

  pub fn set_interval(&self, interval: u64) -> Self {
    let mut data = Self::get_config();

    data.interval = interval;

    println!("data; {:?}", data);

    Self::write_config(data.clone());

    data
  }

  pub fn set_rotate_source(&self, source: String, checked: bool) -> Self {
    let mut data = Self::get_config();

    if checked {
      data.rotate_source.push(source);
    } else {
      data.rotate_source.retain(|x| *x != source);
    }

    println!("data; {:?}", data);

    Self::write_config(data.clone());

    data
  }

  pub fn get_interval() -> u64 {
    let data = Self::get_config();
    println!("data: {:?}", data);

    data.interval.clone()
  }

  pub fn set_randomly(&self, randomly: bool) -> Self {
    let mut data = Self::get_config();

    data.randomly = randomly;

    Self::write_config(data.clone());

    data
  }
}
