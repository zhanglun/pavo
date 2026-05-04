<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { setAsDesktop, downloadWallpaper, isSettingWallpaper, isDownloading } from "../utils/wallpaper";
  import { tick } from "svelte";
  import OverflowMenu from "./OverflowMenu.svelte";
  import { open } from "@tauri-apps/plugin-shell";

  let {
    onFavoritesChanged,
  }: {
    onFavoritesChanged?: () => void;
  } = $props();

  let favorites = $state<FavoriteItem[]>([]);
  let loading = $state(true);
  let expanded = $state(false);
  let sectionRef: HTMLDivElement | undefined = $state();
  const PREVIEW_COUNT = 3;

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
    onFavoritesChanged?.();
  }

  async function toggleExpand() {
    expanded = !expanded;
    if (expanded) {
      await tick();
      sectionRef?.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  function setAsBackground(url: string) {
    setAsDesktop(url);
  }

  function downloadImage(url: string) {
    downloadWallpaper(url);
  }

  function openDetail(copyrightlink: string) {
          open(copyrightlink);
  }

  function buildMenuItems(item: FavoriteItem) {
    return [
      { label: "设为背景", action: () => setAsBackground(item.url), disabled: $isSettingWallpaper },
      { label: "下载", action: () => downloadImage(item.url), disabled: $isDownloading },
      { label: "详情", action: () => openDetail(item.copyrightlink) },
      { label: "取消收藏", action: () => removeFavorite(item.filename), danger: true },
    ];
  }

  loadFavorites();

  let hasMore = $derived(favorites.length > PREVIEW_COUNT);
</script>

<div class="favorites-section" bind:this={sectionRef}>
  <div class="favorites-header">
    <span class="title-text">收藏</span>
    {#if hasMore}
      <button
        type="button"
        class="view-all-btn"
        onclick={toggleExpand}
      >
        {expanded ? "收起" : "查看全部"}
      </button>
    {/if}
  </div>

  {#if loading}
    <div class="empty-hint">加载中…</div>
  {:else if favorites.length === 0}
    <div class="empty-hint">还没有收藏，点击壁纸右下方的 ☆ 收藏</div>
  {:else}
    <div class="favorites-list">
      {#each favorites as item, i (item.filename)}
        <div class="favorite-row" class:hidden={!expanded && i >= PREVIEW_COUNT}>
          <img
            class="favorite-thumb"
            src={item.url}
            alt={item.title}
          />
          <div class="favorite-info">
            <span class="favorite-name">{item.title}</span>
            <span class="favorite-date">{item.startdate}</span>
          </div>
          <span class="heart-icon">♥</span>
          <OverflowMenu items={buildMenuItems(item)} />
        </div>
      {/each}
    </div>

    {#if favorites.length > 0}
      <div class="divider"></div>
    {/if}
  {/if}
</div>

<style>
  .favorites-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .favorites-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title-text {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .view-all-btn {
    border: none;
    border-radius: 4px;
    padding: 4px 12px;
    font-size: 12px;
    color: var(--accent);
    background: transparent;
    cursor: pointer;
    transition: opacity 0.15s;
  }

  .view-all-btn:hover {
    opacity: 0.8;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--text-tertiary);
  }

  .favorites-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .favorite-row {
    display: flex;
    align-items: center;
    padding: 6px 4px;
    border-radius: 4px;
    transition: background-color 0.15s;
    cursor: default;
    gap: 8px;
  }

  .favorite-row:hover {
    background: var(--bg);
  }

  .favorite-row.hidden {
    display: none;
  }

  .favorite-thumb {
    width: 64px;
    height: 36px;
    border-radius: 4px;
    object-fit: cover;
    flex-shrink: 0;
  }

  .favorite-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .favorite-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .favorite-date {
    font-size: 10px;
    color: var(--text-tertiary);
  }

  .heart-icon {
    font-size: 13px;
    color: var(--heart-active);
    flex-shrink: 0;
    line-height: 1;
  }

  .divider {
    height: 1px;
    background: var(--border);
    margin-top: 4px;
  }
</style>
