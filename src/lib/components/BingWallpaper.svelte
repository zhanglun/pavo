<script>
  // your script goes here
  let { title, url, startdate = "hahah", copyright } = $props();

  function view_photo() {

  }

  </script>
  <div class="relative cursor-pointer">
    <div class="relative rounded-2xl overflow-hidden group w-full aspect-[1.78] bg-medirian-1">
      <img class="w-full rounded-2xl transition-all group-hover:scale-125" src={url} title={title}
        onclick={view_photo} />
      <div class="
            p-2 bg-gradient-to-r from-black/50 to-black/0
            rounded-2xl
            overflow-hidden
            absolute left-0 right-0 bottom-0 top-0
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
        <div class="flex justify-between items-center">
          <div class="text-base grid gap-2 grid-flow-col">
            <span class="font-semibold">{title}</span>
            <span class="font-normal">{startdate}</span>
          </div>
          <!--<Toolbar service={PhotoService::Bing} url={ctx.props().href.clone()}-->
          <!--  href={ctx.props().copyrightlink.clone()} />-->
        </div>
        <div class="text-sm">{copyright}</div>
      </div>
    </div>
  </div>
