<!-- svelte-ignore a11y_autofocus -->
<script lang="ts">
  import { Hr } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import BingWallpaper from "../lib/components/BingWallpaper.svelte";
  import QuoteSolid from "flowbite-svelte-icons/QuoteSolid.svelte";
  import Skeleton from "$lib/components/Skeleton.svelte";

  let images = $state<BingImage[]>([]);
  let loading = $state<boolean>(true);

  function getBingWallpaper() {
    loading = true;

    invoke("get_bing_wallpaper_list").then((res) => {
      images = (res as BingImage[]).sort((a, b) =>
        b.startdates[0] > a.startdates[0] ? 1 : -1,
      );
      console.log(
        "🚀 ~ file: bing.svelte:65 ~ getBingWallpaper ~ images:",
        images,
      );
      loading = false;
    });
  }

  getBingWallpaper();
</script>

<div class="grid gap-2">
  <div class="mt-5 {loading ? 'opacity-70' : ''}">
    {#if loading}
      {#each images as image}
        <Skeleton />
        <Hr classHr="my-8 w-64 h-1" icon>
          <QuoteSolid class="w-6 h-6 text-gray-700 dark:text-gray-300" />
        </Hr>
      {/each}
    {:else}
      {#each images as image}
        <BingWallpaper {...image} />
        <Hr classHr="my-8 w-64 h-1" icon>
          <QuoteSolid class="w-6 h-6 text-gray-700 dark:text-gray-300" />
        </Hr>
      {/each}
    {/if}
  </div>
</div>
