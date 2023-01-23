use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use weblog::*;
use yew::prelude::*;

use crate::components::toolbar::{Toolbar, PhotoService};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[warn(dead_code)]
pub struct Wallpaper {
  title: String,
  href: String,
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
  pub copyright: String,
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
      copyright: String::from(&ctx.props().copyright),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="relative">
        <div class="relative overflow-hidden rounded-2xl group">
          <img
            class="w-full group-hover:scale-125 transition-all"
            src={ctx.props().href.clone()}
            title={ctx.props().title.clone()}
          />
          <div class="
            p-2 bg-gradient-to-r from-black/50 to-black/0
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
            <div class="text-base font-semibold">{ctx.props().title.clone()}</div>
            <div class="text-sm">{ctx.props().copyright.clone()}</div>
          </div>
          <Toolbar
            href={ctx.props().href.clone()}
            service={PhotoService::Bing}
          />
        </div>
      </div>
    }
  }
}
