<script lang="ts">
  import CollectionCard from "./CollectionCard.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { tick } from "svelte";
  import ChevronRight from "./icons/chevron-right.svelte";

  let {
    favoritesSet,
    refreshKey,
  }: {
    favoritesSet: Set<string>;
    refreshKey: number;
  } = $props();

  let expanded = $state(false);
  let items: BingImage[] = $state([]);
  let sectionRef: HTMLDivElement | undefined = $state();

  const PREVIEW_COUNT = 4;

  $effect(() => {
    refreshKey;
    invoke<BingImage[]>("get_recent_wallpapers", { days: 7 }).then((data) => {
      items = data;
    });
  });

  let totalCount = $derived(items.length);
  let hasMore = $derived(totalCount > PREVIEW_COUNT);

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
  <div class="section" bind:this={sectionRef}>
    <div class="header">
      <span class="title">近期合集</span>
      {#if hasMore}
        <button type="button" class="toggle-btn" onclick={toggleExpand}>
          {expanded ? "收起" : "查看全部"}
          <ChevronRight size={12} class="chevron {expanded ? 'chevron-up' : ''}" />
        </button>
      {/if}
    </div>
    <div class="card-grid">
      {#each items as img, i}
        {#if expanded || i < PREVIEW_COUNT}
          <CollectionCard
            image={img}
            regionIndex={0}
            favorite={favoritesSet.has(img.urls[0].split("/").pop() ?? "")}
            onToggleFavorite={() => toggleFavorite(img, 0)}
          />
        {/if}
      {/each}
    </div>
  </div>
{/if}

<style>
  .section {
    display: grid;
    gap: 8px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    gap: 2px;
    font-size: 11px;
    color: var(--accent);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .toggle-btn:hover {
    opacity: 0.8;
  }

  .toggle-btn :global(.chevron) {
    transition: transform 0.2s ease;
    transform: rotate(90deg);
  }

  .toggle-btn :global(.chevron-up) {
    transform: rotate(-90deg);
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 4px;
  }
</style>
