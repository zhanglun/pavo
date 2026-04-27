<script lang="ts">
  import { A } from "flowbite-svelte";
  import { getName, getVersion } from "@tauri-apps/api/app";
  import CheckCircleSolid from "flowbite-svelte-icons/CheckCircleSolid.svelte";

  let appName = $state("");
  let appVersion = $state("");

  $effect(() => {
    getName().then((n) => (appName = n));
    getVersion().then((v) => (appVersion = v));
  });
</script>

<div class="flex gap-2 flex-col items-center">
  <div class="flex items-center gap-2 mt-2">
    <img src="/icon.png" width="48px" alt={appName} />
    <div>
      <div class="text-sm font-medium">{appName}</div>
      <div class="text-xs text-neutral-500 flex items-center gap-1">
        <span>v{appVersion}</span>
        <CheckCircleSolid size="xs" color="green" />
      </div>
    </div>
  </div>
  <div class="text-xs text-neutral-400 flex gap-2 items-center justify-center">
    <A
      class="hover:underline text-xs"
      href="https://github.com/zhanglun/pavo"
      target="_blank"
    >
      GitHub
    </A>
    <span class="w-px h-3 bg-gray-300"></span>
    <A
      class="hover:underline text-xs"
      href="https://github.com/zhanglun/pavo/issues"
      target="_blank"
    >
      反馈问题
    </A>
  </div>
</div>
