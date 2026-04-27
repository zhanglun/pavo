<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button, Checkbox, Label, Select, A } from "flowbite-svelte";
  import { getName, getVersion } from "@tauri-apps/api/app";
  import CheckCircleSolid from "flowbite-svelte-icons/CheckCircleSolid.svelte";
  import { checkUpdate } from "../../lib/updater";

  let config = $state<UserConfig>({} as UserConfig);
  let appName = $state("");
  let appVersion = $state("");

  function getUserConfig() {
    invoke("get_config").then((res) => {
      config = res as UserConfig;
    });
  }

  getUserConfig();

  $effect(() => {
    getName().then((n) => (appName = n));
    getVersion().then((v) => (appVersion = v));
  });

  async function updateAutoDailyUpdate(enabled: boolean) {
    await invoke("set_auto_daily_update", { enabled });
  }

  async function updateShowLayer(showLayer: boolean) {
    await invoke("set_show_layer", { showLayer });
  }

  async function updateHistoryRange(days: number) {
    await invoke("set_history_range_days", { days });
  }

  async function handleRevealLog() {
    await invoke("reveal_log_file");
  }

  async function handleCheckUpdate() {
    await checkUpdate();
  }
</script>

<div class="flex gap-4 flex-col">
  <div class="grid gap-3">
    <Checkbox
      bind:checked={config.auto_daily_update as boolean}
      on:change={(e) => {
        if (e.target) {
          const checked = (e.target as HTMLInputElement).checked;
          updateAutoDailyUpdate(checked);
        }
      }}>每日自动更新</Checkbox
    >

    <div class="flex justify-between items-center">
      <Label for="history_range" class="mb-0">历史范围</Label>
      <Select
        id="history_range"
        size="sm"
        class="w-1/2"
        bind:value={config.history_range_days}
        on:change={(e) => {
          if (e.target) {
            const value = Number((e.target as HTMLSelectElement).value);
            updateHistoryRange(value);
          }
        }}
      >
        <option value={7}>最近 7 天</option>
        <option value={14}>最近 14 天</option>
      </Select>
    </div>

    <div class="text-xs text-neutral-500 pl-1">
      缓存位置：~/.pavo/ — 壁纸图片会自动保存到本地以便离线查看
    </div>
  </div>


  <div class="border-t border-gray-200 dark:border-gray-700"></div>

  <div class="grid gap-3">
    <div class="text-xs font-medium text-neutral-400 uppercase tracking-wide">高级设置</div>

    <Checkbox
      bind:checked={config.show_layer as boolean}
      on:change={(e) => {
        if (e.target) {
          const checked = (e.target as HTMLInputElement).checked;
          updateShowLayer(checked);
        }
      }}>显示桌面信息层</Checkbox
    >

    <div class="flex justify-between items-center">
      <Label for="check_update" class="mb-0">检查更新</Label>
      <Button size="sm" on:click={handleCheckUpdate}>检查</Button>
    </div>

    <div class="flex justify-between items-center">
      <Label for="log_file" class="mb-0">日志文件</Label>
      <Button size="sm" on:click={handleRevealLog}>打开</Button>
    </div>
  </div>


  <div class="border-t border-gray-200 dark:border-gray-700"></div>

  <div class="grid gap-2">
    <div class="flex items-center gap-2 justify-center">
      <img src="/icon.png" width="48px" alt={appName} />
      <div>
        <div class="text-sm font-medium">{appName}</div>
        <div class="text-xs text-neutral-500 flex items-center gap-1">
          <span>v{appVersion}</span>
          <CheckCircleSolid size="xs" color="green" />
        </div>
      </div>
    </div>
    <div class="text-xs text-neutral-400 flex gap-2 items-center justify-center">
      <A
        class="hover:underline text-xs"
        href="https://github.com/zhanglun/pavo"
        target="_blank"
      >
        GitHub
      </A>
      <span class="w-px h-3 bg-gray-300"></span>
      <A
        class="hover:underline text-xs"
        href="https://github.com/zhanglun/pavo/issues"
        target="_blank"
      >
        反馈问题
      </A>
    </div>
  </div>
</div>
