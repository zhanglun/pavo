<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Checkbox, Label, Select } from "flowbite-svelte";

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

  let config = $state<UserConfig>({});

  function getUserConfig() {
    invoke("get_config").then((res) => {
      config = res;
    });
  }

  getUserConfig();

  function updateConfigShuffle(key, value) {
    //config[key] = value
    invoke("set_auto_shuffle", { shuffle: value }).then((res) => {
      console.log(res);
    });
  }

  function updateConfigInterval(key, value) {
    invoke("set_interval", { interval: parseInt(value) }).then((res) => {
      console.log(res);
    });
  }
</script>

<div class="flex gap-2 flex-col">
  <div class="flex gap-2">
    <Checkbox
      bind:checked={config.auto_shuffle}
      bind:value={config.auto_shuffle}
      on:change={(e) => updateConfigShuffle("shuffle", e.target.checked)}
      >Shuffle</Checkbox
    >
  </div>
  <div>
    <Label for="interval" class="mb-2">Interval</Label>
    <Select
      id="interval"
      size="sm"
      bind:value={config.interval}
      on:change={(e) => updateConfigInterval("interval", e.target.value)}
    >
      {#each interval_options as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </Select>
  </div>
</div>
