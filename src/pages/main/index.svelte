<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import TodayHero from "../../lib/components/TodayHero.svelte";
  import RecentCollection from "../../lib/components/RecentCollection.svelte";
  import RecentPreview from "../../lib/components/RecentPreview.svelte";
  import FavoritesPreview from "../../lib/components/FavoritesPreview.svelte";
  import SettingsPanel from "../../lib/components/SettingsPanel.svelte";
  import Skeleton from "../../lib/components/Skeleton.svelte";

  // 今日 Hero — 轮播（所有地区今日壁纸）
  let todayImages = $state<SchedulerPhoto[]>([]);
  let todayLoading = $state(true);

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
      let collection = (await invoke("get_today_collection")) as SchedulerPhoto[];
      if (collection.length <= 1) {
        const recent = (await invoke("get_recent_wallpapers", { days: 7 })) as SchedulerPhoto[];
        collection = recent.slice(0, 10);
      }
      todayImages = collection;
      await loadFavorites();
    } finally {
      todayLoading = false;
    }
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

<div class="main-root">
  <div class="topbar">
    <div class="topbar-brand">
      <span class="brand-dot"></span>
      <span class="brand-name">Pavo</span>
    </div>
    <div class="topbar-actions">
      <button
        type="button"
        class="topbar-btn"
        onclick={refreshAll}
        title="刷新"
      >
        ↻
      </button>
      <button
        type="button"
        class="topbar-btn"
        onclick={() => (settingsOpen = true)}
        title="设置"
      >
        ⚙
      </button>
    </div>
  </div>

  <div class="main-scroll scrollbar-stable">
    <div class="hero-section">
      {#if todayLoading}
        <Skeleton />
      {:else if todayImages.length > 0}
        <TodayHero images={todayImages} {favoritesSet} />
      {:else}
        <div class="empty-hint">今日壁纸暂不可用</div>
      {/if}
    </div>

    <div class="section">
      <RecentCollection {favoritesSet} />
    </div>

    <div class="section">
      <RecentPreview {favoritesSet} />
    </div>

    <div class="section">
      <FavoritesPreview />
    </div>
  </div>
</div>

<SettingsPanel open={settingsOpen} onClose={() => (settingsOpen = false)} />

<style>
  .main-root {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg);
  }

  /* 顶部栏 */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    border-bottom: 1px solid var(--topbar-border);
  }

  .topbar-brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .brand-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--accent);
  }

  .brand-name {
    font-size: 15px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .topbar-actions {
    display: flex;
    gap: 12px;
  }

  .topbar-btn {
    font-size: 14px;
    color: var(--text-secondary);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    transition: color 0.15s;
  }

  .topbar-btn:hover {
    color: var(--text-primary);
  }

  /* 主滚动区域 */
  .main-scroll {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  /* Hero 区域：无包裹，贴边 */
  .hero-section {
    margin-bottom: 14px;
  }

  /* 各 section 统一布局 */
  .section {
    padding: 0 16px;
    margin-top: 14px;
  }

  .empty-hint {
    font-size: 13px;
    color: var(--text-secondary);
    padding: 16px;
  }
</style>
