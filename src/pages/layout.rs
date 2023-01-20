use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Home,
  #[at("/unsplash")]
  Unsplash,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! {<Home /> },
    Route::Unsplash => html! {
      <h1>{"Unsplsh"}</h1>
    },
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
}


#[function_component(Layout)]
pub fn layout (props: &Props) -> Html {
  html! {
    <div class="w-full h-screen grid gap-2 grid-rows-[58px_1fr]">
      <div class="flex justify-center pt-2 sticky top-0 z-10">
        <nav class="p-1 flex items-center justify-start rounded-full bg-white shadow-md">
          <a class="inline-block text-center text-base text-black hover:text-black hover:bg-slate-100 px-3 py-2 mx-1 rounded-full" href={"/"}>{"Home"}</a>
          <a class="inline-block text-center text-base text-black hover:text-black hover:bg-slate-100 px-3 py-2 mx-1 rounded-full" href={"/unsplash"}>{"Unsplash"}</a>
        </nav>
      </div>

      <div class="">
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </div>
    </div>
  }
}
