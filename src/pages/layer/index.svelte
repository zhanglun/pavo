<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let meta = $state({ title: "", copyright: "", startdate: "" });

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      unlisten = await listen<
        { title: string; copyright: string; url: string; startdate: string }
      >("wallpaper:changed", (event) => {
        meta.title = event.payload.title;
        meta.copyright = event.payload.copyright;
        meta.startdate = event.payload.startdate;
      });
    })();

    return () => {
      unlisten?.();
    };
  });
</script>

<div class="flex gap-2 flex-col box">
  <span class="title">{meta.title}</span>
  <span class="copyright">
  {#if meta.startdate}
  <span class="date">{`${meta.startdate.slice(0, 4)}-${meta.startdate.slice(4, 6)}-${meta.startdate.slice(6, 8)}`}</span>
  {meta.copyright}
  {/if}
  </span>
</div>
