use wasm_bindgen::prelude::*;
use weblog::console_log;
use yew::prelude::*;

use crate::pages::layout::Layout;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
  let window = web_sys::window().unwrap();
  let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
  let theme = local_storage.get_item("theme").unwrap();
  let document = web_sys::window().unwrap().document().unwrap();

  console_log!(document);
  console_log!(local_storage);

  html! {
    <Layout>
    </Layout>
  }
}
