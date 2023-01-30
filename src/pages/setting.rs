use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::layout::Route;

#[function_component(Setting)]
pub fn setting () -> Html {
  let navigator = use_navigator().unwrap();
  let handle_back = Callback::from(move |_| navigator.push(&Route::Home));

  let interval_options = vec![
    ("0.5h", "Every 30 Minutes"),
    ("1h", "Every Hour"),
    ("2h", "Every 2 Hours"),
    ("5h", "Every 5 Hours"),
    ("24h", "Every Day"),
  ];

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
            <input type="checkbox" />
            {"Auto Rotate"}
            </label>
          </div>
          <div>
            {"Change picture"}
            <label id="">
            <select>
              <option value="volvo">Volvo</option>
              <option value="saab">Saab</option>
              <option value="mercedes">Mercedes</option>
              <option value="audi">Audi</option>
            </select>
            </label>
          </div>
          <div>
            <label id="random">
            <input type="checkbox" />
            {"Randomly"}
            </label>
          </div>
        </div>
      </div>
    </div>
  }
}
