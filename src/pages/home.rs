use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::bingwallpaper::Wallpaper;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bingwallpaper {
  index: u8,
  number: u8,
  files: Vec<String>,
  json: Paper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltips {
  pub loading: String,
  pub next: String,
  pub previous: String,
  pub walle: String,
  pub walls: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingQuery {
  page: u8
}

#[function_component(Home)]
pub fn home() -> Html {
  let images: UseStateHandle<Vec<Images>> = use_state(|| Vec::with_capacity(0));

  {
    let _images = images.clone();

    use_effect_with_deps(move |_| {
      let images = _images.clone();

      spawn_local(async move {
        let list: JsValue = invoke("get_bing_wallpaper_list", to_value(&BingQuery { page: 0 }).unwrap()).await;
        let bing: Bingwallpaper = serde_wasm_bindgen::from_value(list).unwrap();

        let list2: JsValue = invoke("get_bing_wallpaper_list", to_value(&BingQuery { page: 1 }).unwrap()).await;
        let bing2: Bingwallpaper = serde_wasm_bindgen::from_value(list2).unwrap();

        let page1 = bing.json.images;
        let page2 = bing2.json.images;

        images.set(page1.into_iter().chain(page2.into_iter()).collect());
      });
      || ()
    }, ());
  }

  let images = (*images).clone();

  html! {
    <div class="w-full p-4">
      <div class="grid grid-cols-4 gap-4 lg:grid-cols-4 lg:gap-4">
        {images.iter().map(|item| {
          html! {
            <Wallpaper
              href={["https://www.bing.com", &item.url.clone()].concat()}
              title={item.title.clone()}
              copyright={item.copyright.clone()}
            />
          }
        }).collect::<Html>()}
      </div>
    </div>
  }
}
