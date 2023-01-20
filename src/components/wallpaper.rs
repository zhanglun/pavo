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

pub struct Wallpaper {
  title: String,
  href: String,
  description: String,
}

pub enum Msg {
  Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub title: String,
  pub href: String,
  pub description: String,
}

#[derive(Serialize, Deserialize)]
struct SetAsDesktopArgs<'a> {
  url: &'a str,
}

impl Component for Wallpaper {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      title: String::from(&ctx.props().title),
      href: String::from(&ctx.props().href),
      description: String::from(&ctx.props().description),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let set_as_desktop = {
      let clone_url = ctx.props().href.clone().to_string();

      Callback::from(move |_| {
        let url = clone_url.clone();

        spawn_local(async move {
          console_log!(url.as_str());
          let res = invoke("set_as_desktop", to_value(&SetAsDesktopArgs { url: &*url }).unwrap()).await;
          console_log!(&*res.as_string().unwrap());
        })
      })
    };

    let download = {
      let clone_url = ctx.props().href.clone().to_string();

      Callback::from(move |_| {
        let url = clone_url.clone();

        console_log!("download -> ", &url);

        spawn_local(async move {
          console_log!(url.as_str());
          let res = invoke("download", to_value(&SetAsDesktopArgs { url: &*url }).unwrap()).await;
        })
      })
    };

    let open_in_browser = {
      Callback::from(move |_| {
        console_log!("open in browser");
      })
    };

    html! {
      <div class="relative overflow-hidden">
        <img class="w-full rounded-2xl" src={ctx.props().href.clone()} title={ctx.props().title
        .clone()} />
        <div class="flex justify-between items-center p-2 absolute left-0 right-0 bottom-0 bg-slate-600">
          <div class="text-slate-200">{ctx.props().title.clone()}</div>
          <div>
            <span class="px-2 py-1 border border-rose-600 rounded" onclick={set_as_desktop}>{"set"}</span>
            <span class="px-2 py-1 border border-rose-600 rounded" onclick={download}>{"download"}</span>
            <span class="px-2 py-1 border border-rose-600 rounded" onclick={open_in_browser}>{"open"}</span>
          </div>
        </div>
      </div>
    }
  }
}
