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

pub async fn download_file(
  client: &Client,
  url: &str,
  path: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
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

  println!(
    "Downloading {} ({} bytes) to {}",
    url,
    total_size,
    path.display()
  );

  let mut file = create_or_truncate_file(path).await?;

  let mut stream = res.bytes_stream();
  let mut downloaded: u64 = 0;

  println!("Starting download...");

  while let Some(chunk_result) = stream.next().await {
    let chunk = chunk_result.map_err(|e| format!("Error while downloading chunk: {}", e))?;

    let chunk_size = chunk.len() as u64;

    file
      .write_all(&chunk)
      .await
      .map_err(|e| format!("Error while writing to file: {}", e))?;

    downloaded += chunk_size;

    if total_size > 0 {
      let percent = (downloaded as f64 / total_size as f64) * 100.0;
      if downloaded % (1024 * 1024) < chunk_size {
        println!("Progress: {:.1}% ({}/{})", percent, downloaded, total_size);
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

  println!("Successfully downloaded {} to {}", url, path.display());

  Ok(path.to_string_lossy().into_owned())
}

pub fn view_photo(handle: tauri::AppHandle, href: String) {
  let _label = href.clone();
  let label = "view_photo";

  println!("{:?}", label);

  let _view_window = tauri::webview::WebviewWindowBuilder::new(
    &handle,
    label,
    tauri::WebviewUrl::External(href.parse().unwrap()),
  )
  .build()
  .unwrap();

  println!("{:?} ", href);
}

#[cfg(test)]
mod tests {
  use crate::services::bing;
  use std::path::Path;

  #[tokio::test]

  async fn it_works() {
    let url = "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160&idx=0&n=8&mkt=fr-FR";
    let result = bing::Wallpaper::save_wallpaper(&url, None).await.unwrap();
    assert!(Path::new(&result).exists());
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
