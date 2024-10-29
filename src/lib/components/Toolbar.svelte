<!-- svelte-ignore a11y_consider_explicit_label -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import DownloadOutline from "flowbite-svelte-icons/DownloadOutline.svelte";
  import DesktopPcOutline from "flowbite-svelte-icons/DesktopPcOutline.svelte";
  import ArrowUpRightFromSquareOutline from "flowbite-svelte-icons/ArrowUpRightFromSquareOutline.svelte";

  let { service, url, href } = $props<{
    service: string;
    url: string;
    href: string;
  }>();

  function setAsDesktop() {
    invoke("set_as_desktop", { service, url }).then((res) => {
      console.log(res);
    });
  }

  function download() {
    invoke("download", { service, url }).then((res) => {
      console.log(res);
    });
  }
</script>

<div>
  <div class="grid gap-2 grid-flow-col">
    <span
      class="
    py-1
    text-center
    cursor-pointer
    transition-[scale]
    origin-center
    text-neutral-800
    dark:text-white
    hover:text-black
    dark:hover:text-white
    hover:scale-110
  "
      onclick={setAsDesktop}
    >
    <DesktopPcOutline size="sm" />
    </span>
    <span
      class="
    py-1
    text-center
    cursor-pointer
    transition-[scale]
    origin-center
    text-neutral-800
    dark:text-white
    hover:text-black
    dark:hover:text-white
    hover:scale-110
  "
      onclick={download}
    >
      <DownloadOutline size="sm" />
    </span>

    <a
      class="
      py-1
      text-center
      cursor-pointer
      transition-[scale]
      origin-center
      text-neutral-800
      dark:text-white
      hover:text-black
      dark:hover:text-white
      hover:scale-110
    "
      target="_blank"
      {href}
    ><ArrowUpRightFromSquareOutline size="sm" /></a>
  </div>
</div>
