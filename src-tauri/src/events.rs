use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperEvent {
  pub title: String,
  pub copyright: String,
  pub url: String,
  pub startdate: String,
}
