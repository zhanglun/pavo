use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperEvent {
  pub title: String,
  pub copyright: String,
  pub url: String,
  pub startdate: String,
  /// 逐屏设置的屏幕标识；None 表示所有屏幕（统一模式）
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub monitor_id: Option<String>,
}
