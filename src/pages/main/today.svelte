<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WallpaperHero from "../../lib/components/WallpaperHero.svelte";
  import Skeleton from "../../lib/components/Skeleton.svelte";

  let image = $state<BingImage | null>(null);
  let loading = $state(true);
  let favorite = $state(false);

  async function loadFavorites() {
    const favs = (await invoke("list_favorites")) as FavoriteItem[];
    if (image) {
      const filename = image.urls[0].split("/").pop() ?? "";
      favorite = favs.some((f) => f.filename === filename);
    }
  }

  async function loadToday() {
    loading = true;
    try {
      image = (await invoke("get_today_wallpaper")) as BingImage | null;
      await loadFavorites();
    } finally {
      loading = false;
    }
  }

  async function toggleFavorite() {
    if (!image) return;
    const url = image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favorite) {
      await invoke("remove_favorite", { filename });
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: image.titles[0],
        startdate: image.startdates[0],
        copyright: image.copyrights[0],
        copyrightlink: image.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
    }
    favorite = !favorite;
  }

  loadToday();
</script>

<div class="grid gap-4">
  {#if loading}
    <Skeleton />
  {:else if image}
    <WallpaperHero {image} {favorite} onToggleFavorite={toggleFavorite} />
  {:else}
    <div class="text-sm text-neutral-500">今日壁纸暂不可用</div>
  {/if}
</div>
