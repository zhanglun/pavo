use crate::config::RotateMode;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

pub mod bing;
#[derive(Debug, Serialize, Deserialize)]
pub enum PhotoService {
  Bing,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AsyncProcessMessage {
  StartDailyUpdate,
  StopDailyUpdate,
  PreviousPhoto,
  NextPhoto,
  StartRotation,
  StopRotation,
  UpdateRotationConfig {
    interval_minutes: u16,
    mode: RotateMode,
  },
}

pub async fn create_or_truncate_file(
  path: &Path,
) -> Result<File, Box<dyn std::error::Error + Send + Sync>> {
  OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)
    .await
    .map_err(|e| format!("Failed to open file '{}': {}", path.display(), e).into())
}

pub fn download_percent(downloaded: u64, total: u64) -> u8 {
  if total == 0 {
    return 0;
  }
  ((downloaded.saturating_mul(100) / total).min(100)) as u8
}

pub async fn download_file_with_progress<F>(
  client: &Client,
  url: &str,
  path: &str,
  mut on_progress: F,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
where
  F: FnMut(u8) + Send,
{
  let path = Path::new(path);

  let res = client
    .get(url)
    .send()
    .await
    .map_err(|e| format!("Failed to GET from '{}': {}", url, e))?;

  if !res.status().is_success() {
    return Err(format!("HTTP error: {} for URL: {}", res.status(), url).into());
  }

  let total_size = res
    .content_length()
    .ok_or_else(|| format!("Server did not provide Content-Length for '{}'", url))?;

  log::info!(
    "Downloading {} ({} bytes) to {}",
    url,
    total_size,
    path.display()
  );

  let mut file = create_or_truncate_file(path).await?;

  let mut stream = res.bytes_stream();
  let mut downloaded: u64 = 0;
  let mut reported_percent = 0;

  log::debug!("Starting download...");
  on_progress(0);

  while let Some(chunk_result) = stream.next().await {
    let chunk = chunk_result.map_err(|e| format!("Error while downloading chunk: {}", e))?;

    let chunk_size = chunk.len() as u64;

    file
      .write_all(&chunk)
      .await
      .map_err(|e| format!("Error while writing to file: {}", e))?;

    downloaded += chunk_size;

    if total_size > 0 {
      let percent = download_percent(downloaded, total_size);
      if percent != reported_percent {
        reported_percent = percent;
        on_progress(percent);
      }
      if downloaded % (1024 * 1024) < chunk_size {
        log::debug!("Progress: {}% ({}/{})", percent, downloaded, total_size);
      }
    }
  }

  if downloaded != total_size {
    return Err(
      format!(
        "Download incomplete: expected {} bytes, got {} bytes",
        total_size, downloaded
      )
      .into(),
    );
  }

  log::info!("Successfully downloaded {} to {}", url, path.display());

  Ok(path.to_string_lossy().into_owned())
}

pub fn view_photo(handle: tauri::AppHandle, href: String) {
  let label = format!("view_{}", rand::random::<u32>());

  // Only allow https:// URLs to prevent file:// or javascript: scheme attacks
  if !href.starts_with("https://") {
    log::error!("view_photo rejected non-https URL: {}", href);
    return;
  }

  let url = match href.parse() {
    Ok(u) => u,
    Err(e) => {
      log::error!("view_photo failed to parse URL '{}': {}", href, e);
      return;
    }
  };

  if let Err(e) = tauri::webview::WebviewWindowBuilder::new(
    &handle,
    &label,
    tauri::WebviewUrl::External(url),
  )
  .build()
  {
    log::error!("view_photo failed to create window '{}': {}", label, e);
  }
}

#[cfg(test)]
mod tests {
  use crate::services::bing;

  // 原 it_works 测试依赖 Bing 实时接口（法语地区偶发返回不含 OHR 的数据），
  // 测试结果由外部网络决定，不稳定。改为对纯函数 get_filename 的固定 fixture
  // 测试，覆盖成功解析与错误分支，不再触网。

  #[test]
  fn get_filename_extracts_ohr_name_from_image_url() {
    let url = "https://www.bing.com/th?id=OHR.DolomitesDawn_FR-FR1234567890_UHD.jpg&rf=LaDigue_UHD.jpg&pid=hp";
    let filename = bing::Images::get_filename(url).expect("OHR 图片直链应解析成功");
    assert_eq!(filename, "OHR.DolomitesDawn_FR-FR1234567890_UHD.jpg");
  }

  #[test]
  fn get_filename_rejects_url_without_ohr_segment() {
    // 列表接口 URL（HPImageArchive.aspx）不含 OHR 片段，
    // 正是旧 it_works 测试在法语地区失败的根因；此处断言它会返回错误。
    let url = "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160&idx=0&n=8&mkt=fr-FR";
    let result = bing::Images::get_filename(url);
    assert!(
      result.is_err(),
      "不含 OHR 片段的 URL 应返回错误，实际得到: {:?}",
      result
    );
  }

  #[test]
  fn favorite_metadata_type_keeps_local_path_optional() {
    let item = crate::config::FavoriteItem {
      filename: "demo".into(),
      url: "https://example.com/demo".into(),
      title: "Demo".into(),
      startdate: "20260427".into(),
      copyright: "Copyright".into(),
      copyrightlink: "https://www.bing.com".into(),
      local_path: None,
    };
    assert!(item.local_path.is_none());
  }

  #[test]
  fn download_progress_is_a_capped_integer_percentage() {
    assert_eq!(super::download_percent(42, 100), 42);
    assert_eq!(super::download_percent(101, 100), 100);
    assert_eq!(super::download_percent(0, 0), 0);
  }

  #[test]
  fn wallpaper_paths_match_after_canonicalization() {
    let dir = std::env::temp_dir().join("pavo_test_wallpaper_path");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("wallpaper.jpg");
    std::fs::write(&path, b"image").unwrap();
    let equivalent = dir.join(".").join("wallpaper.jpg");

    assert!(crate::services::bing::wallpaper_paths_match(
      &path.to_string_lossy(),
      &equivalent.to_string_lossy(),
    ));
    assert!(!crate::services::bing::wallpaper_paths_match(
      &path.to_string_lossy(),
      &dir.join("other.jpg").to_string_lossy(),
    ));

    let _ = std::fs::remove_dir_all(&dir);
  }

  #[tokio::test]
  async fn create_or_truncate_file_overwrites_existing_content() {
    let dir = std::env::temp_dir().join("pavo_test_overwrite");
    let _ = tokio::fs::create_dir_all(&dir).await;
    let path = dir.join("test_overwrite.jpg");

    tokio::fs::write(&path, b"old content that should be replaced")
      .await
      .unwrap();
    assert_eq!(
      tokio::fs::read(&path).await.unwrap().len(),
      35,
      "precondition: old content length"
    );

    let mut file = super::create_or_truncate_file(&path)
      .await
      .expect("create_or_truncate_file should succeed");
    use tokio::io::AsyncWriteExt;
    file.write_all(b"new").await.unwrap();
    file.flush().await.unwrap();

    let content = tokio::fs::read(&path).await.unwrap();
    assert_eq!(
      content, b"new",
      "file should contain only 'new', not 'old content...new'"
    );

    let _ = tokio::fs::remove_dir_all(&dir).await;
  }
}
