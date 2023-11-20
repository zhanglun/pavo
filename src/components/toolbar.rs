use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use weblog::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SetAsDesktopArgs<'a> {
  url: &'a str,
  service: PhotoService,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PhotoService {
  Bing,
}

impl PhotoService {
  pub fn as_str(&self) -> &'static str {
    match self {
      PhotoService::Bing => "bing",
    }
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub url: String,
  pub href: String,
  pub service: PhotoService,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &Props) -> Html {
  let set_as_desktop = {
    let clone_url = props.url.clone().to_string();
    let service = props.service.clone();

    Callback::from(move |_| {
      let url = clone_url.clone();
      let service = service.clone();

      spawn_local(async move {
        console_log!(url.as_str());
        let res = invoke(
          "set_as_desktop",
          to_value(&SetAsDesktopArgs {
            url: &*url,
            service,
          })
          .unwrap(),
        )
        .await;
        console_log!(&*res.as_string().unwrap());
      })
    })
  };

  let download = {
    let clone_url = props.url.clone().to_string();
    let service = props.service.clone();

    Callback::from(move |_| {
      let url = clone_url.clone();
      let service = service.clone();

      spawn_local(async move {
        console_log!(url.as_str());
        let res = invoke(
          "download",
          to_value(&SetAsDesktopArgs {
            url: &*url,
            service,
          })
          .unwrap(),
        )
        .await;
      })
    })
  };

  html! {
    <div>
      <div class="
        grid gap-2 grid-flow-col
      ">
        <span
          class="
            py-1
            text-center
            cursor-pointer
            transition-[scale]
            origin-center
            text-neutral-800
            dark:text-white
            hover:text-black
            dark:hover:text-white
            hover:scale-110
          "
          onclick={set_as_desktop}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25" />
          </svg>
        </span>
        <span
          class="
            py-1
            text-center
            cursor-pointer
            transition-[scale]
            origin-center
            text-neutral-800
            dark:text-white
            hover:text-black
            dark:hover:text-white
            hover:scale-110
          "
          onclick={download}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-4 h-4">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
            </svg>
        </span>
        <a
          class="
            py-1
            text-center
            cursor-pointer
            transition-[scale]
            origin-center
            text-neutral-800
            dark:text-white
            hover:text-black
            dark:hover:text-white
            hover:scale-110
          "
          target="_blank"
          href={props.href.clone()}
        >
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
          </svg>
        </a>
      </div>
    </div>
  }
}
