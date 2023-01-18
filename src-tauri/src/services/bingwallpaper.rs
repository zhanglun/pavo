use serde::{Serialize, Deserialize};
use serde_json::Number;
use std::{
  env::var,
  fs,
  io::{copy, Cursor},
  path::Path,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tooltips {
  pub loading: String,
  pub next: String,
  pub previous: String,
  pub walle: String,
  pub walls: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
  pub bot: Number,
  pub copyright: String,
  pub copyrightlink: String,
  pub drk: Number,
  pub enddate: String,
  pub fullstartdate: String,
  pub hs: Vec<String>,
  pub hsh: String,
  pub quiz: String,
  pub startdate: String,
  pub title: String,
  pub top: Number,
  pub url: String,
  pub urlbase: String,
  pub wp: bool,
}

impl Images {
  pub fn url(&self) -> String {
    ["https://www.bing.com", &self.url].concat()
  }

  fn filename(&self) -> &str {
    let s = self.url.find("OHR.").ok_or(0).unwrap();
    let e = self.url.find("&rf=").ok_or(0).unwrap();
    &self.url[s..e]
  }

  fn copyright(&self) -> &str {
    self.copyright.as_str()
  }

  fn directory(&self) -> String {
    [var("HOME").unwrap_or_default().as_str(), "/Pictures/Bing"].concat()
  }

  pub async fn save_wallpaper(&self) -> Result<()> {
    fs::create_dir_all(self.directory())?;
    let res = reqwest::get(self.url().as_str()).await?;
    copy(
      &mut Cursor::new(res.bytes().await?),
      &mut fs::File::create(Path::new(&self.directory()).join(self.filename()))?,
    )?;
    Ok(())
  }

  pub fn set_wallpaper(&self) {
    let file = format!("{}/{}", self.directory(), self.filename());
    println!("{}", file);
    println!("{}", self.copyright());
    wallpaper::set_from_path(&file).unwrap();
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Wallpaper {
  pub images: Vec<Images>,
  pub tooltips: Tooltips,
}

impl Wallpaper {
  pub async fn new(index: u8, number: u8) -> Result<Wallpaper> {
    Ok(reqwest::get(get_url(index, number).as_str())
      .await?
      .json::<Wallpaper>()
      .await?)
  }
}

const BING_URL: &str = "https://www.bing.com/HPImageArchive.aspx?&format=js";

#[derive(Debug, Serialize, Deserialize)]
pub struct Bingwallpaper {
  index: u8,
  number: u8,
  files: Vec<String>,
  json: Wallpaper,
}

impl Bingwallpaper {
  pub async fn new(index: u8, number: u8) -> Result<Bingwallpaper> {
    let json = Wallpaper::new(index, number).await?;
    Ok(Bingwallpaper {
      index,
      number,
      files: vec![],
      json,
    })
  }

  pub async fn save_wallpaper(&self) -> Result<()> {
    self.json.images[0].save_wallpaper().await?;
    Ok(())
  }

  pub fn set_wallpaper(&self) {
    self.json.images[0].set_wallpaper();
  }
}

fn get_url(index: u8, number: u8) -> String {
  [
    BING_URL,
    "&idx=",
    &index.to_string(),
    "&n=",
    &number.to_string(),
  ]
    .join("")
}

