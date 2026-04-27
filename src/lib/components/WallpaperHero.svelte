<script lang="ts">
  import WallpaperActions from "./WallpaperActions.svelte";

  let {
    image,
    favorite = false,
    onToggleFavorite,
  }: {
    image: BingImage;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  let url = $derived(image.urls[0]);
  let title = $derived(image.titles[0]);
  let startdate = $derived(image.startdates[0]);
  let copyright = $derived(image.copyrights[0]);
  let copyrightlink = $derived(image.copyrightlinks[0]);
  let filename = $derived(url.split("/").pop() ?? "");

  let formattedDate = $derived(`${startdate.slice(0, 4)}-${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`);
</script>

<div class="grid gap-1">
  <div class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1">
    <img
      class="w-full rounded transition-all group-hover:scale-105"
      src={url}
      alt={title}
    />
    <div
      class="p-2 bg-gradient-to-r from-black/50 to-black/0 rounded overflow-hidden absolute left-0 right-0 bottom-0 top-0"
    ></div>
  </div>
  <div class="grid gap-1 mt-3">
    <div class="flex justify-between items-center">
      <div class="font-semibold truncate mr-2">{title}</div>
      <WallpaperActions
        service="Bing"
        {url}
        href={copyrightlink}
        {filename}
        {favorite}
        {onToggleFavorite}
      />
    </div>
    <div class="text-xs text-neutral-700">
      <span class="font-normal text-xs text-right text-neutral-700">{formattedDate}</span>
      {copyright}
    </div>
  </div>
</div>
