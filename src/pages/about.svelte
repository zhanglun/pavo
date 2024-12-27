<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { A, Button } from "flowbite-svelte";
  import { getName, getVersion } from "@tauri-apps/api/app";
  import CheckCircleSolid from "flowbite-svelte-icons/CheckCircleSolid.svelte";

  let version = $state("");
  let name = $state("");

  $effect(() => {
    getVersion().then((res) => {
      console.log("🚀 ~ file: about.svelte:10 ~ getVersion ~ res:", res);
      version = res;
    });
    getName().then((res) => {
      console.log("🚀 ~ file: about.svelte:10 ~ getVersion ~ res:", res);
      name = res;
    });
  });
</script>

<div class="flex gap-2 flex-col">
  <div class="flex items-center justify-center mt-5">
    <img src="/icon.png" width="90px" alt={name} />
  </div>
  <div class="text-xl text-center">{name}</div>
  <div class="text-sm mt-6 flex gap-3 items-center justify-center">
    <span>version {version}</span><span
      ><CheckCircleSolid stroke-width="1.5" size="sm" color="green" /></span
    >
  </div>
  <div class="text-sm font-light flex gap-2 items-center justify-center mt-5">
    <A class="hover:underline" href="https://github.com/zhanglun/pavo"
      >Github Repo</A
    >
    <span class="w-[1px] h-3 bg-gray-200"></span>
    <A class="hover:underline" href="https://github.com/zhanglun/pavo/issues"
      >Report Issue</A
    >
  </div>
  <!-- <div class="flex gap-2 items-center justify-center">
    <Button size="sm">Check for Updates</Button>
  </div> -->
</div>
