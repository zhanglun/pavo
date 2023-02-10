use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::pexels::Pexels;
use crate::pages::setting::Setting;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/unsplash")]
  Unsplash,
  #[at("/pexels")]
  Pexels,
  #[at("/setting")]
  Setting,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <Home /> },
    Route::Unsplash => html! { <h1>{"Unsplsh"}</h1> },
    Route::Pexels => html! { <Pexels /> },
    Route::Setting => html! { <Setting /> },
    // s9GlfCrhK5qzYQTQjMipbIQ25spgFJnThF9n3uW73g9dge6eFzMJ7aeY
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(Layout)]
pub fn layout() -> Html {
  html! {
    <div class="w-full grid gap-2 grid-cols-[58px_1fr]">
      <div class="w-full h-[100vh] border-r">
        <div class="fixed left-0 top-0 w-[58px] h-full bg-white">
            <nav class="grid gap-1">
              <a class="text-center text-base text-black hover:text-black hover:bg-slate-100 py-2 mx-1"
                 href={"/"}
              >{"Bing"}</a>
              // <a class="text-center text-base text-black hover:text-black hover:bg-slate-100 py-2 mx-1" href={"/pexels"}>{"Pexels"}</a>
              <a class="text-center text-base text-black hover:text-black hover:bg-slate-100 py-2 mx-1" href={"/setting"}>{"Setting"}</a>
            </nav>
        </div>
      </div>
      <div class="">
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </div>
    </div>
  }
}
