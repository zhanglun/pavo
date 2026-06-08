<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getName, getVersion } from "@tauri-apps/api/app";
  import { open as openLink } from "@tauri-apps/plugin-shell";
  import { isEnabled } from "@tauri-apps/plugin-autostart";
  import { checkUpdate } from "../../lib/updater";
  import X from "./icons/x.svelte";

  let {
    open = false,
    onClose,
  }: {
    open: boolean;
    onClose: () => void;
  } = $props();

  let config = $state<UserConfig>({} as UserConfig);
  let appName = $state("");
  let appVersion = $state("");
  let theme = $state<"light" | "dark">("light");

  async function getUserConfig() {
    const res = await invoke("get_config");
    let cfg = res as UserConfig;
    try {
      const osEnabled = await isEnabled();
      if (cfg.auto_start !== osEnabled) {
        await invoke("set_auto_start", { enabled: osEnabled });
        cfg.auto_start = osEnabled;
      }
    } catch {}
    config = cfg;
  }

  $effect(() => {
    if (open) {
      getUserConfig();
      getName().then((n) => (appName = n));
      getVersion().then((v) => (appVersion = v));
    }
  });

  async function updateAutoDailyUpdate(enabled: boolean) {
    await invoke("set_auto_daily_update", { enabled });
  }

  async function updateHistoryRange(days: number) {
    await invoke("set_history_range_days", { days });
  }

  async function updateAutoRotate(enabled: boolean) {
    await invoke("set_auto_rotate", { enabled });
  }

  async function updateRotateInterval(minutes: number) {
    await invoke("set_rotate_interval", { minutes });
  }

  async function updateRotateMode(mode: string) {
    await invoke("set_rotate_mode", { mode });
  }

  async function updateAutoStart(enabled: boolean) {
    await invoke("set_auto_start", { enabled });
  }

  async function updateCacheRetentionDays(days: number) {
    await invoke("set_cache_retention_days", { days });
  }

  async function handleRevealLog() {
    await invoke("reveal_log_file");
  }

  async function handleCheckUpdate() {
    await checkUpdate();
  }

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    const app = document.getElementById("app");
    if (app) {
      app.classList.remove("theme-light", "theme-dark");
      app.classList.add(`theme-${theme}`);
    }
  }

  function handleOpenUrl(url: string) {
    openLink(url);
  }

  $effect(() => {
    const app = document.getElementById("app");
    if (app) {
      if (app.classList.contains("theme-dark")) {
        theme = "dark";
      }
    }
  });
</script>

