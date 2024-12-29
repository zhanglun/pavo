<script>
  import { listen } from "@tauri-apps/api/event";
  import { Tabs, TabItem } from "flowbite-svelte";
  import Bing from "../pages/bing.svelte";
  import Settings from "../pages/settings.svelte";
  import About from "../pages/about.svelte";

  let current = $state("Bing");

  $effect(() => {
    const sub = () => {
      listen("go-to-about", (event) => {
        current = "About";
      });
    };
  });
</script>

<div class="w-full h-full flex flex-col">
  <Tabs tabStyle="underline" contentClass="flex-1 h-0">
    <TabItem open title="Bing" divClass="p-4 pr-0 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <Bing />
    </TabItem>
    <TabItem title="Settings" divClass="p-4 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <Settings />
    </TabItem>
    <TabItem title="About" divClass="p-4 h-full overflow-y-auto overflow-x-hidden scrollbar-stable">
      <About />
    </TabItem>
  </Tabs>
</div>
