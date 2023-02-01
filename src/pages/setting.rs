use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement};
use weblog::console_log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::layout::Route;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name=invoke)]
  async fn invoke_get_config(cmd: &str) -> JsValue;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  auto_rotate: bool,
  randomly: bool,
  interval: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RotateParams {
  rotate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomParams {
  randomly: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntervalParams {
  interval: u8,
}

#[function_component(Setting)]
pub fn setting () -> Html {
  let navigator = use_navigator().unwrap();

  let handle_back = Callback::from(move |_| navigator.push(&Route::Home));

  let ref_auto_rotate = use_node_ref();
  let state_auto_rotate: UseStateHandle<bool> = use_state(|| false);
  let handle_auto_rotate = {
    let ref_auto_rotate = ref_auto_rotate.clone();
    let state_auto_rotate: UseStateHandle<bool> = state_auto_rotate.clone();

    Callback::from(move |_| {
      let checkbox = ref_auto_rotate.cast::<HtmlInputElement>();

      if let Some(checkbox) = checkbox {
        state_auto_rotate.set(checkbox.checked());
        spawn_local(async move {
          invoke("set_auto_rotate", serde_wasm_bindgen::to_value(&RotateParams { rotate: checkbox.checked()}).unwrap()).await;
        });
      }
    })
  };

  let ref_randomly = use_node_ref();
  let state_randomly: UseStateHandle<bool> = use_state(|| false);
  let handle_randomly = {
    let ref_randomly = ref_randomly.clone();
    let state_randomly = state_randomly.clone();

    Callback::from(move |_| {
      let checkbox = ref_randomly.cast::<HtmlInputElement>();

      if let Some(checkbox) = checkbox {
        state_randomly.set(checkbox.checked());
        spawn_local(async move {
          invoke("set_randomly", serde_wasm_bindgen::to_value(&RandomParams { randomly: checkbox.checked()}).unwrap()).await;
        });
      }
    })
  };

  let interval_options = vec![
    (30, "Every 30 Minutes"),
    (60, "Every Hour"),
    (120, "Every 2 Hours"),
    (300, "Every 5 Hours"),
    (3600, "Every Day"),
  ];
  let ref_interval = use_node_ref();
  let state_interval = use_state(|| 30);
  let handle_interval = {
    let ref_interval = ref_interval.clone();
    let state_interval = state_interval.clone();

    Callback::from(move |_| {
      let select = ref_interval.cast::<HtmlInputElement>();
      if let Some(select) = select {
        let val = select.value().parse::<u8>().unwrap();
        state_interval.set(val);
        spawn_local(async move {
          invoke("set_interval", serde_wasm_bindgen::to_value(&IntervalParams { interval:  val }).unwrap()).await;
        });
      }
    })
  };

  {
    let state_auto_rotate = state_auto_rotate.clone();
    let state_randomly = state_randomly.clone();
    let state_interval = state_interval.clone();

    use_effect_with_deps(move |_| {
      spawn_local(async move {
        let config = invoke_get_config("get_config").await;
        let config: PavoConfig = serde_wasm_bindgen::from_value(config).unwrap();

        console_log!(serde_wasm_bindgen::to_value(&config).unwrap());

        state_auto_rotate.set(config.auto_rotate);
        state_randomly.set(config.randomly);
        state_interval.set(config.interval);
      });
    }, ());
  }

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
              checked={*state_auto_rotate}
            />
            {"Auto Rotate"} {*state_auto_rotate}
            </label>
          </div>
          <div>
            {"Change picture"}
            <label id="">
            {*state_interval}
            <select onchange={handle_interval}>
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
              checked={*state_randomly}
            />
            {"Randomly"} {*state_randomly}
            </label>
          </div>
        </div>
      </div>
    </div>
  }
}
