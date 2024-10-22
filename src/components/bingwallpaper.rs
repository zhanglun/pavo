use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::toolbar::{PhotoService, Toolbar};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[warn(dead_code)]
pub struct Wallpaper {
  title: String,
  href: String,
  startdate: String,
  copyright: String,
}

pub enum Msg {
  Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub title: String,
  pub href: String,
  pub startdate: String,
  pub copyright: String,
  pub copyrightlink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewPhotoParams {
  href: String,
}

#[derive(Serialize, Deserialize)]
struct SetAsDesktopArgs<'a> {
  url: &'a str,
  service: PhotoService,
}

impl Component for Wallpaper {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      title: String::from(&ctx.props().title),
      href: String::from(&ctx.props().href),
      startdate: String::from(&ctx.props().startdate),
      copyright: String::from(&ctx.props().copyright),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Clicked => {
        {
          let href = ctx.props().href.clone();

          spawn_local(async move {
            let href = href.clone();
            invoke(
              "view_photo",
              to_value(&ViewPhotoParams { href: href }).unwrap(),
            )
            .await;
          });
        }

        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let view_photo = ctx.link().callback(|_| Msg::Clicked);

    html! {
      <div class="relative cursor-pointer">
        <div class="relative rounded-2xl overflow-hidden group w-full aspect-[1.78] bg-medirian-1">
          <img
            class="w-full rounded-2xl transition-all group-hover:scale-125"
            src={ctx.props().href.clone()}
            title={ctx.props().title.clone()}
            onclick={view_photo}
          />
          <div class="
            p-2 bg-gradient-to-r from-black/50 to-black/0
            rounded-2xl
            overflow-hidden
            absolute left-0 right-0 bottom-0 top-0
          ">
          </div>
        </div>
        <div class="
          grid gap-2 mt-4
          grid-flow-row
          sm:grid-flow-col
          lg:grid-flow-row
          xl:grid-flow-col
        ">
          <div class="grid gap-2">
            <div class="flex justify-between items-center">
              <div class="text-base grid gap-2 grid-flow-col">
                <span class="font-semibold">{ctx.props().title.clone()}</span>
                <span class="font-normal">{ctx.props().startdate.clone()}</span>
              </div>
              <Toolbar
                service={PhotoService::Bing}
                url={ctx.props().href.clone()}
                href={ctx.props().copyrightlink.clone()}
              />
            </div>
            <div class="text-sm">{ctx.props().copyright.clone()}</div>
          </div>
        </div>
      </div>
    }
  }
}
