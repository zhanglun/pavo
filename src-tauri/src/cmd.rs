use std::cmp::min;
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::path::Path;
use wallpaper;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

use crate::{config, services};
use crate::services::bing;
use crate::services::pexels;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<String, String> {
  let res = client
    .get(url)
    .send()
    .await
    .or(Err(format!("Failed to GET from '{}'", &url)))?;
  let total_size = res
    .content_length()
    .ok_or(format!("Failed to get content length from '{}'", &url))?;

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
      .read(true)
      .append(true)
      .open(path)
      .unwrap();

    let file_size = std::fs::metadata(path).unwrap().len();

    file.seek(std::io::SeekFrom::Start(file_size)).unwrap();
    downloaded = file_size;
  } else {
    println!("Fresh file..");

    file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
  }

  println!("Commencing transfer");

  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;

    file.write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
    let new = min(downloaded + (chunk.len() as u64), total_size);
    downloaded = new;
    // pb.set_position(new);
  }

  // pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
  println!("Downloaded ==> {:?} to {:?}", url, path);
  return Ok(path.to_string());
}

#[tauri::command]
pub async fn set_as_desktop(url: &str) -> Result<String, String> {
  println!("set as {:?}", url);

  let app_folder = config::PavoConfig::get_app_folder();

  match app_folder {
    Ok(dir) => {
      let filename = bing::Images::get_filename(&url);
      let path = Path::new(&dir).join(&*filename);
      let a = download_file(&Client::new(), url, path.clone().to_str().unwrap()).await;

      match a {
        Ok(a) => {
          wallpaper::set_from_path(a.as_str()).unwrap();

          Ok(a)
        }
        Err(e) => {
          Err(e)
        }
      }
    }
    Err(e) => {
      let (num, msg) = e;

      Err(msg)
    }
  }
}

#[tauri::command]
pub async fn download(url: &str) -> Result<String, String> {
  let app_folder = config::PavoConfig::get_app_folder();

  match app_folder {
    Ok(dir) => {
      let filename = bing::Images::get_filename(&url);
      let path = Path::new(&dir).join(&*filename);
      let a = download_file(&Client::new(), url, path.clone().to_str().unwrap()).await;

      match a {
        Ok(a) => {
          Ok(a)
        }
        Err(e) => {
          Err(e)
        }
      }
    }
    Err(e) => {
      let (num, msg) = e;

      Err(msg)
    }
  }
}

#[tauri::command]
pub async fn get_bing_wallpaper_list() -> Result<bing::Wallpaper, String> {
  let bing = services::bing::Wallpaper::new(0, 12).await;

  match bing {
    Ok(bing) => {
      Ok(bing)
    }
    Err(_) => {
      Err("".to_string())
    }
  }
}

#[tauri::command]
pub async fn get_pexels_curated_photos() -> serde_json::Value {
  let pexels_client = pexels::Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
  let res = pexels_client.get_photo_curated(20, 1).await;

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_download() {
    let a = download_file(&Client::new(), "https://bing.com/th?id=OHR.SessileOaks_ZH-CN6385464274_1920x1080.jpg&qlt=100", "./aa.jpg").await.unwrap();

    println!("{:?}", a);
  }
}
