<script>
  import { Tabs, TabItem } from 'flowbite-svelte';
  import Bing from '../pages/bing.svelte';
  import Settings from '../pages/settings.svelte';
  import About from '../pages/about.svelte';
</script>
<Tabs tabStyle="underline" contentClass="p-4">
  <TabItem open title="Bing" >
    <Bing />
  </TabItem>
  <TabItem title="Settings">
    <Settings />
  </TabItem>
  <TabItem title="About">
    <About />
  </TabItem>
</Tabs>
