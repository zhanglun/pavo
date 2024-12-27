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

<Tabs tabStyle="underline" contentClass="p-4">
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
