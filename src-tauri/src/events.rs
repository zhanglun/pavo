use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperEvent {
  pub title: String,
  pub copyright: String,
  pub url: String,
  pub startdate: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWallpaperProgress {
  pub operation_id: String,
  pub phase: &'static str,
  pub percent: Option<u8>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWallpaperResult {
  pub confirmed: bool,
}