{#if open}
  <div class="overlay" onclick={onClose} role="presentation"></div>

  <div class="panel scrollbar-stable">
    <div class="panel-inner">
      <div class="topbar">
        <span class="topbar-title">设置</span>
        <button type="button" class="close-btn" onclick={onClose}><X size={16} /></button>
      </div>

      <div class="section">
        <div class="row">
          <span class="row-label">外观主题</span>
          <button type="button" class="theme-toggle-btn" onclick={toggleTheme}>
            <span class="theme-label">{theme === "light" ? "浅色" : "深色"}</span>
            <span class="theme-icon">{theme === "light" ? "☀" : "☾"}</span>
          </button>
        </div>

        <label class="toggle-row">
          <input
            type="checkbox"
            bind:checked={config.auto_daily_update}
            onchange={(e) => {
              updateAutoDailyUpdate((e.target as HTMLInputElement).checked);
            }}
          />
          <span>每日自动更新</span>
        </label>

        <label class="toggle-row">
          <input
            type="checkbox"
            bind:checked={config.auto_start}
            onchange={(e) => {
              updateAutoStart((e.target as HTMLInputElement).checked);
            }}
          />
          <span>开机启动</span>
        </label>

        <div class="subsection">
          <label class="toggle-row">
            <input
              type="checkbox"
              bind:checked={config.auto_rotate}
              onchange={(e) => {
                updateAutoRotate((e.target as HTMLInputElement).checked);
              }}
            />
            <span>自动轮播壁纸</span>
          </label>

          {#if config.auto_rotate}
            <div class="sub-options">
              <div class="row">
                <span class="row-label">切换间隔</span>
                <select
                  class="settings-select"
                  bind:value={config.rotate_interval_minutes}
                  onchange={(e) => {
                    updateRotateInterval(Number((e.target as HTMLSelectElement).value));
                  }}
                >
                  <option value={10}>10 分钟</option>
                  <option value={30}>30 分钟</option>
                  <option value={60}>1 小时</option>
                  <option value={120}>2 小时</option>
                  <option value={360}>6 小时</option>
                  <option value={720}>12 小时</option>
                  <option value={1440}>24 小时</option>
                </select>
              </div>

              <div class="row">
                <span class="row-label">轮播模式</span>
                <div class="radio-group">
                  <label class="radio-label">
                    <input
                      type="radio"
                      name="rotate-mode"
                      value="Sequential"
                      bind:group={config.rotate_mode}
                      onchange={() => updateRotateMode("Sequential")}
                    />
                    <span>顺序</span>
                  </label>
                  <label class="radio-label">
                    <input
                      type="radio"
                      name="rotate-mode"
                      value="Random"
                      bind:group={config.rotate_mode}
                      onchange={() => updateRotateMode("Random")}
                    />
                    <span>随机</span>
                  </label>
                </div>
              </div>
            </div>
          {/if}
        </div>

        <div class="row">
          <span class="row-label">历史范围</span>
          <select
            class="settings-select"
            bind:value={config.history_range_days}
            onchange={(e) => {
              updateHistoryRange(Number((e.target as HTMLSelectElement).value));
            }}
          >
            <option value={7}>最近 7 天</option>
            <option value={14}>最近 14 天</option>
          </select>
        </div>

        <div class="row">
          <span class="row-label">缓存保留天数</span>
          <select
            class="settings-select"
            bind:value={config.cache_retention_days}
            onchange={(e) => {
              updateCacheRetentionDays(Number((e.target as HTMLSelectElement).value));
            }}
          >
            <option value={3}>3 天</option>
            <option value={7}>7 天</option>
            <option value={14}>14 天</option>
            <option value={30}>30 天</option>
          </select>
        </div>

        <div class="hint-text">
          缓存位置：~/.pavo/ — 壁纸图片会自动保存到本地以便离线查看
        </div>
      </div>

      <div class="divider"></div>

      <div class="section">
        <div class="section-title">高级设置</div>

        <div class="row">
          <span class="row-label">检查更新</span>
          <button type="button" class="settings-btn" onclick={handleCheckUpdate}>检查</button>
        </div>

        <div class="row">
          <span class="row-label">日志文件</span>
          <button type="button" class="settings-btn" onclick={handleRevealLog}>打开</button>
        </div>
      </div>

      <div class="divider"></div>

      <div class="section about-section">
        <div class="about-info">
          <img src="/icon.png" width="48px" alt={appName} />
          <div>
            <div class="about-name">{appName}</div>
            <div class="about-version">
              <span>v{appVersion}</span>
              <span class="version-dot"></span>
            </div>
          </div>
        </div>
        <div class="about-links">
          <button type="button" class="link-btn" onclick={() => handleOpenUrl("https://github.com/zhanglun/pavo")}>
            GitHub
          </button>
          <span class="link-sep"></span>
          <button type="button" class="link-btn" onclick={() => handleOpenUrl("https://github.com/zhanglun/pavo/issues")}>
            反馈问题
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 遮罩 */
  .overlay {
    position: fixed;
    inset: 0;
    background-color: var(--overlay);
    z-index: 50;
    border-radius: 12px;
  }

  /* 面板 */
  .panel {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    z-index: 50;
    overflow-y: auto;
    background-color: var(--bg);
    border-radius: 12px;
  }

  .panel-inner {
    padding: 16px;
    display: grid;
    gap: 16px;
  }

  /* 顶部栏 */
  .topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .topbar-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .close-btn {
    font-size: 16px;
    color: var(--text-tertiary);
    cursor: pointer;
    background: none;
    border: none;
    padding: 4px;
    line-height: 1;
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  /* 区块 */
  .section {
    display: grid;
    gap: 12px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* 行 */
  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .row-label {
    font-size: 13px;
    color: var(--text-primary);
  }

  /* 分割线 */
  .divider {
    border-top: 1px solid var(--border);
  }

  /* 提示文字 */
  .hint-text {
    font-size: 11px;
    color: var(--text-tertiary);
    padding-left: 4px;
  }

  /* 主题切换按钮 */
  .theme-toggle-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 12px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .theme-toggle-btn:hover {
    background: var(--menu-hover);
  }

  .theme-label {
    font-size: 12px;
  }

  .theme-icon {
    font-size: 13px;
  }

  /* Checkbox toggle 行 */
  .toggle-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--text-primary);
    cursor: pointer;
    user-select: none;
  }

  .toggle-row input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
    cursor: pointer;
    flex-shrink: 0;
  }

  /* Select */
  .settings-select {
    font-size: 13px;
    padding: 4px 8px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    background: var(--bg-secondary);
    color: var(--text-primary);
    cursor: pointer;
    outline: none;
    transition: border-color 0.15s;
  }

  .settings-select:focus {
    border-color: var(--accent);
  }

  .subsection {
    display: grid;
    gap: 8px;
  }

  .sub-options {
    display: grid;
    gap: 8px;
    padding-left: 24px;
  }

  .radio-group {
    display: flex;
    gap: 12px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-primary);
    cursor: pointer;
    user-select: none;
  }

  .radio-label input[type="radio"] {
    accent-color: var(--accent);
    cursor: pointer;
  }

  /* 按钮 */
  .settings-btn {
    font-size: 12px;
    padding: 4px 12px;
    border-radius: 6px;
    border: 1px solid var(--border-strong);
    background: var(--bg-secondary);
    color: var(--text-primary);
    cursor: pointer;
    transition: background-color 0.15s, border-color 0.15s;
  }

  .settings-btn:hover {
    background: var(--menu-hover);
  }

  .settings-btn:active {
    border-color: var(--accent);
  }

  /* 关于 */
  .about-section {
    gap: 12px;
  }

  .about-info {
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
  }

  .about-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .about-version {
    font-size: 11px;
    color: var(--text-tertiary);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .version-dot {
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--accent);
  }

  .about-links {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 11px;
    color: var(--text-tertiary);
  }

  .link-btn {
    background: none;
    border: none;
    color: var(--text-tertiary);
    font-size: 11px;
    cursor: pointer;
    padding: 0;
    transition: color 0.15s;
  }

  .link-btn:hover {
    color: var(--text-primary);
    text-decoration: underline;
  }

  .link-sep {
    width: 1px;
    height: 10px;
    background-color: var(--border-strong);
  }
</style>
