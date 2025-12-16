<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button, Checkbox, Label, Select } from "flowbite-svelte";

  let interval_options = [
    {
      label: "Every 1 Minutes",
      value: 1,
    },
    {
      label: "Every 5 Minutes",
      value: 5,
    },
    {
      label: "Every 10 Minutes",
      value: 10,
    },
    {
      label: "Every 15 Minutes",
      value: 15,
    },
    {
      label: "Every 30 Minutes",
      value: 30,
    },
    {
      label: "Every Hour",
      value: 60,
    },
    {
      label: "Every 2 Hours",
      value: 120,
    },
    {
      label: "Every 5 Hours",
      value: 300,
    },
    {
      label: "Every Day",
      value: 3600,
    },
  ];

  let config = $state<UserConfig>({} as UserConfig);

  function getUserConfig() {
    invoke("get_config").then((res) => {
      config = res as UserConfig;
      console.log("🚀 ~ getUserConfig ~ config:", config)
    });
  }

  getUserConfig();

  function updateConfigShuffle(key: string, value: boolean) {
    invoke("set_auto_shuffle", { shuffle: value }).then((res) => {
      console.log(res);
    });
  }

  function updateConfigShowLayer(key: string, value: boolean) {
    invoke("set_show_layer", { showLayer: value }).then((res) => {
      console.log(res);
    });
  }

  function updateConfigInterval(key: string, value: string) {
    invoke("set_interval", { interval: parseInt(value) }).then((res) => {
      console.log(res);
    });
  }

  async function handleRevealLog() {
    await invoke("reveal_log_file")
  }
</script>

<div class="flex gap-2 flex-col">
  <div class="flex gap-2">
    <Checkbox
      bind:checked={config.auto_shuffle as boolean}
      bind:value={config.auto_shuffle as any}
      on:change={(e) => {
        if (e.target) {
          const checked = (e.target as HTMLInputElement).checked;
          updateConfigShuffle("shuffle", checked);
        }
      }}>Shuffle</Checkbox
    >
  </div>
  <div class="flex gap-2">
    <Checkbox
      bind:checked={config.show_layer as boolean}
      bind:value={config.show_layer as any}
      on:change={(e) => {
        if (e.target) {
          const checked = (e.target as HTMLInputElement).checked;
          updateConfigShowLayer("show_layer", checked);
        }
      }}>Show desktop layer</Checkbox
    >
  </div>
  <div class="flex justify-between items-center">
    <Label for="interval" class="mb-2">Interval</Label>
    <Select
      id="interval"
      size="sm"
      class="w-1/2"
      bind:value={config.interval}
      on:change={(e) => {
        if (e.target) {
          const value = (e.target as HTMLSelectElement).value;
          updateConfigInterval("interval", value);
        }
      }}
    >
      {#each interval_options as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </Select>
  </div>
  <div class="flex justify-between items-center my-4">
    <Label for="log file" class="mb-2">Log file</Label>
    <Button size="sm" on:click={handleRevealLog}>Reveal</Button>
  </div>
</div>
