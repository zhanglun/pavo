use yew::prelude::*;

pub struct Wallpaper {
  title: String,
  href: String,
}

pub enum Msg {
  Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub title: String,
  pub href: String,
}

impl Component for Wallpaper {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      title: String::from(&ctx.props().title),
      href: String::from(&ctx.props().href),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="text-[red]">
        <img class="w-full" src={ctx.props().href.clone()} title={ctx.props().title.clone()} />
        <div class="flex justify-between items-center p-2">
          <span class="px-2 py-1 border border-rose-600 rounded">{"set"}</span>
          <span class="px-2 py-1 border border-rose-600 rounded">{"download"}</span>
          <span class="px-2 py-1 border border-rose-600 rounded">{"open"}</span>
        </div>
      </div>
    }
  }
}
