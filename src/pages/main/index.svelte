<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import TodayHero from "../../lib/components/TodayHero.svelte";
  import RecentCollection from "../../lib/components/RecentCollection.svelte";
  import RecentPreview from "../../lib/components/RecentPreview.svelte";
  import FavoritesPreview from "../../lib/components/FavoritesPreview.svelte";
  import SettingsPanel from "../../lib/components/SettingsPanel.svelte";
  import Skeleton from "../../lib/components/Skeleton.svelte";

  // 今日 Hero
  let todayImage = $state<BingImage | null>(null);
  let todayLoading = $state(true);
  let todayFavorite = $state(false);

  // 共享收藏集合
  let favoritesSet = $state<Set<string>>(new Set());

  // Settings 面板
  let settingsOpen = $state(false);

  async function loadFavorites() {
    const favs = (await invoke("list_favorites")) as FavoriteItem[];
    favoritesSet = new Set(favs.map((f) => f.filename));
  }

  async function loadTodayHero() {
    todayLoading = true;
    try {
      todayImage = (await invoke("get_today_wallpaper")) as BingImage | null;
      await loadFavorites();
      if (todayImage) {
        const filename = todayImage.urls[0].split("/").pop() ?? "";
        todayFavorite = favoritesSet.has(filename);
      }
    } finally {
      todayLoading = false;
    }
  }

  async function toggleTodayFavorite() {
    if (!todayImage) return;
    const url = todayImage.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (todayFavorite) {
      await invoke("remove_favorite", { filename });
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: todayImage.titles[0],
        startdate: todayImage.startdates[0],
        copyright: todayImage.copyrights[0],
        copyrightlink: todayImage.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
    }
    todayFavorite = !todayFavorite;
    await loadFavorites();
  }

  // 刷新全部
  async function refreshAll() {
    await loadTodayHero();
  }

  // 事件监听
  $effect(() => {
    const unlisteners: (() => void)[] = [];

    listen("go-to-settings", () => {
      settingsOpen = true;
    }).then((unlisten) => unlisteners.push(unlisten));

    listen("check-for-updates", async () => {
      const { checkUpdate } = await import("../../lib/updater");
      checkUpdate();
    }).then((unlisten) => unlisteners.push(unlisten));

    return () => {
      for (const unlisten of unlisteners) {
        unlisten();
      }
    };
  });

  // 初始加载
  refreshAll();
</script>

<div class="w-full h-full flex flex-col" style="background: #f5f5f6">
  <div class="flex items-center justify-between px-4 py-2 bg-white border-b border-gray-100">
    <div class="flex items-center gap-2">
      <div class="w-2 h-2 rounded-full bg-gradient-to-br from-bayOfMany-900 to-apple-500"></div>
      <span class="text-sm font-semibold">Pavo</span>
    </div>
    <div class="flex gap-2">
      <button
        type="button"
        class="text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={refreshAll}
        title="刷新"
      >
        ↻
      </button>
      <button
        type="button"
        class="text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={() => (settingsOpen = true)}
        title="设置"
      >
        ⚙
      </button>
    </div>
  </div>

  <div class="flex-1 h-0 overflow-y-auto overflow-x-hidden scrollbar-stable p-3">
    <div class="grid gap-4">
      <div class="bg-white rounded-[10px] p-4" style="box-shadow: 0 1px 4px rgba(0,0,0,0.06)">
        {#if todayLoading}
          <Skeleton />
        {:else if todayImage}
          <TodayHero
            image={todayImage}
            favorite={todayFavorite}
            onToggleFavorite={toggleTodayFavorite}
          />
        {:else}
          <div class="text-sm text-neutral-500">今日壁纸暂不可用</div>
        {/if}
      </div>

      <div class="bg-white rounded-[10px] p-4" style="box-shadow: 0 1px 4px rgba(0,0,0,0.06)">
        <RecentCollection {favoritesSet} />
      </div>

      <div class="bg-white rounded-[10px] p-4" style="box-shadow: 0 1px 4px rgba(0,0,0,0.06)">
        <RecentPreview {favoritesSet} />
      </div>

      <div class="bg-white rounded-[10px] p-4" style="box-shadow: 0 1px 4px rgba(0,0,0,0.06)">
        <FavoritesPreview />
      </div>
    </div>
  </div>
</div>

<SettingsPanel open={settingsOpen} onClose={() => (settingsOpen = false)} />

<style>
  @keyframes fadeInSection {
    from {
      opacity: 0;
      transform: translateY(12px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
