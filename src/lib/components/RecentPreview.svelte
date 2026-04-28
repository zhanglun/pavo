<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import Skeleton from "./Skeleton.svelte";
  import { Select } from "flowbite-svelte";

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

  $effect(() => {
    range;
    loadRecent();
  });

  let hasMore = $derived(images.length > PREVIEW_COUNT);
</script>

<div class="recent-preview" bind:this={sectionRef}>
  <div class="header">
    <div class="header-left">
      <span class="section-indicator"></span>
      <span class="section-title">近期壁纸</span>
    </div>
    <Select
      size="sm"
      style="font-size: 11px"
      bind:value={range}
    >
      <option value={7}>7天</option>
      <option value={14}>14天</option>
    </Select>
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
          {isFav ? "★" : "☆"}
        </button>
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

  .header-left {
    display: flex;
    align-items: center;
  }

  .section-indicator {
    display: inline-block;
    width: 3px;
    height: 16px;
    background: #4a7c96;
    border-radius: 1px;
    margin-right: 8px;
  }

  .section-title {
    font-size: 13px;
    letter-spacing: 1px;
    color: #333;
  }

  /* 空状态 */
  .empty-text {
    font-size: 12px;
    color: #999;
    padding: 8px 0;
  }

  /* 每行壁纸项 */
  .item-row {
    display: flex;
    align-items: center;
    padding: 6px 4px;
    border-radius: 4px;
    transition: background 0.15s;
    cursor: default;
    animation: fadeInUp 0.25s ease forwards;
  }

  .item-row:hover {
    background: #f5f5f6;
  }

  .item-row.hidden {
    display: none;
  }

  /* 缩略图 */
  .thumb {
    width: 72px;
    aspect-ratio: 16 / 9;
    border-radius: 4px;
    object-fit: cover;
    flex-shrink: 0;
  }

  /* 中间信息区 */
  .item-info {
    flex: 1;
    padding-left: 8px;
    min-width: 0;
  }

  .item-title {
    font-size: 12px;
    color: #333;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-date {
    font-size: 10px;
    color: #999;
    margin-top: 2px;
  }

  /* 收藏按钮 */
  .fav-btn {
    flex-shrink: 0;
    background: none;
    border: none;
    font-size: 14px;
    cursor: pointer;
    padding: 2px 4px;
    color: #ccc;
    line-height: 1;
  }

  .fav-btn:hover {
    color: #e8b74a;
  }

  .fav-active {
    color: #e8b74a;
  }

  /* 查看全部按钮 */
  .view-all-btn {
    border: 1px dashed #ddd;
    border-radius: 4px;
    padding: 4px 12px;
    color: #888;
    background: transparent;
    cursor: pointer;
    font-size: 12px;
    width: 100%;
    margin-top: 2px;
  }

  .view-all-btn:hover {
    color: #666;
    border-color: #bbb;
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
