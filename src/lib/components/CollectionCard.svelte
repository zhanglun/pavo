<script lang="ts">
  import WallpaperActions from "./WallpaperActions.svelte";

  let {
    image,
    regionIndex = 0,
    favorite = false,
    onToggleFavorite,
  }: {
    image: BingImage;
    regionIndex: number;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  let url = $derived(image.urls[regionIndex] ?? image.urls[0]);
  let title = $derived(image.titles[regionIndex] ?? image.titles[0]);
  let copyright = $derived(image.copyrights[regionIndex] ?? image.copyrights[0]);
  let copyrightlink = $derived(
    image.copyrightlinks[regionIndex] ?? image.copyrightlinks[0]
  );
  let filename = $derived(url.split("/").pop() ?? "");

  let hovered = $state(false);
</script>

<div
  class="border border-gray-200 rounded-[10px] overflow-hidden transition-all duration-300"
  class:hover:border-gray-300={hovered}
  style="box-shadow: {hovered ? '0 4px 16px rgba(74,124,150,0.15)' : '0 1px 4px rgba(0,0,0,0.06)'}"
  onmouseenter={() => hovered = true}
  onmouseleave={() => hovered = false}
>
  <div
    class="relative overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
  >
    <img
      class="w-full transition-all duration-300 group-hover:scale-105"
      style="filter: {hovered ? 'brightness(1.05)' : 'none'}"
      src={url}
      alt={title}
    />
  </div>
  <div style="border-bottom: 1px solid #f0f0f0"></div>
  <div class="p-2.5 grid gap-1">
    <div class="flex justify-between items-center">
      <div class="text-xs font-medium truncate mr-1">{title}</div>
      <WallpaperActions
        service="Bing"
        {url}
        href={copyrightlink}
        {filename}
        {favorite}
        {onToggleFavorite}
      />
    </div>
  </div>
</div>
