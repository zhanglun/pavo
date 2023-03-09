use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::pexels_photo::{Photo, PexelsPhoto};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PexelQuery {
  page: u8
}

#[function_component(Pexels)]
pub fn pexels() -> Html {
  let page:UseStateHandle<u8> = use_state(|| 1);
  let photos: UseStateHandle<Vec<Photo>> = use_state(|| Vec::with_capacity(0));
  let get_next_page = {
    let page = page.clone();

    Callback::from(move |_| {
      page.set(*page + 1)
    })
  };

  {
    let photos = photos.clone();
    let page = page.clone();

    use_effect_with_deps(move |page| {
      let photos = photos.clone();
      let page = page.clone();

      spawn_local(async move {
        let list:JsValue = invoke("get_pexels_curated_photos", to_value(&PexelQuery { page: page }).unwrap()).await;
        let res: Vec<Photo> = serde_wasm_bindgen::from_value(list).unwrap();
        let mut r = vec![];

        r.append(&mut (*photos).clone());
        r.append(&mut res.clone());

        photos.set(r);
      });
      || ()
    }, *page);
  }

  let photos = (*photos).clone();
  let photos = photos.iter().map(|item| {
    html! {
      <PexelsPhoto
        key={item.clone().id}
        photo={item.clone()}
      />
    }
  }).collect::<Html>();

  html! {
    <div class="w-full p-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3 lg:grid-cols-5 lg:gap-8">
        {photos}
      </div>
      <div class="p-4 m-6 flex items-center justify-center">
        <div class="px-6 py-2
          rounded-full
          text-white
          cursor-pointer
          transition-all
          btn-grad
          bg-button-gradient"
          onclick={get_next_page}>{"Load More"}</div>
      </div>
    </div>
  }
}
