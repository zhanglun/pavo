<script lang="ts">
  import OverflowMenu from "./OverflowMenu.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { setAsDesktop } from "../utils/wallpaper";
  import { open } from "@tauri-apps/plugin-shell";

  let {
    image,
    regionIndex = 0,
    favorite = false,
    onToggleFavorite,
  }: {
    image: BingImage;
    regionIndex: number;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  let url = $derived(image.urls[regionIndex] ?? image.urls[0]);
  let title = $derived(image.titles[regionIndex] ?? image.titles[0]);
  let copyrightlink = $derived(
    image.copyrightlinks[regionIndex] ?? image.copyrightlinks[0]
  );
  let startdate = $derived(
    image.startdates[regionIndex] ?? image.startdates[0] ?? ""
  );

  // 从 startdate 提取 "MM/DD" 格式
  let dateLabel = $derived.by(() => {
    if (!startdate || startdate.length < 8) return "";
    const month = startdate.slice(4, 6);
    const day = startdate.slice(6, 8);
    return `${month}/${day}`;
  });

  let menuItems = $derived([
    {
      label: "设为背景",
      action: () => setAsDesktop(url),
    },
    {
      label: "下载",
      action: () => {
        invoke("download", { service: "Bing", url });
      },
    },
    {
      label: "详情",
      action: () => {
        if (copyrightlink) {
          open(copyrightlink);
        }
      },
    },
  ]);

  async function handleFavorite(e: MouseEvent) {
    e.stopPropagation();
    await onToggleFavorite();
  }
</script>

<div class="card">
  <img class="card-image" src={url} alt={title} />

  <div class="card-overlay">
    <div class="overlay-content">
      <span class="date-label" title={title}>
        {#if dateLabel}
          {dateLabel} {title}
        {:else}
          {title}
        {/if}
      </span>
      <div class="actions">
        <button
          class="fav-btn"
          class:active={favorite}
          onclick={handleFavorite}
          aria-label={favorite ? "取消收藏" : "收藏"}
        >
          ♡
        </button>
        <OverflowMenu items={menuItems} />
      </div>
    </div>
  </div>
</div>

<style>
  .card {
    position: relative;
    width: 100%;
    aspect-ratio: 16 / 10;
    border-radius: 6px;
    overflow: visible;
  }

  .card-image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 6px;
  }

  .card-overlay {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.55), transparent);
    padding: 14px 6px 4px;
    border-radius: 0 0 6px 6px;
  }

  .overlay-content {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 4px;
  }

  .date-label {
    flex: 1;
    font-size: 10px;
    line-height: 1.3;
    color: rgba(255, 255, 255, 0.9);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }

  .fav-btn {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: none;
    background: rgba(0, 0, 0, 0.4);
    color: rgba(255, 255, 255, 0.8);
    font-size: 11px;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: background-color 0.15s;
  }

  .fav-btn:hover {
    background: rgba(0, 0, 0, 0.6);
  }

  .fav-btn.active {
    color: var(--heart-active);
  }

  /* 覆盖 OverflowMenu 的 trigger 样式，适配卡片内的深色背景 */
  .actions :global(.trigger) {
    width: 20px;
    height: 20px;
    background: rgba(0, 0, 0, 0.4);
    color: rgba(255, 255, 255, 0.8);
    font-size: 12px;
    border-radius: 50%;
  }

  .actions :global(.trigger:hover) {
    background: rgba(0, 0, 0, 0.6);
  }
</style>
