<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { setAsDesktop } from "../utils/wallpaper";
  import { tick } from "svelte";
  import Skeleton from "./Skeleton.svelte";
  import OverflowMenu from "./OverflowMenu.svelte";

  let {
    favoritesSet,
  }: {
    favoritesSet: Set<string>;
  } = $props();

  let range = $state(7);
  let images = $state<BingImage[]>([]);
  let loading = $state(true);
  let expanded = $state(false);
  let sectionRef: HTMLDivElement | undefined = $state();
  const PREVIEW_COUNT = 3;

  async function loadRecent() {
    loading = true;
    try {
      images = (await invoke("get_recent_wallpapers", { days: range })) as BingImage[];
    } finally {
      loading = false;
    }
  }

  async function toggleFavorite(image: BingImage) {
    const url = image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favoritesSet.has(filename)) {
      await invoke("remove_favorite", { filename });
      favoritesSet.delete(filename);
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
      favoritesSet.add(filename);
    }
    favoritesSet = new Set(favoritesSet);
  }

  function formatDate(startdate: string): string {
    if (startdate.length === 8) {
      return `${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`;
    }
    return startdate;
  }

  async function toggleExpand() {
    expanded = !expanded;
    if (expanded) {
      await tick();
      sectionRef?.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  async function downloadImage(url: string) {
    await invoke("download", { service: "Bing", url });
  }

  function getMenuItems(img: BingImage) {
    return [
      { label: "设为背景", icon: "🖥", action: () => setAsDesktop(img.urls[0]) },
      { label: "下载", icon: "⬇", action: () => downloadImage(img.urls[0]) },
    ];
  }

  $effect(() => {
    range;
    loadRecent();
  });

  let hasMore = $derived(images.length > PREVIEW_COUNT);
</script>

<div class="recent-preview" bind:this={sectionRef}>
  <div class="header">
    <span class="section-title">近期壁纸</span>
    <select class="range-select" bind:value={range}>
      <option value={7}>7天</option>
      <option value={14}>14天</option>
    </select>
  </div>

  {#if loading}
    {#each Array(2) as _}
      <Skeleton />
    {/each}
  {:else if images.length === 0}
    <div class="empty-text">暂无近期壁纸</div>
  {:else}
    {#each images as img, i}
      {@const filename = img.urls[0].split("/").pop() ?? ""}
      {@const isFav = favoritesSet.has(filename)}
      <div class="item-row" class:hidden={!expanded && i >= PREVIEW_COUNT}>
        <img
          class="thumb"
          src={img.urls[0]}
          alt={img.titles[0]}
        />
        <div class="item-info">
          <div class="item-title">{img.titles[0]}</div>
          <div class="item-date">{formatDate(img.startdates[0])}</div>
        </div>
        <button
          type="button"
          class="fav-btn"
          class:fav-active={isFav}
          onclick={() => toggleFavorite(img)}
          title={isFav ? "取消收藏" : "收藏"}
        >
          {isFav ? "♥" : "♡"}
        </button>
        <OverflowMenu items={getMenuItems(img)} />
      </div>
    {/each}

    {#if hasMore}
      <button
        type="button"
        class="view-all-btn"
        onclick={toggleExpand}
      >
        {expanded ? "收起" : "查看全部"}
      </button>
    {/if}
  {/if}

  <div class="divider"></div>
</div>

<style>
  .recent-preview {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  /* 标题行 */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .range-select {
    font-size: 11px;
    padding: 2px 4px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg);
    color: var(--text-secondary);
    cursor: pointer;
    outline: none;
    line-height: 1.2;
  }

  .range-select:focus {
    border-color: var(--accent);
  }

  /* 空状态 */
  .empty-text {
    font-size: 12px;
    color: var(--text-tertiary);
    padding: 8px 0;
  }

  /* 每行壁纸项 */
  .item-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 4px;
    border-radius: 4px;
    transition: background 0.15s;
    cursor: default;
  }

  .item-row:hover {
    background: var(--menu-hover);
  }

  .item-row.hidden {
    display: none;
  }

  /* 缩略图 */
  .thumb {
    width: 64px;
    height: 36px;
    border-radius: 4px;
    object-fit: cover;
    flex-shrink: 0;
  }

  /* 中间信息区 */
  .item-info {
    flex: 1;
    min-width: 0;
  }

  .item-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-date {
    font-size: 10px;
    color: var(--text-tertiary);
    margin-top: 2px;
  }

  /* 收藏按钮 */
  .fav-btn {
    flex-shrink: 0;
    background: none;
    border: none;
    font-size: 13px;
    cursor: pointer;
    padding: 2px 4px;
    color: var(--heart);
    line-height: 1;
    transition: color 0.15s;
  }

  .fav-active {
    color: var(--heart-active);
  }

  /* 查看全部按钮 */
  .view-all-btn {
    border: none;
    border-radius: 4px;
    padding: 4px 12px;
    color: var(--accent);
    background: transparent;
    cursor: pointer;
    font-size: 12px;
    width: 100%;
    margin-top: 2px;
  }

  .view-all-btn:hover {
    opacity: 0.85;
  }

  /* 底部分割线 */
  .divider {
    height: 1px;
    background: var(--border);
    margin-top: 8px;
  }
</style>
