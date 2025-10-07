<script lang="ts">
  import Toolbar from "./Toolbar.svelte";

  let { titles, urls, startdates, copyrights, copyrightlinks }: BingImage =
    $props();

  let title = titles[0];
  let url = urls[0];
  let startdate = startdates[0];
  let copyright = copyrights[0];
  let copyrightlink = copyrightlinks[0];

  function view_photo() {}
</script>

<div class="relative cursor-pointer">
  <button
    type="button"
    class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
    onclick={view_photo}
  >
    <img
      class="w-full rounded transition-all group-hover:scale-125"
      src={url}
      alt={title}
    />
    <div
      class="
            p-2 bg-gradient-to-r from-black/50 to-black/0
            rounded
            overflow-hidden
            absolute left-0 right-0 bottom-0 top-0
          "
    ></div>
  </button>
  <div
    class="
          grid gap-2 mt-3
          grid-flow-row
          sm:grid-flow-col
          lg:grid-flow-row
          xl:grid-flow-col
        "
  >
    <div class="grid gap-1">
      <div class="flex justify-between items-center">
        <div class="font-semibold">{title}</div>
        <Toolbar service={"Bing"} {url} href={copyrightlink} />
      </div>
      <div class="text-xs text-neutral-700">
        <span class="font-normal text-xs text-right text-neutral-700"
          >{`${startdate.slice(0, 4)}-${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`}</span
        >
        {copyright}
      </div>
    </div>
  </div>
</div>
