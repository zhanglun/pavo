use std::path::Path;
use reqwest::Client;

use crate::config;
use super::download_file;

const API_URL: &'static str = "https://api.pexels.com/";

#[derive(Debug)]
pub struct Pexels {
  api_key: String,
  client: Client,
}

pub struct PhotoSrcSet {
  pub original: String,
  pub large2x: String,
  pub large: String,
  pub medium: String,
  pub small: String,
  pub portrait: String,
  pub landscape: String,
  pub tiny: String,
}

pub struct PexelsPhoto {
  pub id: usize,
  pub width: usize,
  pub height: usize,
  pub url: String,
  pub photographer: String,
  pub photographer_url: String,
  pub photographer_id: usize,
  pub avg_color: String,
  pub src: PhotoSrcSet,
  pub liked: bool,
  pub alt: String,
}

impl Pexels {
  pub fn new(api_key: String) -> Pexels {
    Pexels {
      api_key,
      client: Client::new(),
    }
  }

  pub async fn get(
    &self,
    endpoint: &str,
    param: Option<Vec<(&str, String)>>,
  ) -> Result<serde_json::Value, String> {
    let mut builder = self.client.get(&(API_URL.to_owned() + endpoint));

    builder = match param {
      Some(x) => builder.query(&x),
      None => builder,
    };

    let request = builder
      .header(reqwest::header::AUTHORIZATION, self.api_key.clone())
      .send()
      .await;
    // .expect(&format!("Failed to send request to {}", endpoint));

    match request {
      Ok(res) => {
        println!("{:?}", res);
        // Ok(serde_json::from_str("{}").unwrap())
        Ok(serde_json::from_str(&res.text().await.unwrap()).unwrap())
      }
      Err(e) => Err(String::from("Failed to read the response")),
    }
  }

  //   pub fn photo_detail(&self, id: String) -> ::serde_json::Value {
  //     self.get(&format!("{}{}", "v1/photos/", id), None).await
  //   }

  pub async fn get_photo_curated(&self, per_page: u32, page: u32) -> serde_json::Value {
    self.get(
      "v1/curated",
      Some(
        [
          ("per_page", per_page.to_string()),
          ("page", page.to_string()),
        ]
        .to_vec(),
      ),
    ).await.unwrap()
  }

  pub fn get_filename(url: &str) -> &str {
    let s = url.find("pexels-").ok_or(0).unwrap();
    let e = url.find("?").ok_or(0).unwrap();

    &url[s..e]
  }

  pub async fn save_photo(url: &str) -> Result<String, String> {
    let filename = Pexels::get_filename(url);
    let app_folder = config::PavoConfig::get_app_folder().unwrap();
    let path = Path::new(&app_folder).join(&*filename);
    let res = download_file(&Client::new(), &url, path.clone().to_str().unwrap()).await.unwrap();

    Ok(res)
  }

  pub async fn set_wallpaper(url: &str) -> Result<String, String> {
    let a = Pexels::save_photo(url).await;

    match a {
      Ok(a) => {
        wallpaper::set_from_path(a.as_str()).unwrap();

        Ok(String::from("OK"))
      }
      Err(e) => {
        Err(e.to_string().into())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test() {
    let pexels =
      Pexels::new("s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY".to_string());
    println!("{:?}", pexels);
    let a = pexels
      .get(
        "v1/curated",
        Some([("per_page", "10".to_string()), ("page", "1".to_string())].to_vec()),
      )
      .await
      .unwrap();

    println!("a: {:?}", a);
  }
}
