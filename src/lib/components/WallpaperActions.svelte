<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Tooltip } from "flowbite-svelte";
  import DownloadOutline from "flowbite-svelte-icons/DownloadOutline.svelte";
  import DesktopPcOutline from "flowbite-svelte-icons/DesktopPcOutline.svelte";
  import ArrowUpRightFromSquareOutline from "flowbite-svelte-icons/ArrowUpRightFromSquareOutline.svelte";
  import HeartOutline from "flowbite-svelte-icons/HeartOutline.svelte";
  import HeartSolid from "flowbite-svelte-icons/HeartSolid.svelte";

  let {
    service,
    url,
    href,
    filename,
    favorite = false,
    onToggleFavorite,
  }: {
    service: string;
    url: string;
    href: string;
    filename: string;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  async function setAsDesktop() {
    await invoke("set_as_desktop", { service, url });
  }

  async function saveLocally() {
    await invoke("download", { service, url });
  }
</script>

<div class="grid gap-2 grid-flow-col">
  <button
    type="button"
    class="py-1 text-center cursor-pointer transition-[scale] origin-center text-neutral-800 dark:text-white hover:text-black dark:hover:text-white hover:scale-110"
    onclick={setAsDesktop}
  >
    <DesktopPcOutline size="sm" />
  </button>
  <Tooltip class="text-xs text-nowrap">设为壁纸</Tooltip>

  <button
    type="button"
    class="py-1 text-center cursor-pointer transition-[scale] origin-center text-neutral-800 dark:text-white hover:text-black dark:hover:text-white hover:scale-110"
    onclick={saveLocally}
  >
    <DownloadOutline size="sm" />
  </button>
  <Tooltip class="text-xs text-nowrap">下载</Tooltip>

  <button
    type="button"
    class="py-1 text-center cursor-pointer transition-[scale] origin-center text-neutral-800 dark:text-white hover:text-black dark:hover:text-white hover:scale-110"
    onclick={() => onToggleFavorite()}
  >
    {#if favorite}
      <HeartSolid size="sm" class="text-red-500" />
    {:else}
      <HeartOutline size="sm" />
    {/if}
  </button>
  <Tooltip class="text-xs text-nowrap">{favorite ? "取消收藏" : "收藏"}</Tooltip>

  <a
    class="py-1 text-center cursor-pointer transition-[scale] origin-center text-neutral-800 dark:text-white hover:text-black dark:hover:text-white hover:scale-110"
    target="_blank"
    {href}
  >
    <ArrowUpRightFromSquareOutline size="sm" />
  </a>
  <Tooltip class="text-xs text-nowrap">在 Bing 中打开</Tooltip>
</div>
