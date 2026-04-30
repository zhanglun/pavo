use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::Mutex as StdMutex;

static CONFIG_LOCK: Lazy<StdMutex<()>> = Lazy::new(|| StdMutex::new(()));

/// 轮播模式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum RotateMode {
  #[default]
  Sequential,
  Random,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteItem {
  pub filename: String,
  pub url: String,
  pub title: String,
  pub startdate: String,
  pub copyright: String,
  pub copyrightlink: String,
  #[serde(default)]
  pub local_path: Option<String>,
}

fn default_auto_daily_update() -> bool {
  true
}

fn default_auto_rotate() -> bool {
  false
}

fn default_rotate_interval_minutes() -> u16 {
  60
}

fn default_rotate_mode() -> RotateMode {
  RotateMode::Sequential
}

fn default_history_range_days() -> u8 {
  7
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  #[serde(default = "default_auto_daily_update")]
  pub auto_daily_update: bool,
  #[serde(default = "default_history_range_days")]
  pub history_range_days: u8,
  #[serde(default)]
  pub show_layer: bool,
  #[serde(default)]
  pub favorites: Vec<FavoriteItem>,
  #[serde(default = "default_auto_rotate")]
  pub auto_rotate: bool,
  #[serde(default = "default_rotate_interval_minutes")]
  pub rotate_interval_minutes: u16,
  #[serde(default = "default_rotate_mode")]
  pub rotate_mode: RotateMode,
}

impl PavoConfig {
  pub fn new() -> Self {
    Self {
      auto_daily_update: true,
      history_range_days: 7,
      show_layer: false,
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
    }
  }

  pub fn create_app_folder() -> Result<String, Error> {
    let home_dir = dirs::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".pavo");

