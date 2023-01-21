use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::pexels_photo::{Photo, PexelsPhoto};
use crate::pages::mock;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PexlesJSON {
  pub next_page: String,
  pub page: usize,
  pub per_page: usize,
  pub photos: Vec<Photo>,
  pub total_results: usize,
}

#[function_component(Pexels)]
pub fn pexels() -> Html {
  let photos: UseStateHandle<Vec<Photo>> = use_state(|| Vec::with_capacity(0));

  {
    let photos = photos.clone();

    use_effect_with_deps(move |_| {
      let photos = photos.clone();
      spawn_local(async move {
        // let list:JsValue = invoke("get_pexels_curated_photos", to_value("").unwrap()).await;
        // let res: PexlesJSON = serde_wasm_bindgen::from_value(list).unwrap();

        let res = mock::Mock::pexel_curated();
        photos.set(res.photos);
      });
      || ()
    }, ());
  }

  let photos = (*photos).clone();
  let photos = photos.iter().map(|item| {
    html! {
      <PexelsPhoto
        photo={item.clone()}
      />
    }
  }).collect::<Html>();

  html! {
    <div class="w-full p-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 lg:gap-8 xl:grid-cols-4">
        {photos}
      </div>
    </div>
  }
}
