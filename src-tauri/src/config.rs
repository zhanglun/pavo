use std::sync::LazyLock;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::Mutex as StdMutex;

static CONFIG_LOCK: LazyLock<StdMutex<()>> = LazyLock::new(|| StdMutex::new(()));

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

fn default_auto_start() -> bool {
  false
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  #[serde(default = "default_auto_daily_update")]
  pub auto_daily_update: bool,
  #[serde(default = "default_history_range_days")]
  pub history_range_days: u8,
  #[serde(default)]
  pub favorites: Vec<FavoriteItem>,
  #[serde(default = "default_auto_rotate")]
  pub auto_rotate: bool,
  #[serde(default = "default_rotate_interval_minutes")]
  pub rotate_interval_minutes: u16,
  #[serde(default = "default_rotate_mode")]
  pub rotate_mode: RotateMode,
  #[serde(default = "default_auto_start")]
  pub auto_start: bool,
}

impl PavoConfig {
  pub fn new() -> Self {
    Self {
      auto_daily_update: true,
      history_range_days: 7,
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
      auto_start: false,
    }
  }

  pub fn create_app_folder() -> Result<String, Error> {
    let home_dir = dirs::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".pavo");

        match fs::create_dir_all(app_config_dir.clone()) {
          Ok(_) => app_config_dir.to_str().map(|s| s.to_string()).ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "home path contains non-UTF-8 characters")
          }),
          Err(e) => Err(e),
        }
      }
      None => Err(Error::new(ErrorKind::NotFound, "home dir is not fount")),
    }
  }

  pub fn get_app_folder() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("home directory not found".to_string())?;
    let app_config_dir = Path::new(&home_dir).join(".pavo");

    if app_config_dir.exists() {
      app_config_dir.to_str().map(|s| s.to_string()).ok_or_else(|| {
        "home path contains non-UTF-8 characters".to_string()
      })
    } else {
      Self::create_app_folder().map_err(|e| e.to_string())
    }
  }

  pub fn write_config(data: PavoConfig) -> Result<(), String> {
    let folder_dir = Self::get_app_folder().map_err(|e| e)?;
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
      auto_start: table
        .get("auto_start")
        .and_then(|v| v.as_bool())
        .unwrap_or(false),
    }
  }

  pub fn get_config() -> Self {
    let folder_dir = match Self::get_app_folder() {
      Ok(dir) => dir,
      Err(e) => {
        log::error!("Failed to get app folder: {}", e);
        return Self::new();
      }
    };
    let file_path = Path::new(&folder_dir).join("pavo.toml");

    if !file_path.exists() {
      if let Err(e) = fs::File::create(&file_path).map_err(|e| e.to_string()) {
        log::error!("Failed to create config file: {}", e);
      }
      return Self::new();
    }

    let content = fs::read_to_string(&file_path).unwrap_or_default();
    Self::parse_tolerant(&content)
  }

  fn update_config(modifier: impl FnOnce(&mut PavoConfig)) -> Self {
    let _guard = match CONFIG_LOCK.lock() {
      Ok(g) => g,
      Err(e) => {
        log::error!("Config lock poisoned: {}", e);
        return Self::get_config();
      }
    };
    let mut data = Self::get_config();
    modifier(&mut data);
    if let Err(e) = Self::write_config(data.clone()) {
      log::error!("Failed to write config: {}", e);
    }
    data
  }

  pub fn set_auto_daily_update(&self, enabled: bool) -> Self {
    Self::update_config(|cfg| cfg.auto_daily_update = enabled)
  }

  pub fn set_history_range_days(&self, days: u8) -> Self {
    Self::update_config(|cfg| cfg.history_range_days = days)
  }

  pub fn add_favorite(&self, item: FavoriteItem) -> Self {
    Self::update_config(|cfg| {
      if !cfg.favorites.iter().any(|f| f.filename == item.filename) {
        cfg.favorites.push(item);
      }
    })
  }

  pub fn remove_favorite_by_filename(&self, filename: &str) -> Self {
    Self::update_config(|cfg| {
      cfg.favorites.retain(|f| f.filename != filename);
    })
  }

  pub fn set_auto_rotate(&self, enabled: bool) -> Self {
    Self::update_config(|cfg| cfg.auto_rotate = enabled)
  }

  pub fn set_rotate_interval(&self, minutes: u16) -> Self {
    Self::update_config(|cfg| cfg.rotate_interval_minutes = minutes)
  }

  pub fn set_rotate_mode(&self, mode: RotateMode) -> Self {
    Self::update_config(|cfg| cfg.rotate_mode = mode)
  }

  pub fn set_auto_start(&self, enabled: bool) -> Self {
    Self::update_config(|cfg| cfg.auto_start = enabled)
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
  }

  #[test]
  fn serialize_favorites_with_metadata() {
    let cfg = PavoConfig {
      auto_daily_update: true,
      history_range_days: 14,
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
      auto_start: false,
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
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
      auto_start: false,
    };
    let text = toml::to_string(&cfg).unwrap();
    assert!(text.contains("auto_daily_update = true"));
  }

  #[test]
  fn serialization_omits_legacy_shuffle_fields() {
    let cfg = PavoConfig {
      auto_daily_update: false,
      history_range_days: 7,
      favorites: vec![],
      auto_rotate: false,
      rotate_interval_minutes: 60,
      rotate_mode: RotateMode::Sequential,
      auto_start: false,
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
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn parse_tolerant_preserves_fields_when_strict_parse_fails() {
    let input = r#"
auto_daily_update = false
history_range_days = 14
favorites = "not_an_array"
"#;
    assert!(
      toml::from_str::<PavoConfig>(input).is_err(),
      "strict parse should fail on type mismatch"
    );

    let parsed = PavoConfig::parse_tolerant(input);
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
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn parse_tolerant_handles_empty_string() {
    let parsed = PavoConfig::parse_tolerant("");
    assert_eq!(parsed.auto_daily_update, true);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }
}
