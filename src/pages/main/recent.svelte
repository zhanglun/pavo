<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WallpaperHero from "../../lib/components/WallpaperHero.svelte";
  import Skeleton from "../../lib/components/Skeleton.svelte";
  import { Select, Label } from "flowbite-svelte";

  let range = $state(7);
  let images = $state<BingImage[]>([]);
  let loading = $state(true);
  let favoritesSet = $state<Set<string>>(new Set());

  async function loadFavorites() {
    const favs = (await invoke("list_favorites")) as FavoriteItem[];
    favoritesSet = new Set(favs.map((f) => f.filename));
  }

  async function loadRecent() {
    loading = true;
    try {
      images = (await invoke("get_recent_wallpapers", { days: range })) as BingImage[];
      await loadFavorites();
    } finally {
      loading = false;
    }
  }

  async function toggleFavorite(image: BingImage) {
    const url = image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favoritesSet.has(filename)) {
      await invoke("remove_favorite", { filename });
      const next = new Set(favoritesSet);
      next.delete(filename);
      favoritesSet = next;
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
      const next = new Set(favoritesSet);
      next.add(filename);
      favoritesSet = next;
    }
  }

  $effect(() => {
    range;
    loadRecent();
  });
</script>

<div class="grid gap-4">
  <div class="flex justify-end items-center gap-2">
    <Label for="recent-range">范围</Label>
    <Select
      id="recent-range"
      size="sm"
      class="w-auto"
      bind:value={range}
    >
      <option value={7}>最近 7 天</option>
      <option value={14}>最近 14 天</option>
    </Select>
  </div>

  {#if loading}
    <div class="grid gap-4">
      {#each Array(3) as _}
        <Skeleton />
      {/each}
    </div>
  {:else if images.length === 0}
    <div class="text-sm text-neutral-500">暂无近期壁纸</div>
  {:else}
    <div class="grid gap-4">
      {#each images as img}
        <WallpaperHero
          image={img}
          favorite={favoritesSet.has(img.urls[0].split("/").pop() ?? "")}
          onToggleFavorite={() => toggleFavorite(img)}
        />
      {/each}
    </div>
  {/if}
</div>
