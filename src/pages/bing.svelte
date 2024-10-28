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
      name: "日本",
      code: "ja-JP",
      icon: "🇯🇵",
    },
  ];

  let images = $state([]);
  let selectCountry = $state(navigator.language || navigator.userLanguage);

  console.log(window);

  function getBingWallpaper() {
    console.log(
      "🚀 ~ file: bing.svelte:38 ~ getBingWallpaper ~ selectCountry:",
      selectCountry,
    );

    invoke("get_bing_wallpaper_list", { page: 0, country: selectCountry }).then(
      (res) => {
        console.log(
          "🚀 ~ file: bing.svelte:32 ~ getBingWallpaper ~ res:",
          res[0],
        );
        images = res;
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
  <div class="flex gap-1">
    {#each country as country}
      <Badge
        large
        color={selectCountry === country.code ? "green" : "dark"}
        class="cursor-pointer"
      >
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
