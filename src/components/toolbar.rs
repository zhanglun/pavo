use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[function_component(Toolbar)]
pub fn toolbar(props: &Poprs) -> Html {
    let set_as_desktop = {
      let clone_url = ctx.props().href.clone().to_string();

      Callback::from(move |_| {
        let url = clone_url.clone();

        spawn_local(async move {
          console_log!(url.as_str());
          let res = invoke("set_as_desktop", to_value(&SetAsDesktopArgs { url: &*url }).unwrap()).await;
          console_log!(&*res.as_string().unwrap());
        })
      })
    };

    let download = {
      let clone_url = ctx.props().href.clone().to_string();

      Callback::from(move |_| {
        let url = clone_url.clone();

        console_log!("download -> ", &url);

        spawn_local(async move {
          console_log!(url.as_str());
          let res = invoke("download", to_value(&SetAsDesktopArgs { url: &*url }).unwrap()).await;
        })
      })
    };

    let open_in_browser = {
      Callback::from(move |_| {
        console_log!("open in browser");
      })
    };

    html! {
      <div>
            <div class="
            grid gap-2 grid-flow-col

          ">
              <span
                class="w-4 h-4 py-1 text-center text-neutral-800 cursor-pointer transition-all origin-center hover:text-black hover:scale-110"
                onclick={set_as_desktop}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25" />
                </svg>
              </span>
              <span
                class="w-4 h-4 py-1 text-center text-neutral-800 cursor-pointer transition-all origin-center hover:text-black hover:scale-110"
                onclick={download}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                   fill="none"
                   viewBox="0 0 24 24"
                   stroke-width="1.5"
                   stroke="currentColor"
                   class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
                  </svg>
              </span>
              <span
                class="w-4 h-4 py-1 text-center text-neutral-800 cursor-pointer transition-all origin-center hover:text-black hover:scale-110"
                onclick={open_in_browser}
              >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
                </svg>
              </span>
            </div>
      </div>
    }
}
