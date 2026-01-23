<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let meta = $state({ title: "", copyright: "", startdate: "" });

  onMount(async () => {
    const unlisten = await listen<{ title: string; copyright: string; url: string; startdate: string }>(
      "wallpaper:changed",
      (event) => {
        meta.title = event.payload.title;
        meta.copyright = event.payload.copyright;
        meta.startdate = event.payload.startdate;
      }
    );

    return () => {
      unlisten();
    };
  });
</script>

<div class="flex gap-2 flex-col">
  <span class="title">{meta.title}</span>
  <span class="copyright">{meta.copyright}</span>
</div>
