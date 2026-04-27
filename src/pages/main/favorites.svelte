<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WallpaperActions from "../../lib/components/WallpaperActions.svelte";

  let favorites = $state<FavoriteItem[]>([]);
  let loading = $state(true);

  function formattedDate(startdate: string): string {
    return startdate.slice(0, 4) + "-" + startdate.slice(4, 6) + "-" + startdate.slice(6, 8);
  }

  async function loadFavorites() {
    loading = true;
    try {
      favorites = (await invoke("list_favorites")) as FavoriteItem[];
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(filename: string) {
    await invoke("remove_favorite", { filename });
    await loadFavorites();
  }

  loadFavorites();
</script>

<div class="grid gap-4">
  {#if loading}
    <div class="text-sm text-neutral-500">加载中…</div>
  {:else if favorites.length === 0}
    <div class="rounded-lg border border-dashed p-4 text-sm text-neutral-500">
      还没有收藏。你可以在 Today 或 Recent 里收藏喜欢的 Bing 壁纸。
    </div>
  {:else}
    {#each favorites as item (item.filename)}
      <div class="grid gap-1">
        <div class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1">
          <img
            class="w-full rounded transition-all group-hover:scale-105"
            src={item.url}
            alt={item.title}
          />
          <div
            class="p-2 bg-gradient-to-r from-black/50 to-black/0 rounded overflow-hidden absolute left-0 right-0 bottom-0 top-0"
          ></div>
        </div>
        <div class="grid gap-1 mt-3">
          <div class="flex justify-between items-center">
            <div class="font-semibold truncate mr-2">{item.title}</div>
            <WallpaperActions
              service="Bing"
              url={item.url}
              href={item.copyrightlink}
              filename={item.filename}
              favorite={true}
              onToggleFavorite={async () => removeFavorite(item.filename)}
            />
          </div>
          <div class="text-xs text-neutral-700">
            <span class="font-normal text-xs text-right text-neutral-700">{formattedDate(item.startdate)}</span>
            {item.copyright}
          </div>
        </div>
      </div>
    {/each}
  {/if}
</div>
