use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::wallpaper::Wallpaper;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;

  // #[wasm_bindgen(js_namespace = console)]
  // fn log(s: &str);
  //
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bingwallpaper {
  index: u8,
  number: u8,
  files: Vec<String>,
  json: Paper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paper {
  pub images: Vec<Images>,
  pub tooltips: Tooltips,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
  pub bot: i32,
  pub copyright: String,
  pub copyrightlink: String,
  pub drk: i32,
  pub enddate: String,
  pub fullstartdate: String,
  pub hs: Vec<String>,
  pub hsh: String,
  pub quiz: String,
  pub startdate: String,
  pub title: String,
  pub top: i32,
  pub url: String,
  pub urlbase: String,
  pub wp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tooltips {
  pub loading: String,
  pub next: String,
  pub previous: String,
  pub walle: String,
  pub walls: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[function_component(Home)]
pub fn home() -> Html {
  // let list: JsValue = invoke("get_bing_wallpaper_list", to_value("").unwrap()).await;
  // let list: Bingwallpaper = serde_wasm_bindgen::from_value(list).unwrap();

  let images = use_state(|| Vec::new());

  spawn_local(async move {
    let list: JsValue = invoke("get_bing_wallpaper_list", to_value("").unwrap()).await;
    let bing: Bingwallpaper = serde_wasm_bindgen::from_value(list).unwrap();

    images.set(bing.json.images.clone());
  });

  fn loop_images () -> Html {
    { for image in images.into_iter() {
      html! {
            <div>{"hhhhhh---"}</div>
         }
    }
  }

  html! {
    <div>
      {"Home"}


      <div class="grid grid-cols-3 gap-4">
        <Wallpaper href="https://bing.com/th?id=OHR.SessileOaks_ZH-CN6385464274_1920x1080.jpg&qlt=100" title="Self" />
        <Wallpaper href="https://bing.com/th?id=OHR.SessileOaks_ZH-CN6385464274_1920x1080.jpg&qlt=100" title="Self" />
        <Wallpaper href="https://bing.com/th?id=OHR.SessileOaks_ZH-CN6385464274_1920x1080.jpg&qlt=100" title="Self" />
        <Wallpaper href="https://bing.com/th?id=OHR.SessileOaks_ZH-CN6385464274_1920x1080.jpg&qlt=100" title="Self" />
      </div>
    </div>
  }
}