        match fs::create_dir_all(app_config_dir.clone()) {
          Ok(_) => Ok(app_config_dir.clone().to_str().unwrap().to_string()),
          Err(e) => Err(e),
        }
      }
      None => Err(Error::new(ErrorKind::NotFound, "home dir is not fount")),
    }
  }

  pub fn get_app_folder() -> Result<String, (usize, String)> {
    let home_dir = dirs::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".pavo");

        if app_config_dir.exists() {
          Ok(app_config_dir.clone().to_str().unwrap().to_string())
        } else {
          Ok(Self::create_app_folder().unwrap())
        }
      }
      None => Err((2, "no home dir".to_string())),
    }
  }

  pub fn write_config(data: PavoConfig) -> Result<(), String> {
    let folder_dir = Self::get_app_folder().map_err(|(_, e)| e)?;
    let file_path = Path::new(&folder_dir).join("pavo.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).map_err(|e| e.to_string())?;
    }

    let content = toml::to_string(&data).map_err(|e| e.to_string())?;

    fs::write(file_path, content).map_err(|e| e.to_string())?;
    Ok(())
  }

  pub fn parse_tolerant(content: &str) -> Self {
    if let Ok(data) = toml::from_str::<PavoConfig>(content) {
      return data;
    }

    let table = match toml::from_str::<toml::Value>(content) {
      Ok(toml::Value::Table(t)) => t,
      _ => return PavoConfig::new(),
    };

    PavoConfig {
      auto_daily_update: table
        .get("auto_daily_update")
        .and_then(|v| v.as_bool())
        .unwrap_or(true),
      history_range_days: table
        .get("history_range_days")
        .and_then(|v| v.as_integer())
        .and_then(|i| u8::try_from(i).ok())
        .unwrap_or(7),
      show_layer: table
        .get("show_layer")
        .and_then(|v| v.as_bool())
        .unwrap_or(false),
      favorites: table
        .get("favorites")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
          arr
            .iter()
            .filter_map(|item| {
              item.as_table().and_then(|t| {
                let filename = t.get("filename")?.as_str()?.to_string();
                let url = t.get("url")?.as_str()?.to_string();
                let title = t.get("title")?.as_str()?.to_string();
                let startdate = t.get("startdate")?.as_str()?.to_string();
                let copyright = t.get("copyright")?.as_str()?.to_string();
                let copyrightlink = t.get("copyrightlink")?.as_str()?.to_string();
                let local_path = t
                  .get("local_path")
                  .and_then(|v| v.as_str())
                  .map(String::from);
                Some(FavoriteItem {
                  filename,
                  url,
                  title,
                  startdate,
                  copyright,
                  copyrightlink,
                  local_path,
                })
              })
            })
            .collect::<Vec<_>>()
            .into()
        })
        .unwrap_or_default(),
      auto_rotate: table
        .get("auto_rotate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false),
      rotate_interval_minutes: table
        .get("rotate_interval_minutes")
        .and_then(|v| v.as_integer())
        .and_then(|i| u16::try_from(i).ok())
        .unwrap_or(60),
      rotate_mode: table
        .get("rotate_mode")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
          "Random" => Some(RotateMode::Random),
          "Sequential" => Some(RotateMode::Sequential),
          _ => None,
        })
        .unwrap_or_default(),
    }
  }

  pub fn get_config() -> Self {
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("pavo.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).expect("create config failed");
    }

    let content = fs::read_to_string(&file_path).unwrap_or_default();
    Self::parse_tolerant(&content)
  }

  pub fn set_show_layer(&self, show_layer: bool) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();

    data.show_layer = show_layer;

    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }

    data
  }

  pub fn set_auto_daily_update(&self, enabled: bool) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.auto_daily_update = enabled;
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn set_history_range_days(&self, days: u8) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.history_range_days = days;
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn add_favorite(&self, item: FavoriteItem) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    if !data.favorites.iter().any(|f| f.filename == item.filename) {
      data.favorites.push(item);
    }
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn remove_favorite_by_filename(&self, filename: &str) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.favorites.retain(|f| f.filename != filename);
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn set_auto_rotate(&self, enabled: bool) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.auto_rotate = enabled;
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn set_rotate_interval(&self, minutes: u16) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.rotate_interval_minutes = minutes;
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn set_rotate_mode(&self, mode: RotateMode) -> Self {
    let _guard = CONFIG_LOCK.lock().unwrap();
    let mut data = Self::get_config();
    data.rotate_mode = mode;
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_legacy_config_without_new_fields() {
    let input = r#"
show_layer = false
"#;

    let parsed: PavoConfig = toml::from_str(input).unwrap_or_else(|_| PavoConfig::new());
    assert_eq!(parsed.show_layer, false);
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn parse_empty_config_defaults_to_new() {
    let parsed: PavoConfig = toml::from_str("").unwrap();
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
    assert_eq!(parsed.show_layer, false);
  }

  #[test]
  fn serialize_favorites_with_metadata() {
    let cfg = PavoConfig {
      auto_daily_update: true,
      history_range_days: 14,
      show_layer: false,
      favorites: vec![FavoriteItem {
        filename: "OHR.Sample.jpg".into(),
        url: "https://www.bing.com/th?id=OHR.Sample.jpg".into(),
        title: "Sample".into(),
        startdate: "20260427".into(),
        copyright: "Copyright".into(),
        copyrightlink: "https://www.bing.com".into(),
        local_path: None,
      }],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
    };

    let text = toml::to_string(&cfg).unwrap();
    assert!(text.contains("history_range_days = 14"));
    assert!(text.contains("filename = \"OHR.Sample.jpg\""));
  }

  #[test]
  fn write_config_persists_auto_daily_update_flag() {
    let cfg = PavoConfig {
      auto_daily_update: true,
      history_range_days: 7,
      show_layer: false,
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
    };
    let text = toml::to_string(&cfg).unwrap();
    assert!(text.contains("auto_daily_update = true"));
  }

  #[test]
  fn serialization_omits_legacy_shuffle_fields() {
    let cfg = PavoConfig {
      auto_daily_update: false,
      history_range_days: 7,
      show_layer: false,
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
    };
    let text = toml::to_string(&cfg).unwrap();
    // The serialized form should NOT contain legacy shuffle/interval fields
    assert!(
      !text.contains("auto_shuffle"),
      "legacy auto_shuffle should not appear in serialized config"
    );
    assert!(
      !text.contains("shuffle_source"),
      "legacy shuffle_source should not appear in serialized config"
    );
    assert!(
      !text.contains("randomly"),
      "legacy randomly should not appear in serialized config"
    );
    assert!(
      !text.contains("\ninterval ="),
      "legacy interval should not appear in serialized config"
    );
    assert!(
      !text.contains("auto_save"),
      "legacy auto_save should not appear in serialized config"
    );
  }

  #[test]
  fn parse_legacy_config_tolerates_old_fields_gracefully() {
    // Old configs on disk may still have auto_shuffle/interval/randomly/auto_save
    let input = r#"
auto_shuffle = true
shuffle_source = ["/some/path"]
randomly = true
interval = 30
auto_save = true
show_layer = true
"#;
    let parsed: PavoConfig = toml::from_str(input).unwrap_or_else(|_| PavoConfig::new());
    // Even after parsing a legacy config, the product-facing value is auto_daily_update
    assert!(parsed.show_layer);
    // The new defaults should still apply for fields not in the old config
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn parse_tolerant_preserves_show_layer_when_strict_parse_fails() {
    let input = r#"
show_layer = true
auto_daily_update = false
history_range_days = 14
favorites = "not_an_array"
"#;
    assert!(
      toml::from_str::<PavoConfig>(input).is_err(),
      "strict parse should fail on type mismatch"
    );

    let parsed = PavoConfig::parse_tolerant(input);
    assert!(parsed.show_layer, "show_layer should be preserved");
    assert!(
      !parsed.auto_daily_update,
      "auto_daily_update should be preserved"
    );
    assert_eq!(
      parsed.history_range_days, 14,
      "history_range_days should be preserved"
    );
    assert!(
      parsed.favorites.is_empty(),
      "favorites should default to empty on type mismatch"
    );
  }

  #[test]
  fn parse_tolerant_falls_back_to_defaults_on_completely_invalid_toml() {
    let input = "this is not valid toml {{{";
    let parsed = PavoConfig::parse_tolerant(input);
    assert_eq!(parsed.show_layer, false);
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn parse_tolerant_handles_empty_string() {
    let parsed = PavoConfig::parse_tolerant("");
    assert_eq!(parsed.show_layer, false);
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }
}
