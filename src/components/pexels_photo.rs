use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::components::toolbar::{Toolbar, PhotoService};

pub enum Msg {
  Clicked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoSrcSet {
  pub original: String,
  pub large2x: String,
  pub large: String,
  pub medium: String,
  pub small: String,
  pub portrait: String,
  pub landscape: String,
  pub tiny: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
  pub id: usize,
  pub width: usize,
  pub height: usize,
  pub url: String,
  pub photographer: String,
  pub photographer_url: String,
  pub photographer_id: usize,
  pub avg_color: String,
  pub src: PhotoSrcSet,
  pub liked: bool,
  pub alt: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub photo: Photo,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct PexelsPhoto {
  pub photo: Photo,
}

impl Component for PexelsPhoto {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      photo: ctx.props().photo.clone(),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <div class="relative overflow-hidden rounded-2xl group w-full aspect-[0.67] bg-medirian-1">
          <img
            class="w-full max-h-full group-hover:scale-125 transition-all"
            src={ctx.props().photo.src.portrait.clone()}
            title={ctx.props().photo.alt.clone()}
          />
          <div class="
            absolute left-0 right-0 bottom-0 top-0 transition-all
            p-2 bg-gradient-to-r from-black/50 to-black/0
          ">
          </div>
        </div>
        <div class="
          grid gap-2 mt-4
          grid-flow-row
          sm:grid-flow-col
          lg:grid-flow-row
          xl:grid-flow-col
        ">
          <div class="grid gap-2">
            <div class="text-base font-semibold">{ctx.props().photo.alt.clone()}</div>
            <div class="text-sm">{ctx.props().photo.photographer.clone()}</div>
          </div>
          <Toolbar href={ctx.props().photo.src.original.clone()} service={PhotoService::Pexels} />
        </div>
      </div>
    }
  }
}
