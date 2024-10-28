<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import BingWallpaper from '../lib/components/BingWallpaper.svelte'

  console.log("bing svelte init")

  let images = $state([])

  console.log(window)
  function getBingWallpaper() {
    invoke("get_bing_wallpaper_list", { page: 0}).then((res) => {
      images = res
    })
  }

  getBingWallpaper()
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-4">
  {#each images as image}
    <BingWallpaper {...image} />
  {/each}
</div>
