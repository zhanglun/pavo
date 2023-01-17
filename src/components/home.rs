use yew::prelude::*;

pub struct Home {
}

pub enum Msg {

}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {

}

impl Component for Home {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        {"Home"}
      </div>
    }
  }
}
