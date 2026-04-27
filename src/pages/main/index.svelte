<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { Tabs, TabItem } from "flowbite-svelte";
  import Today from "./today.svelte";
  import Recent from "./recent.svelte";
  import Favorites from "./favorites.svelte";
  import Settings from "./settings.svelte";
  import { checkUpdate } from "../../lib/updater";

  let current = $state("Today");

  $effect(() => {
    const unlisteners: (() => void)[] = [];

    listen("go-to-settings", () => {
      current = "Settings";
    }).then((unlisten) => unlisteners.push(unlisten));

    listen("check-for-updates", () => {
      checkUpdate().then(() => {});
    }).then((unlisten) => unlisteners.push(unlisten));

    return () => {
      for (const unlisten of unlisteners) {
        unlisten();
      }
    };
  });
</script>

<div class="w-full h-full flex flex-col">
  <Tabs tabStyle="underline" contentClass="flex-1 h-0">
    <TabItem
      open={current === "Today"}
      title="Today"
      divClass="p-4 pr-2 h-full overflow-y-auto overflow-x-hidden scrollbar-stable"
      onclick={() => {
        current = "Today";
      }}
    >
      <Today />
    </TabItem>
    <TabItem
      open={current === "Recent"}
      title="Recent"
      divClass="p-4 pr-2 h-full overflow-y-auto overflow-x-hidden scrollbar-stable"
      onclick={() => {
        current = "Recent";
      }}
    >
      <Recent />
    </TabItem>
    <TabItem
      open={current === "Favorites"}
      title="Favorites"
      divClass="p-4 pr-2 h-full overflow-y-auto overflow-x-hidden scrollbar-stable"
      onclick={() => {
        current = "Favorites";
      }}
    >
      <Favorites />
    </TabItem>
    <TabItem
      open={current === "Settings"}
      title="Settings"
      divClass="p-4 h-full overflow-y-auto overflow-x-hidden scrollbar-stable"
      onclick={() => {
        current = "Settings";
      }}
    >
      <Settings />
    </TabItem>
  </Tabs>
</div>
