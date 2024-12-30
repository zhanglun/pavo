<script>
  import { listen } from "@tauri-apps/api/event";
  import { Tabs, TabItem } from "flowbite-svelte";
  import Bing from "../pages/bing.svelte";
  import Settings from "../pages/settings.svelte";
  import About from "../pages/about.svelte";
  import { checkUpdate } from "$lib/updater";

  let current = $state("Bing");

  $effect(() => {
    const sub = () => {
      listen("go-to-about", (event) => {
        current = "About";
      });

      listen("go-to-settings", (event) => {
        current = "Settings";
      });

      listen("check-for-updates", (event) => {
        console.log("🚀 ~ file: +page.svelte:22 ~ listen ~ check-for-update:")
        checkUpdate().then(() => {

        })
      });
    };

    return sub();
  });
</script>

<div class="w-full h-full flex flex-col">
  <Tabs tabStyle="underline" contentClass="flex-1 h-0">
    <TabItem open={current === "Bing"} title="Bing" divClass="p-4 pr-2 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <Bing />
    </TabItem>
    <TabItem open={current === "Settings"} title="Settings" divClass="p-4 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <Settings />
    </TabItem>
    <TabItem open={current === "About"} title="About" divClass="p-4 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <About />
    </TabItem>
  </Tabs>
</div>
