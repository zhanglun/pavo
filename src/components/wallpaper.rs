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
        <img src={ctx.props().href.clone()} title={ctx.props().title.clone()} />
        {"This is a wallpaper"}
      </div>
    }
  }
}
