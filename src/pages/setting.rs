use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::layout::Route;

#[function_component(Setting)]
pub fn setting () -> Html {
  let navigator = use_navigator().unwrap();
  let handle_back = Callback::from(move |_| navigator.push(&Route::Home));

  html! {
    <div>
      <div>
        <span onclick={handle_back}>{"Back"}</span>
      </div>
      {"Setting"}
    </div>
  }
}
