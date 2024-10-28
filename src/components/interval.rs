use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name=invoke)]
  async fn invoke_get_config(cmd: &str) -> JsValue;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  auto_shuffle: bool,
  randomly: bool,
  interval: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntervalParams {
  interval: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Properties)]
pub struct Props {
  pub label: String,
  pub interval: usize,
}

#[function_component(IntervalItem)]
pub fn interval(props: &Props) -> Html {
  let ref_interval = use_node_ref();
  let state_interval: UseStateHandle<usize> = use_state(|| props.interval);
  let handle_interval = {
    let ref_interval = ref_interval.clone();
    let state_interval = state_interval.clone();

    Callback::from(move |_| {
      let select = ref_interval.cast::<HtmlInputElement>();
      if let Some(select) = select {
        let val = select.value().parse::<usize>().unwrap();

        console_log!(val);

        state_interval.set(val);
        spawn_local(async move {
          invoke(
            "set_interval",
            serde_wasm_bindgen::to_value(&IntervalParams { interval: val }).unwrap(),
          )
          .await;
        });
      }
    })
  };

  {
    let state_interval = state_interval.clone();
    let p = props.clone();

    use_effect_with_deps(
      move |_| {
        state_interval.set(p.interval);
      },
      (),
    );
  }

  html! {
    <div>
      <label for={props.interval.to_string()}>
        <input
          ref={ref_interval}
          type="radio"
          id={props.interval.to_string()}
          name={"interval"}
          value={props.interval.to_string()}
          onchange={handle_interval.clone()}
          checked={props.interval == *state_interval}/>
        {props.label.clone()}
      </label>
    </div>
  }
}
