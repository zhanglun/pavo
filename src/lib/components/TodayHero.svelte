<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { setAsDesktop, downloadWallpaper, isSettingWallpaper, isDownloading } from "../utils/wallpaper";

  let {
    images,
    favoritesSet,
  }: {
    images: SchedulerPhoto[];
    favoritesSet: Set<string>;
  } = $props();

  let currentIndex = $state(0);
  let paused = $state(false);
  let timer: ReturnType<typeof setInterval> | null = null;

  let current = $derived(images[currentIndex] ?? images[0]);
  let url = $derived(current?.urls[0] ?? "");
  let title = $derived(current?.titles[0] ?? "");
  let startdate = $derived(current?.startdates[0] ?? "");
  let filename = $derived(url.split("/").pop() ?? "");
  let favorite = $derived(favoritesSet.has(filename));
  let formattedDate = $derived(
    startdate.length >= 8
      ? `${startdate.slice(0, 4)}-${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`
      : ""
  );
  let total = $derived(images.length);

  function startTimer() {
    stopTimer();
    timer = setInterval(() => {
      if (!paused && total > 1) {
        currentIndex = (currentIndex + 1) % total;
      }
    }, 5000);
  }

  function stopTimer() {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
  }

  function goNext() {
    currentIndex = (currentIndex + 1) % total;
    startTimer();
  }

  function goPrev() {
    currentIndex = (currentIndex - 1 + total) % total;
    startTimer();
  }

  function goTo(idx: number) {
    currentIndex = idx;
    startTimer();
  }

  function onMouseEnter() {
    paused = true;
  }
  function onMouseLeave() {
    paused = false;
  }

  async function handleSetAsDesktop() {
    await setAsDesktop(url);
  }

  async function download() {
    await downloadWallpaper(url);
  }

  async function toggleFavorite() {
    if (!current) return;
    if (favorite) {
      await invoke("remove_favorite", { filename });
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: current.titles[0],
        startdate: current.startdates[0],
        copyright: current.copyrights[0],
        copyrightlink: current.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
    }
    const favs = (await invoke("list_favorites")) as FavoriteItem[];
    favoritesSet = new Set(favs.map((f) => f.filename));
  }

  $effect(() => {
    if (images.length > 0) {
      currentIndex = 0;
      startTimer();
    }
    return () => stopTimer();
  });
</script>

<div class="hero" onmouseenter={onMouseEnter} onmouseleave={onMouseLeave}>
  {#key url}
    <img class="hero-img" src={url} alt={title} />
  {/key}

  <div class="debug">{total} 张 / 第 {currentIndex + 1} 张</div>

  {#if total > 1}
    <button class="arrow arrow-left" type="button" onclick={goPrev} aria-label="上一张">‹</button>
    <button class="arrow arrow-right" type="button" onclick={goNext} aria-label="下一张">›</button>
  {/if}

  <div class="overlay">
    <div class="info">
      <div class="title">{title}</div>
      <div class="date">{formattedDate}</div>
    </div>

    <div class="actions">
      <button class="pill" type="button" onclick={handleSetAsDesktop} disabled={$isSettingWallpaper}>设为背景</button>
      <button class="pill" type="button" onclick={download} disabled={$isDownloading}>下载</button>
      <button
        class="fav"
        type="button"
        onclick={toggleFavorite}
        aria-label={favorite ? "取消收藏" : "收藏"}
      >
        {favorite ? "♥" : "♡"}
      </button>
    </div>
  </div>

  {#if total > 1}
    <div class="indicators">
      {#each images as _, i}
        <button
          class="dot"
          class:active={i === currentIndex}
          type="button"
          onclick={() => goTo(i)}
          aria-label="切换到第{i + 1}张"
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .hero {
    position: relative;
    width: 100%;
    aspect-ratio: 16 / 9;
    overflow: hidden;
    background-color: var(--bg-secondary, #f5f5f5);
  }

  .hero-img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    animation: fadeIn 0.5s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .debug {
    position: absolute;
    top: 4px;
    right: 4px;
    padding: 2px 6px;
    background: rgba(0, 0, 0, 0.6);
    color: rgba(255, 255, 255, 0.8);
    font-size: 10px;
    border-radius: 4px;
    z-index: 20;
    pointer-events: none;
  }

  .arrow {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: rgba(0, 0, 0, 0.3);
    color: rgba(255, 255, 255, 0.9);
    font-size: 18px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.15s;
    z-index: 10;
  }

  .arrow:hover {
    background: rgba(0, 0, 0, 0.5);
  }

  .arrow-left {
    left: 8px;
  }

  .arrow-right {
    right: 8px;
  }

  .overlay {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 8px;
    padding: 28px 10px 10px;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.55), transparent);
    z-index: 5;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .title {
    color: rgba(255, 255, 255, 0.95);
    font-size: 13px;
    font-weight: 600;
    line-height: 1.3;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .date {
    color: rgba(255, 255, 255, 0.55);
    font-size: 10px;
    line-height: 1.2;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-shrink: 0;
  }

  .pill {
    padding: 4px 10px;
    border-radius: 99px;
    border: 1px solid rgba(255, 255, 255, 0.3);
    background: rgba(255, 255, 255, 0.12);
    color: rgba(255, 255, 255, 0.95);
    font-size: 10px;
    line-height: 1.2;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .pill:hover {
    background: rgba(255, 255, 255, 0.22);
  }

  .pill:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .fav {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.3);
    background: rgba(255, 255, 255, 0.12);
    color: var(--heart-active, #e8b74a);
    font-size: 13px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.15s;
    padding: 0;
  }

  .fav:hover {
    background: rgba(255, 255, 255, 0.22);
  }

  .indicators {
    position: absolute;
    bottom: 6px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 4px;
    z-index: 10;
  }

  .dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.4);
    padding: 0;
    cursor: pointer;
    transition: background 0.2s, transform 0.2s;
  }

  .dot.active {
    background: rgba(255, 255, 255, 0.9);
    transform: scale(1.3);
  }

  .dot:hover {
    background: rgba(255, 255, 255, 0.7);
  }
</style>
