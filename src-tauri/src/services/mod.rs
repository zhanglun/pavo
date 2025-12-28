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
  StartShuffle,
  StopShuffle,
  PreviousPhoto,
  NextPhoto,
}

pub async fn download_file(
  client: &Client,
  url: &str,
  path: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  let path = Path::new(path);

  // 发送请求并检查状态
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

  // 创建或打开文件
  let mut file = if path.exists() {
    println!("File exists. Opening for appending.");
    OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .await
      .map_err(|e| format!("Failed to open existing file '{}': {}", path.display(), e))?
  } else {
    println!("Creating new file.");
    File::create(path)
      .await
      .map_err(|e| format!("Failed to create file '{}': {}", path.display(), e))?
  };

  // 使用流式下载
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

    // 可选：添加进度显示
    if total_size > 0 {
      let percent = (downloaded as f64 / total_size as f64) * 100.0;
      if downloaded % (1024 * 1024) < chunk_size {
        // 每约1MB打印一次进度
        println!("Progress: {:.1}% ({}/{})", percent, downloaded, total_size);
      }
    }
  }

  // 验证下载完整性
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
    let result = bing::Wallpaper::save_wallpaper(&url).await.unwrap();
    assert!(Path::new(&result).exists());
  }
}
