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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[function_component(Home)]
pub fn home() -> Html {
  let list = use_state(|| "");

  use_effect(move || {
    spawn_local(async {
      let list: JsValue = invoke("get_bing_wallpaper_list", to_value("").unwrap()).await;
      // log(&list);
      log("hahahha");
    });
  });

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
