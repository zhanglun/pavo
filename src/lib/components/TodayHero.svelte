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
  let formattedDate = $derived(
    `${startdate.slice(0, 4)}-${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`
  );
</script>

<div class="relative overflow-hidden group w-full aspect-[1.78] bg-medirian-1 rounded-[10px]">
  <img
    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
    src={url}
    alt={title}
  />

  <span
    class="absolute top-2 left-2 text-white text-[10px] tracking-[2px] px-2 py-0.5 rounded z-10"
    style="background: rgba(0,0,0,0.35); backdrop-filter: blur(4px)"
  >TODAY</span>

  <div
    class="absolute left-0 right-0 bottom-0 flex items-end justify-between gap-2 px-3 py-2 z-10"
    style="background: rgba(0,0,0,0.4); backdrop-filter: blur(8px); border-radius: 0 0 10px 10px"
  >
    <div class="flex flex-col gap-0.5 min-w-0 flex-1">
      <div class="text-white font-semibold text-[13px] truncate">{title}</div>
      <div class="flex items-center gap-2">
        <span class="text-[10px]" style="color: rgba(255,255,255,0.6)">{formattedDate}</span>
        <span class="text-[10px] truncate" style="color: rgba(255,255,255,0.4)">{copyright}</span>
      </div>
    </div>
    <div class="shrink-0 [&_button]:!text-white [&_a]:!text-white [&_button:hover]:!text-white [&_a:hover]:!text-white">
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
