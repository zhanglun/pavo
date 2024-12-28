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
  <Tabs tabStyle="underline" contentClass="p-4 pr-0 flex-1 overflow-y-auto overflow-x-hidden scrollbar-stable">
    <TabItem open title="Bing">
      <Bing />
    </TabItem>
    <TabItem title="Settings">
      <Settings />
    </TabItem>
    <TabItem title="About">
      <About />
    </TabItem>
  </Tabs>
</div>
