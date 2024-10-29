<!-- svelte-ignore a11y_autofocus -->
<script lang="ts">
  import { Badge } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import BingWallpaper from "../lib/components/BingWallpaper.svelte";

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
  let selectCountry = $state(navigator.language);

  function getBingWallpaper() {
    console.log(
      "🚀 ~ file: bing.svelte:38 ~ getBingWallpaper ~ selectCountry:",
      selectCountry,
    );

    invoke("get_bing_wallpaper_list", { page: 0, country: selectCountry }).then(
      (res) => {
        console.log("🚀 ~ file: bing.svelte:66 ~ getBingWallpaper ~ res:", res)
        images = res as BingImage[];
      },
    );
  }

  function switchCountry(country: string) {
    selectCountry = country;
    console.log(
      "🚀 ~ file: bing.svelte:39 ~ switchCountry ~ selectCountry:",
      selectCountry,
    );
    getBingWallpaper();
  }

  getBingWallpaper();
</script>

<div class="grid gap-2">
  <div class="flex gap-1 flex-wrap">
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
  <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-4">
    {#each images as image}
      <BingWallpaper {...image} />
    {/each}
  </div>
</div>
