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

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
  name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
  let greet_input_ref = use_node_ref();

  let name = use_state(|| String::new());

  let greet_msg = use_state(|| String::new());
  {
    let greet_msg = greet_msg.clone();
    let name = name.clone();
    let name2 = name.clone();
    use_effect_with_deps(
      move |_| {
        spawn_local(async move {
          if name.is_empty() {
            return;
          }

          // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
          let new_msg =
            invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
          log(&new_msg.as_string().unwrap());
          greet_msg.set(new_msg.as_string().unwrap());
        });

        || {}
      },
      name2,
    );
  }

  let greet = {
    let name = name.clone();
    let greet_input_ref = greet_input_ref.clone();
    Callback::from(move |_| {
      name.set(
        greet_input_ref
          .cast::<web_sys::HtmlInputElement>()
          .unwrap()
          .value(),
      );
    })
  };

  html! {
      <main class="container">
        <div class="row">
            <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
            <button type="button" onclick={greet}>{"Greetffff"}</button>
        </div>
        <div class="grid grid-cols-3 gap-4">
          <Wallpaper href="https://picsum.photos/seed/picsum/200/300?random=1" title="Self" />
          <Wallpaper href="https://picsum.photos/seed/picsum/200/300?random=2" title="Self" />
          <Wallpaper href="https://picsum.photos/seed/picsum/200/300?random=3" title="Self" />
          <Wallpaper href="https://picsum.photos/seed/picsum/200/300?random=4" title="Self" />
        </div>
    </main>
  }
}
