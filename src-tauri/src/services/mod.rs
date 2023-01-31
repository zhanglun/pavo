use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{Seek, Write};
use std::cmp::min;
use futures_util::StreamExt;
use reqwest::Client;

pub mod bing;
pub mod pexels;
pub mod mock;

#[derive(Debug, Serialize, Deserialize)]
pub enum PhotoService {
  Bing,
  Pexels,
  Unsplash,
}

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
