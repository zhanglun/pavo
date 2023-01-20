
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children, // the field name `children` is important!
}


#[function_component(Layout)]
pub fn layout (props: &Props) -> Html {
  html! {
    <div class="w-full h-screen grid grid-rows-[50px_1fr]">
      <div class="bg-red-200">

      </div>
      <div class="bg-slate-400">
        { for props.children.iter() }
      </div>
    </div>
  }
}
