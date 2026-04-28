<script lang="ts">
  import CollectionCard from "./CollectionCard.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";

  let {
    favoritesSet,
  }: {
    favoritesSet: Set<string>;
  } = $props();

  let expanded = $state(false);
  let items: BingImage[] = $state([]);
  let sectionRef: HTMLDivElement | undefined = $state();

  const PREVIEW_COUNT = 4;

  $effect(() => {
    invoke<BingImage[]>("get_recent_wallpapers", { days: 7 }).then((data) => {
      items = data;
    });
  });

  let totalCount = $derived(items.length);
  let hasMore = $derived(totalCount > PREVIEW_COUNT);
  let headerText = $derived(
    expanded ? `近期合集 · 共 ${totalCount} 张` : `近期合集 · ${Math.min(PREVIEW_COUNT, totalCount)} / ${totalCount}`
  );

  async function toggleExpand() {
    expanded = !expanded;
    if (expanded) {
      await tick();
      sectionRef?.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  async function toggleFavorite(image: BingImage, regionIndex: number) {
    const url = image.urls[regionIndex] ?? image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favoritesSet.has(filename)) {
      await invoke("remove_favorite", { filename });
      favoritesSet.delete(filename);
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: image.titles[regionIndex] ?? image.titles[0],
        startdate: image.startdates[regionIndex] ?? image.startdates[0],
        copyright: image.copyrights[regionIndex] ?? image.copyrights[0],
        copyrightlink: image.copyrightlinks[regionIndex] ?? image.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
      favoritesSet.add(filename);
    }
    favoritesSet = new Set(favoritesSet);
  }
</script>

{#if items.length > 0}
  <div class="grid gap-2" bind:this={sectionRef}>
    <div class="flex justify-between items-center">
      <div class="flex items-center">
        <span class="section-indicator"></span>
        <span class="section-title">{headerText}</span>
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
    <div class="card-grid" class:collapsed={!expanded}>
      {#each items as img, i}
        <div class="card-wrapper" class:hidden={!expanded && i >= PREVIEW_COUNT}>
          <CollectionCard
            image={img}
            regionIndex={0}
            favorite={favoritesSet.has(img.urls[0].split("/").pop() ?? "")}
            onToggleFavorite={() => toggleFavorite(img, 0)}
          />
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
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

  .view-all-btn {
    border: 1px dashed #ddd;
    border-radius: 4px;
    padding: 4px 12px;
    color: #888;
    background: transparent;
    cursor: pointer;
    font-size: 12px;
  }

  .view-all-btn:hover {
    color: #666;
    border-color: #bbb;
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    overflow: hidden;
    transition: max-height 0.35s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .card-grid:not(.collapsed) {
    grid-template-columns: 1fr;
  }

  .card-wrapper {
    animation: fadeInUp 0.3s ease forwards;
  }

  .card-wrapper.hidden {
    display: none;
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
