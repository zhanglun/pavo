<script lang="ts">
  import Toolbar from "./Toolbar.svelte";

  // your script goes here
  let { title, url, startdate, copyright, copyrightlink }: BingImage = $props();

  function view_photo() {}
</script>

<div class="relative cursor-pointer">
  <div
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
  </div>
  <div
    class="
          grid gap-2 mt-4
          grid-flow-row
          sm:grid-flow-col
          lg:grid-flow-row
          xl:grid-flow-col
        "
  >
    <div class="grid gap-2">
      <div class="flex justify-between items-center">
        <div class="text-base grid gap-2 grid-flow-col">
          <span class="font-semibold">{title}</span>
          <span class="font-normal">{startdate}</span>
        </div>
        <Toolbar service={"Bing"} {url} href={copyrightlink} />
      </div>
      <div class="text-sm">{copyright}</div>
    </div>
  </div>
</div>
