use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::setting::Setting;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/setting")]
  Setting,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <Home /> },
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
    <div class="w-full h-[100vh] grid gap-2 grid-cols-[58px_1fr]">
      <div class="w-full h-[100vh] border-r border-gray-50 dark:border-gray-400">
        <div class="fixed left-0 top-0 w-[58px] h-full bg-white dark:bg-[unset]">
          <nav class="h-full flex flex-col justify-between">
            <div class="flex flex-col">
              <div class="flex items-center justify-center p-2">
                <a
                  class="text-center text-base text-black hover:text-white dark:text-white hover:bg-medirian-2 p-2 rounded-lg"
                  href={"/"}
                >
                  <span class="w-6 h-6 block">
                    {"B"}
                  </span>
                </a>
              </div>
            </div>
            <div class="flex items-center justify-center p-2">
              <a class="text-base text-black hover:text-white dark:text-white hover:bg-medirian-2 p-2 rounded-lg" href={"/setting"}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z" />
                  <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </a>
            </div>
          </nav>
        </div>
      </div>
      <div class="overflow-auto">
        <BrowserRouter>
          <Switch<Route> render={switch} />
        </BrowserRouter>
      </div>
    </div>
  }
}
