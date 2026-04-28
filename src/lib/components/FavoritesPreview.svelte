<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";

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

  loadFavorites();

  let hasMore = $derived(favorites.length > PREVIEW_COUNT);
</script>

<div class="favorites-section" bind:this={sectionRef}>
  <div class="favorites-header">
    <div class="favorites-title">
      <span class="title-bar"></span>
      <span class="title-text">收藏</span>
    </div>
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
          <span class="favorite-name">{item.title}</span>
          <button
            type="button"
            class="remove-btn"
            title="移除收藏"
            onclick={() => removeFavorite(item.filename)}
          >✕</button>
        </div>
      {/each}
    </div>
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

  .favorites-title {
    display: flex;
    align-items: center;
  }

  .title-bar {
    display: inline-block;
    width: 3px;
    height: 16px;
    background: #4a7c96;
    border-radius: 1px;
    margin-right: 8px;
  }

  .title-text {
    font-size: 13px;
    letter-spacing: 1px;
    color: #333;
    font-weight: 500;
  }

  .view-all-btn {
    border: 1px dashed #ddd;
    border-radius: 4px;
    padding: 4px 12px;
    font-size: 12px;
    color: #999;
    background: transparent;
    cursor: pointer;
    transition: color 0.2s, border-color 0.2s;
  }

  .view-all-btn:hover {
    color: #666;
    border-color: #bbb;
  }

  .empty-hint {
    font-size: 12px;
    color: #999;
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
    transition: background 0.15s;
    cursor: default;
    animation: fadeInUp 0.25s ease forwards;
  }

  .favorite-row:hover {
    background: #f5f5f6;
  }

  .favorite-row.hidden {
    display: none;
  }

  .favorite-thumb {
    width: 72px;
    aspect-ratio: 16 / 9;
    border-radius: 4px;
    object-fit: cover;
    flex-shrink: 0;
  }

  .favorite-name {
    flex: 1;
    padding-left: 8px;
    font-size: 12px;
    color: #333;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .remove-btn {
    font-size: 14px;
    color: #999;
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 2px 6px;
    line-height: 1;
    border-radius: 3px;
    transition: color 0.15s, background 0.15s;
    flex-shrink: 0;
  }

  .remove-btn:hover {
    color: #e55;
    background: #fde8e8;
  }

  @keyframes fadeInUp {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
