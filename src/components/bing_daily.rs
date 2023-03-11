use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::pages::home::{Images};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Param {}

#[function_component(BingDaily)]
pub fn bing_daily() -> Html {
  let daily_image: UseStateHandle<Images> = use_state(|| Images::default());

  {
    let daily_image = daily_image.clone();

    use_effect_with_deps(
      move |_| {
        let daily_image = daily_image.clone();

        spawn_local(async move {
          let res = invoke("get_bing_daily", to_value(&Param {}).unwrap()).await;
          let image: Images = serde_wasm_bindgen::from_value(res).unwrap();

          daily_image.set(image);
        });
        || ()
      },
      (),
    );
  }

  let daily_image = (*daily_image).clone();

  html! {
    <div class="w-full my-5">
      <h2 class="font-semibold text-2xl mt-2 mb-4">{"Today"}</h2>
      <div class="relative rounded-2xl overflow-hidden aspect-[1.78] bg-medirian-2">
        <img class="rounded-2xl" src={["https://www.bing.com", &daily_image.url.clone()].concat()} alt="" />
        <div class="absolute right-0 bottom-0 text-white bg-black bg-opacity-75 px-3 py-2 rounded-xl">
          <p classe="text-2xl">{daily_image.title.clone()}</p>
          <p class="text-sm">
            <a class="text-white hover:text-white link:text-white" href={daily_image.copyrightlink.clone()}>
            {daily_image.copyright.clone()}
            </a>
          </p>
        </div>
      </div>
    </div>
  }
}
