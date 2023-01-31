use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::layout::Route;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotateParams {
  autoRotate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomParams {
  randomly: bool,
}

#[function_component(Setting)]
pub fn setting () -> Html {
  let navigator = use_navigator().unwrap();

  let interval_options = vec![
    (30, "Every 30 Minutes"),
    (60, "Every Hour"),
    (120, "Every 2 Hours"),
    (300, "Every 5 Hours"),
    (3600, "Every Day"),
  ];

  let handle_back = Callback::from(move |_| navigator.push(&Route::Home));
  let ref_auto_rotate = use_node_ref();
  let state_auto_rotate: UseStateHandle<bool> = use_state(|| false);
  let value_auto_rotate = (*state_auto_rotate).clone();
  let handle_auto_rotate = {
    let ref_auto_rotate = ref_auto_rotate.clone();

    Callback::from(move |_| {
      let checkbox = ref_auto_rotate.cast::<HtmlInputElement>();

      if let Some(checkbox) = checkbox {
        state_auto_rotate.set(checkbox.checked());
        spawn_local(async move {
          invoke("set_auto_rotate", to_value(&RotateParams { autoRotate: checkbox.checked()}).unwrap()).await;
        });
      }
    })
  };

  let ref_randomly = use_node_ref();
  let state_randomly: UseStateHandle<bool> = use_state(|| false);
  let value_randomly = (*state_randomly).clone();
  let handle_randomly = {
    let ref_randomly = ref_randomly.clone();

    Callback::from(move |_| {
      let checkbox = ref_randomly.cast::<HtmlInputElement>();

      if let Some(checkbox) = checkbox {
        state_randomly.set(checkbox.checked());
        spawn_local(async move {
          invoke("set_randomly", to_value(&RandomParams { randomly: checkbox.checked()}).unwrap()).await;
        });
      }
    })
  };


  html! {
    <div>
      <div>
        <span onclick={handle_back}>{"Back"}</span>
      </div>
      {"Setting"}
      <div>
        <div>
          <div>
            <label id="rotate">
            <input
              ref={ref_auto_rotate}
              type="checkbox"
              onchange={handle_auto_rotate}
              checked={value_auto_rotate}
            />
            {"Auto Rotate"}
            </label>
          </div>
          <div>
            {"Change picture"}
            <label id="">
            <select>
              {
                interval_options.iter().map(|item| {
                  html! {
                  <option value={item.0.to_string()}>{item.1}</option>
                  }
                }).collect::<Html>()
              }
            </select>
            </label>
          </div>
          <div>
            <label id="random">
            <input
              ref={ref_randomly}
              type="checkbox"
              onchange={handle_randomly}
              checked={value_randomly}
            />
            {"Randomly"}
            </label>
          </div>
        </div>
      </div>
    </div>
  }
}
