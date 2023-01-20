use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::pages::layout::Layout;
use crate::pages::home::Home;

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

  // let greet = {
  //   let name = name.clone();
  //   let greet_input_ref = greet_input_ref.clone();
  //   Callback::from(move |_| {
  //     name.set(
  //       greet_input_ref
  //         .cast::<web_sys::HtmlInputElement>()
  //         .unwrap()
  //         .value(),
  //     );
  //   })
  // };

  html! {
    <Layout>
      <Home />
    </Layout>
  }
}
