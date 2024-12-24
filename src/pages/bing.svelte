<!-- svelte-ignore a11y_autofocus -->
<script lang="ts">
  import { Badge, Hr } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import BingWallpaper from "../lib/components/BingWallpaper.svelte";
  import QuoteSolid from "flowbite-svelte-icons/QuoteSolid.svelte"
  import Skeleton from "$lib/components/Skeleton.svelte";

  const country = [
    {
      name: "中国",
      code: "zh-CN",
      icon: "🇨🇳",
    },
    {
      name: "United States",
      code: "en-US",
      icon: "🇺🇸",
    },
    {
      name: "France",
      code: "fr-FR",
      icon: "🇫🇷",
    },
    {
      name: " Deutschland",
      code: "de-DE",
      icon: "🇩🇪",
    },
    {
      name: "日本",
      code: "ja-JP",
      icon: "🇯🇵",
    },
    {
      name: "Canada",
      code: "en-CA",
      icon: "🇨🇦",
    },
    {
      name: "United Kingdom",
      code: "en-GB",
      icon: "🇬🇧",
    },
    {
      name: "India",
      code: "en-IN",
      icon: "🇮🇳",
    },
    {
      name: "Italy",
      code: "it-IT",
      icon: "🇮🇹",
    },
  ];

  let images = $state<BingImage[]>([]);
  let loading = $state<boolean>(true);
  let selectCountry = $state(navigator.language);

  function getBingWallpaper() {
    loading = true

    invoke("get_bing_wallpaper_list", { page: 0, country: selectCountry }).then(
      (res) => {
        images = res as BingImage[];
        console.log("🚀 ~ file: bing.svelte:65 ~ getBingWallpaper ~ images:", images)
        loading = false
      },
    );
  }

  function switchCountry(country: string) {
    selectCountry = country;

    getBingWallpaper();
  }

  getBingWallpaper();
</script>

<div class="grid gap-2">
  <div class="flex gap-1 flex-wrap mb-2">
    {#each country as country}
      <Badge
        large
        color={selectCountry === country.code ? "green" : "dark"}
        class="cursor-pointer"
      >
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <span onclick={() => switchCountry(country.code)} class="select-none">
          {country.icon}
          {country.name}
        </span>
      </Badge>
    {/each}
  </div>
  <div class="mt-5 {loading ? 'opacity-70': ''}">
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
