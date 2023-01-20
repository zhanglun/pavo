use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::pages::layout::Layout;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <Layout>
    </Layout>
  }
}
