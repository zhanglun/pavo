use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::io::{Write};
use std::fs::File;

use crate::config;

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

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<String, String> {
  let res = client
    .get(url)
    .send()
    .await
    .map_err(|e| format!("Failed to GET from '{}': {}", url, e))?;
  let total_size = res
    .content_length()
    .ok_or_else(|| format!("Server did not provide Content-Length for '{}'", url))?;

  // let pb = ProgressBar::new(total_size);
  // pb.set_style(ProgressStyle::default_bar()
  //   .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
  //   .progress_chars("█  "));
  // pb.set_message(&format!("Downloading {}", url));

  let mut file;
  let mut downloaded: u64 = 0;
  let mut stream = res.bytes_stream();

  println!("Seeking in file.");

  if std::path::Path::new(path).exists() {
    println!("File exists. Resuming.");

    file = std::fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .map_err(|e| {
        format!(
          "Failed to open existing file for appending '{}': {}",
          path, e
        )
      })?;

    // let file_size = std::fs::metadata(path)?.len();

    // file.seek(std::io::SeekFrom::Start(file_size))?;
    // downloaded = file_size;
  } else {
    println!("Fresh file..");

    file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
  }

  println!("Commencing transfer");

  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;

    file
      .write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
    // let new = min(downloaded + (chunk.len() as u64), total_size);
    // downloaded = new;
    // pb.set_position(new);
  }

  // pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
  println!("Downloaded ==> {:?} to {:?}", url, path);
  return Ok(path.to_string());
}

pub async fn save_wallpaper(url: &str, filename: &str) -> Result<String, String> {
  let app_folder = config::PavoConfig::get_app_folder().unwrap();
  let path = Path::new(&app_folder).join(&*filename);
  let res = download_file(&Client::new(), &url, path.clone().to_str().unwrap()).await;

  match res {
    Ok(a) => Ok(a),
    Err(e) => Err(e.to_string()),

  }
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
  #[tokio::test]

  async fn it_works() {
    let url = "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160&idx=0&n=8&mkt=fr-FR";
    let result = save_wallpaper(&url).await.unwrap();
    assert!(Path::new(&result).exists());
  }
}

