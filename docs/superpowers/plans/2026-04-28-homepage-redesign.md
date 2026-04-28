# 首页重构：方案 E 实施

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 Pavo 主窗口从 4-Tab 导航改为单页面，首页以 Hero + 今日各地区合集（可展开）为核心，Recent/Favorites 降级为首页下方紧凑列表，Settings 变为右上角轻量入口。

**Architecture:** 单页面滚动结构，顶部为今日壁纸 Hero 大图，紧接着是今日各地区合集 section（默认展示 4 张，点击"查看全部"原地展开完整列表），下方依次是 Recent 预览和 Favorites 预览。Settings 通过右上角齿轮图标触发轻量面板（overlay 或滑出）。后端需新增一个命令获取"今日各地区合集"数据。前端组件拆分为：HomeShell（壳）、TodayHero、TodayCollection（含展开逻辑）、RecentPreview、FavoritesPreview、SettingsPanel。

**Tech Stack:** Svelte 5（runes）、Tailwind CSS（遵循现有工具类风格）、Flowbite-Svelte（仅保留必要的 Checkbox/Select）、Tauri 2 invoke API。

**风格约束（极重要）：**
- 圆角使用 `rounded`（4px），不要用 `rounded-xl`/`rounded-2xl`/`rounded-3xl`
- 间距使用 `gap-2`/`gap-3`/`gap-4`，和现有一致
- 不要加 box-shadow 除非现有代码里已有
- 不要加 backdrop-blur、渐变遮罩等装饰效果（除非现有 Hero 代码里已有）
- 按钮/输入框保持 Flowbite 默认样式
- 所有文案使用简体中文
- 代码注释使用简体中文

---

## 文件结构

| 操作 | 文件路径 | 职责 |
|------|----------|------|
| 修改 | `src-tauri/src/cmd.rs` | 新增 `get_today_collection` 命令 |
| 修改 | `src-tauri/src/scheduler.rs` | 新增 `filter_today` 方法 |
| 新增 | `src/typing.d.ts` 补充 | 新增 `TodayCollectionItem` 类型（复用 `SchedulerPhoto` 即可） |
| 新增 | `src/lib/components/TodayHero.svelte` | 今日 Hero 大图（从现有 `WallpaperHero.svelte` 简化） |
| 新增 | `src/lib/components/TodayCollection.svelte` | 今日各地区合集 section，含展开/收起逻辑 |
| 新增 | `src/lib/components/CollectionCard.svelte` | 合集内单张壁纸卡片 |
| 新增 | `src/lib/components/RecentPreview.svelte` | Recent 紧凑预览列表 |
| 新增 | `src/lib/components/FavoritesPreview.svelte` | Favorites 紧凑预览列表 |
| 新增 | `src/lib/components/SettingsPanel.svelte` | Settings 轻量面板（overlay） |
| 修改 | `src/pages/main/index.svelte` | 从 Tab 导航改为单页面 |
| 删除 | `src/pages/main/today.svelte` | 合并入新的首页 |
| 删除 | `src/pages/main/recent.svelte` | 合并入 RecentPreview |
| 删除 | `src/pages/main/favorites.svelte` | 合并入 FavoritesPreview |
| 删除 | `src/pages/main/settings.svelte` | 合并入 SettingsPanel |
| 删除 | `src/pages/main/about.svelte` | 已确认是孤立页面 |
| 删除 | `src/pages/main/bing.svelte` | 已确认是过时页面 |
| 删除 | `src/lib/components/BingWallpaper.svelte` | 旧体系组件 |
| 删除 | `src/lib/components/Toolbar.svelte` | 旧体系组件 |
| 保留 | `src/lib/components/WallpaperHero.svelte` | 被新组件替代但先保留，确认无引用后删 |
| 保留 | `src/lib/components/WallpaperActions.svelte` | 通用操作按钮，继续使用 |
| 保留 | `src/lib/components/Skeleton.svelte` | 加载态骨架屏，继续使用 |

---

## Task 1: 后端 — 新增 `get_today_collection` 命令

**Files:**
- 修改: `src-tauri/src/scheduler.rs`
- 修改: `src-tauri/src/cmd.rs`

**背景:** 现有 `get_today_wallpaper` 只返回 1 张（`pick_today`），但首页需要展示"今日各地区合集"——即 `startdates` 中包含今天日期的所有 `SchedulerPhoto`，无论地区。如果某张壁纸在多个地区出现，它已经被 `batch_fetch` 合并了，`regions` 会包含多个地区代码。

- [ ] **Step 1: 在 `scheduler.rs` 中新增 `filter_today` 方法**

在 `impl Scheduler` 块中，`pick_today` 方法之后添加：

```rust
/// 从列表中筛选 startdate 包含指定日期的所有 SchedulerPhoto
pub fn filter_today(list: &[SchedulerPhoto], today: &str) -> Vec<SchedulerPhoto> {
    list.iter()
        .filter(|photo| {
            photo
                .startdates
                .iter()
                .any(|sd| sd.as_str() == today)
        })
        .cloned()
        .collect()
}
```

- [ ] **Step 2: 在 `scheduler.rs` 的测试模块中新增测试**

在 `mod scheduler_tests` 中添加：

```rust
#[test]
fn filter_today_returns_all_matching_date() {
    let list = vec![
        photo("a", "20260427"),
        photo("b", "20260427"),
        photo("c", "20260426"),
        photo("d", "20260427"),
    ];

    let today_list = Scheduler::filter_today(&list, "20260427");
    assert_eq!(today_list.len(), 3);
    assert_eq!(
        today_list.iter().map(|p| p.filename.clone()).collect::<Vec<_>>(),
        vec!["a", "b", "d"]
    );
}

#[test]
fn filter_today_empty_when_no_match() {
    let list = vec![photo("a", "20260426")];
    let today_list = Scheduler::filter_today(&list, "20260427");
    assert!(today_list.is_empty());
}
```

- [ ] **Step 3: 运行测试确认通过**

Run: `cargo test --manifest-path src-tauri/Cargo.toml scheduler_tests`
Expected: 全部 PASS

- [ ] **Step 4: 在 `cmd.rs` 中新增命令**

在 `cmd.rs` 的 `get_recent_wallpapers` 之后添加：

```rust
#[tauri::command]
pub async fn get_today_collection() -> Vec<scheduler::SchedulerPhoto> {
    let mut scheduler = scheduler::SCHEDULER.lock().await;
    let list = scheduler.batch_fetch().await.unwrap_or_default();
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    scheduler::Scheduler::filter_today(&list, &today)
}
```

- [ ] **Step 5: 在 `src-tauri/src/lib.rs` 中注册新命令**

找到现有的 `invoke_handler` 宏中的命令列表，在 `get_recent_wallpapers` 之后添加 `get_today_collection`。

- [ ] **Step 6: 运行编译确认无错误**

Run: `cargo build --manifest-path src-tauri/Cargo.toml`
Expected: 编译成功

- [ ] **Step 7: 提交**

```bash
git add src-tauri/src/scheduler.rs src-tauri/src/cmd.rs src-tauri/src/lib.rs
git commit -m "feat: 新增 get_today_collection 命令，返回今日各地区合集"
```

---

## Task 2: 前端 — 新增 `TodayHero.svelte` 组件

**Files:**
- 新增: `src/lib/components/TodayHero.svelte`

**背景:** 这是对现有 `WallpaperHero.svelte` 的简化版，直接放在首页顶部。保持现有风格：`rounded`、`gap-1`/`gap-3`、无多余装饰。

- [ ] **Step 1: 创建 `TodayHero.svelte`**

```svelte
<script lang="ts">
  import WallpaperActions from "./WallpaperActions.svelte";

  let {
    image,
    favorite = false,
    onToggleFavorite,
  }: {
    image: BingImage;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  let url = $derived(image.urls[0]);
  let title = $derived(image.titles[0]);
  let startdate = $derived(image.startdates[0]);
  let copyright = $derived(image.copyrights[0]);
  let copyrightlink = $derived(image.copyrightlinks[0]);
  let filename = $derived(url.split("/").pop() ?? "");
  let formattedDate = $derived(
    `${startdate.slice(0, 4)}-${startdate.slice(4, 6)}-${startdate.slice(6, 8)}`
  );
</script>

<div class="grid gap-1">
  <div
    class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
  >
    <img
      class="w-full rounded transition-all group-hover:scale-105"
      src={url}
      alt={title}
    />
    <div
      class="p-2 bg-gradient-to-r from-black/50 to-black/0 rounded overflow-hidden absolute left-0 right-0 bottom-0 top-0"
    ></div>
  </div>
  <div class="grid gap-1 mt-3">
    <div class="flex justify-between items-center">
      <div class="font-semibold truncate mr-2">{title}</div>
      <WallpaperActions
        service="Bing"
        {url}
        href={copyrightlink}
        {filename}
        {favorite}
        {onToggleFavorite}
      />
    </div>
    <div class="text-xs text-neutral-700">
      <span class="font-normal text-xs text-right text-neutral-700"
        >{formattedDate}</span
      >
      {copyright}
    </div>
  </div>
</div>
```

- [ ] **Step 2: 提交**

```bash
git add src/lib/components/TodayHero.svelte
git commit -m "feat: 新增 TodayHero 组件"
```

---

## Task 3: 前端 — 新增 `CollectionCard.svelte` 组件

**Files:**
- 新增: `src/lib/components/CollectionCard.svelte`

**背景:** 今日合集里的单张壁纸卡片。紧凑布局：缩略图 + 标题 + 操作按钮。风格和现有 `WallpaperHero` 保持一致，但更紧凑（无大图模式）。

- [ ] **Step 1: 创建 `CollectionCard.svelte`**

```svelte
<script lang="ts">
  import WallpaperActions from "./WallpaperActions.svelte";

  let {
    image,
    regionIndex = 0,
    favorite = false,
    onToggleFavorite,
  }: {
    image: BingImage;
    regionIndex: number;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  } = $props();

  let url = $derived(image.urls[regionIndex] ?? image.urls[0]);
  let title = $derived(image.titles[regionIndex] ?? image.titles[0]);
  let copyright = $derived(image.copyrights[regionIndex] ?? image.copyrights[0]);
  let copyrightlink = $derived(
    image.copyrightlinks[regionIndex] ?? image.copyrightlinks[0]
  );
  let filename = $derived(url.split("/").pop() ?? "");
</script>

<div class="border border-gray-200 rounded overflow-hidden">
  <div
    class="relative overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
  >
    <img
      class="w-full transition-all group-hover:scale-105"
      src={url}
      alt={title}
    />
  </div>
  <div class="p-2 grid gap-1">
    <div class="flex justify-between items-center">
      <div class="text-xs font-medium truncate mr-1">{title}</div>
      <WallpaperActions
        service="Bing"
        {url}
        href={copyrightlink}
        {filename}
        {favorite}
        {onToggleFavorite}
      />
    </div>
  </div>
</div>
```

- [ ] **Step 2: 提交**

```bash
git add src/lib/components/CollectionCard.svelte
git commit -m "feat: 新增 CollectionCard 紧凑卡片组件"
```

---

## Task 4: 前端 — 新增 `TodayCollection.svelte` 组件（含展开/收起）

**Files:**
- 新增: `src/lib/components/TodayCollection.svelte`

**背景:** 这是方案 E 的核心交互组件。默认展示 4 张卡片，点击"查看全部"原地展开全部，按钮变为"收起"。使用 `grid-cols-2` 的两列布局，和现有 `gap-3` 间距一致。

- [ ] **Step 1: 创建 `TodayCollection.svelte`**

```svelte
<script lang="ts">
  import CollectionCard from "./CollectionCard.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let {
    items,
    favoritesSet,
  }: {
    items: BingImage[];
    favoritesSet: Set<string>;
  } = $props();

  let expanded = $state(false);

  // 默认展示数量
  const PREVIEW_COUNT = 4;

  // 展示的列表
  let visibleItems = $derived(
    expanded ? items : items.slice(0, PREVIEW_COUNT)
  );

  let totalCount = $derived(items.length);
  let hasMore = $derived(totalCount > PREVIEW_COUNT);
  let headerText = $derived(
    expanded ? `今日合集 · 共 ${totalCount} 张` : `今日合集 · ${Math.min(PREVIEW_COUNT, totalCount)} / ${totalCount}`
  );

  async function toggleFavorite(image: BingImage, regionIndex: number) {
    const url = image.urls[regionIndex] ?? image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favoritesSet.has(filename)) {
      await invoke("remove_favorite", { filename });
      favoritesSet.delete(filename);
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: image.titles[regionIndex] ?? image.titles[0],
        startdate: image.startdates[regionIndex] ?? image.startdates[0],
        copyright: image.copyrights[regionIndex] ?? image.copyrights[0],
        copyrightlink: image.copyrightlinks[regionIndex] ?? image.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
      favoritesSet.add(filename);
    }
    // 触发响应式更新
    favoritesSet = new Set(favoritesSet);
  }
</script>

{#if items.length > 0}
  <div class="grid gap-2">
    <div class="flex justify-between items-center">
      <span class="text-sm font-medium">{headerText}</span>
      {#if hasMore}
        <button
          type="button"
          class="text-xs text-neutral-500 hover:text-neutral-700 cursor-pointer"
          onclick={() => (expanded = !expanded)}
        >
          {expanded ? "收起" : "查看全部"}
        </button>
      {/if}
    </div>
    <div class="grid grid-cols-2 gap-3">
      {#each visibleItems as img, i}
        <CollectionCard
          {image}
          regionIndex={0}
          favorite={favoritesSet.has(img.urls[0].split("/").pop() ?? "")}
          onToggleFavorite={() => toggleFavorite(img, 0)}
        />
      {/each}
    </div>
  </div>
{/if}
```

- [ ] **Step 2: 提交**

```bash
git add src/lib/components/TodayCollection.svelte
git commit -m "feat: 新增 TodayCollection 组件，含展开/收起交互"
```

---

## Task 5: 前端 — 新增 `RecentPreview.svelte` 和 `FavoritesPreview.svelte`

**Files:**
- 新增: `src/lib/components/RecentPreview.svelte`
- 新增: `src/lib/components/FavoritesPreview.svelte`

**背景:** 这两个是首页下方的紧凑预览。每个 section 展示 2-3 条，带"查看全部"入口。但注意：因为取消了 Tab 导航，"查看全部"不再是跳 Tab，而是展开更多项（和 TodayCollection 一样的原地展开逻辑）。

- [ ] **Step 1: 创建 `RecentPreview.svelte`**

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WallpaperActions from "./WallpaperActions.svelte";
  import Skeleton from "./Skeleton.svelte";
  import { Select, Label } from "flowbite-svelte";

  let {
    favoritesSet,
  }: {
    favoritesSet: Set<string>;
  } = $props();

  let range = $state(7);
  let images = $state<BingImage[]>([]);
  let loading = $state(true);
  let expanded = $state(false);
  const PREVIEW_COUNT = 3;

  async function loadRecent() {
    loading = true;
    try {
      images = (await invoke("get_recent_wallpapers", { days: range })) as BingImage[];
    } finally {
      loading = false;
    }
  }

  async function toggleFavorite(image: BingImage) {
    const url = image.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (favoritesSet.has(filename)) {
      await invoke("remove_favorite", { filename });
      favoritesSet.delete(filename);
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: image.titles[0],
        startdate: image.startdates[0],
        copyright: image.copyrights[0],
        copyrightlink: image.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
      favoritesSet.add(filename);
    }
    favoritesSet = new Set(favoritesSet);
  }

  $effect(() => {
    range;
    loadRecent();
  });

  let visibleImages = $derived(
    expanded ? images : images.slice(0, PREVIEW_COUNT)
  );
  let hasMore = $derived(images.length > PREVIEW_COUNT);
</script>

<div class="grid gap-3">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2">
      <span class="text-sm font-medium">Recent</span>
      <Select
        size="sm"
        class="w-auto text-xs"
        bind:value={range}
      >
        <option value={7}>7天</option>
        <option value={14}>14天</option>
      </Select>
    </div>
    {#if hasMore}
      <button
        type="button"
        class="text-xs text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={() => (expanded = !expanded)}
      >
        {expanded ? "收起" : "全部"}
      </button>
    {/if}
  </div>

  {#if loading}
    <div class="grid gap-3">
      {#each Array(2) as _}
        <Skeleton />
      {/each}
    </div>
  {:else if images.length === 0}
    <div class="text-xs text-neutral-500">暂无近期壁纸</div>
  {:else}
    <div class="grid gap-3">
      {#each visibleImages as img}
        <div class="grid gap-1">
          <div
            class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
          >
            <img
              class="w-full rounded transition-all group-hover:scale-105"
              src={img.urls[0]}
              alt={img.titles[0]}
            />
          </div>
          <div class="grid gap-1">
            <div class="flex justify-between items-center">
              <div class="text-xs font-medium truncate mr-2">{img.titles[0]}</div>
              <WallpaperActions
                service="Bing"
                url={img.urls[0]}
                href={img.copyrightlinks[0]}
                filename={img.urls[0].split("/").pop() ?? ""}
                favorite={favoritesSet.has(img.urls[0].split("/").pop() ?? "")}
                onToggleFavorite={() => toggleFavorite(img)}
              />
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
```

- [ ] **Step 2: 创建 `FavoritesPreview.svelte`**

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WallpaperActions from "./WallpaperActions.svelte";

  let {
    onFavoritesChanged,
  }: {
    onFavoritesChanged?: () => void;
  } = $props();

  let favorites = $state<FavoriteItem[]>([]);
  let loading = $state(true);
  let expanded = $state(false);
  const PREVIEW_COUNT = 3;

  async function loadFavorites() {
    loading = true;
    try {
      favorites = (await invoke("list_favorites")) as FavoriteItem[];
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(filename: string) {
    await invoke("remove_favorite", { filename });
    await loadFavorites();
    onFavoritesChanged?.();
  }

  loadFavorites();

  let visibleFavorites = $derived(
    expanded ? favorites : favorites.slice(0, PREVIEW_COUNT)
  );
  let hasMore = $derived(favorites.length > PREVIEW_COUNT);
</script>

<div class="grid gap-3">
  <div class="flex justify-between items-center">
    <span class="text-sm font-medium">Favorites</span>
    {#if hasMore}
      <button
        type="button"
        class="text-xs text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={() => (expanded = !expanded)}
      >
        {expanded ? "收起" : "全部"}
      </button>
    {/if}
  </div>

  {#if loading}
    <div class="text-xs text-neutral-500">加载中…</div>
  {:else if favorites.length === 0}
    <div class="text-xs text-neutral-500">
      还没有收藏。在 Today 或 Recent 里收藏喜欢的壁纸。
    </div>
  {:else}
    <div class="grid gap-3">
      {#each visibleFavorites as item (item.filename)}
        <div class="grid gap-1">
          <div
            class="relative rounded overflow-hidden group w-full aspect-[1.78] bg-medirian-1"
          >
            <img
              class="w-full rounded transition-all group-hover:scale-105"
              src={item.url}
              alt={item.title}
            />
          </div>
          <div class="grid gap-1">
            <div class="flex justify-between items-center">
              <div class="text-xs font-medium truncate mr-2">{item.title}</div>
              <WallpaperActions
                service="Bing"
                url={item.url}
                href={item.copyrightlink}
                {filename: item.filename}
                favorite={true}
                onToggleFavorite={async () => removeFavorite(item.filename)}
              />
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
```

- [ ] **Step 3: 提交**

```bash
git add src/lib/components/RecentPreview.svelte src/lib/components/FavoritesPreview.svelte
git commit -m "feat: 新增 RecentPreview 和 FavoritesPreview 紧凑预览组件"
```

---

## Task 6: 前端 — 新增 `SettingsPanel.svelte` 轻量设置面板

**Files:**
- 新增: `src/lib/components/SettingsPanel.svelte`

**背景:** 从现有 `settings.svelte` 提取设置内容，放在一个可关闭的 overlay 面板中。右上角齿轮图标触发打开，面板从右侧滑入或淡入覆盖。

- [ ] **Step 1: 创建 `SettingsPanel.svelte`**

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button, Checkbox, Label, Select, A } from "flowbite-svelte";
  import { getName, getVersion } from "@tauri-apps/api/app";
  import CheckCircleSolid from "flowbite-svelte-icons/CheckCircleSolid.svelte";
  import { checkUpdate } from "../../lib/updater";

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

  function getUserConfig() {
    invoke("get_config").then((res) => {
      config = res as UserConfig;
    });
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

{#if open}
  <!-- 遮罩 -->
  <div
    class="fixed inset-0 bg-black/20 z-50"
    onclick={onClose}
    role="presentation"
  ></div>

  <!-- 面板 -->
  <div
    class="fixed top-0 right-0 bottom-0 w-full bg-white z-50 overflow-y-auto scrollbar-stable"
  >
    <div class="p-4 grid gap-4">
      <!-- 顶部栏 -->
      <div class="flex justify-between items-center">
        <span class="text-sm font-medium">设置</span>
        <button
          type="button"
          class="text-neutral-500 hover:text-neutral-700 cursor-pointer text-lg"
          onclick={onClose}
        >
          ✕
        </button>
      </div>

      <!-- 基本设置 -->
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

      <div class="border-t border-gray-200"></div>

      <!-- 高级设置 -->
      <div class="grid gap-3">
        <div class="text-xs font-medium text-neutral-400 uppercase tracking-wide">
          高级设置
        </div>

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

      <div class="border-t border-gray-200"></div>

      <!-- 关于 -->
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
  </div>
{/if}
```

- [ ] **Step 2: 提交**

```bash
git add src/lib/components/SettingsPanel.svelte
git commit -m "feat: 新增 SettingsPanel 轻量设置面板"
```

---

## Task 7: 前端 — 重写 `index.svelte` 为单页面

**Files:**
- 修改: `src/pages/main/index.svelte`

**背景:** 核心重构。从 4-Tab 导航改为单页面滚动。顶部栏只保留品牌名 + 刷新按钮 + 设置齿轮。页面内容区可滚动，依次排列 Hero → TodayCollection → RecentPreview → FavoritesPreview。

- [ ] **Step 1: 重写 `index.svelte`**

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import TodayHero from "../../lib/components/TodayHero.svelte";
  import TodayCollection from "../../lib/components/TodayCollection.svelte";
  import RecentPreview from "../../lib/components/RecentPreview.svelte";
  import FavoritesPreview from "../../lib/components/FavoritesPreview.svelte";
  import SettingsPanel from "../../lib/components/SettingsPanel.svelte";
  import Skeleton from "../../lib/components/Skeleton.svelte";

  // 今日 Hero
  let todayImage = $state<BingImage | null>(null);
  let todayLoading = $state(true);
  let todayFavorite = $state(false);

  // 今日合集
  let todayCollection = $state<BingImage[]>([]);

  // 共享收藏集合
  let favoritesSet = $state<Set<string>>(new Set());

  // Settings 面板
  let settingsOpen = $state(false);

  async function loadFavorites() {
    const favs = (await invoke("list_favorites")) as FavoriteItem[];
    favoritesSet = new Set(favs.map((f) => f.filename));
  }

  async function loadTodayHero() {
    todayLoading = true;
    try {
      todayImage = (await invoke("get_today_wallpaper")) as BingImage | null;
      await loadFavorites();
      if (todayImage) {
        const filename = todayImage.urls[0].split("/").pop() ?? "";
        todayFavorite = favoritesSet.has(filename);
      }
    } finally {
      todayLoading = false;
    }
  }

  async function loadTodayCollection() {
    todayCollection = (await invoke("get_today_collection")) as BingImage[];
  }

  async function toggleTodayFavorite() {
    if (!todayImage) return;
    const url = todayImage.urls[0];
    const filename = url.split("/").pop() ?? "";
    if (todayFavorite) {
      await invoke("remove_favorite", { filename });
    } else {
      const item: FavoriteItem = {
        filename,
        url,
        title: todayImage.titles[0],
        startdate: todayImage.startdates[0],
        copyright: todayImage.copyrights[0],
        copyrightlink: todayImage.copyrightlinks[0],
      };
      await invoke("add_favorite", { item });
    }
    todayFavorite = !todayFavorite;
    await loadFavorites();
  }

  // 刷新全部
  async function refreshAll() {
    await Promise.all([loadTodayHero(), loadTodayCollection()]);
  }

  // 事件监听
  $effect(() => {
    const unlisteners: (() => void)[] = [];

    listen("go-to-settings", () => {
      settingsOpen = true;
    }).then((unlisten) => unlisteners.push(unlisten));

    listen("check-for-updates", async () => {
      const { checkUpdate } = await import("../../lib/updater");
      checkUpdate();
    }).then((unlisten) => unlisteners.push(unlisten));

    return () => {
      for (const unlisten of unlisteners) {
        unlisten();
      }
    };
  });

  // 初始加载
  refreshAll();
</script>

<div class="w-full h-full flex flex-col">
  <!-- 顶部栏 -->
  <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200">
    <div class="flex items-center gap-2">
      <div class="w-2 h-2 rounded-full bg-gradient-to-br from-bayOfMany-900 to-apple-500"></div>
      <span class="text-sm font-semibold">Pavo</span>
    </div>
    <div class="flex gap-2">
      <button
        type="button"
        class="text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={refreshAll}
        title="刷新"
      >
        ↻
      </button>
      <button
        type="button"
        class="text-neutral-500 hover:text-neutral-700 cursor-pointer"
        onclick={() => (settingsOpen = true)}
        title="设置"
      >
        ⚙
      </button>
    </div>
  </div>

  <!-- 内容区 -->
  <div class="flex-1 h-0 overflow-y-auto overflow-x-hidden scrollbar-stable p-4 pr-2">
    <div class="grid gap-4">
      <!-- Hero -->
      {#if todayLoading}
        <Skeleton />
      {:else if todayImage}
        <TodayHero
          image={todayImage}
          favorite={todayFavorite}
          onToggleFavorite={toggleTodayFavorite}
        />
      {:else}
        <div class="text-sm text-neutral-500">今日壁纸暂不可用</div>
      {/if}

      <!-- 今日各地区合集 -->
      <TodayCollection items={todayCollection} {favoritesSet} />

      <!-- 分隔线 -->
      <div class="border-t border-gray-200"></div>

      <!-- Recent 预览 -->
      <RecentPreview {favoritesSet} />

      <!-- 分隔线 -->
      <div class="border-t border-gray-200"></div>

      <!-- Favorites 预览 -->
      <FavoritesPreview />
    </div>
  </div>
</div>

<SettingsPanel open={settingsOpen} onClose={() => (settingsOpen = false)} />
```

- [ ] **Step 2: 检查 `src-tauri/tauri.conf.json` 中是否注册了 `get_today_collection` 命令**

如果 Tauri 使用 `allowlist` 或权限系统，确认新命令已添加。

- [ ] **Step 3: 运行前端开发服务器确认页面可加载**

Run: `npm run dev`（或 `pnpm dev`）
Expected: 页面加载，无编译错误，首页显示 Hero + 今日合集

- [ ] **Step 4: 提交**

```bash
git add src/pages/main/index.svelte
git commit -m "feat: 重写首页为单页面滚动布局"
```

---

## Task 8: 清理旧文件

**Files:**
- 删除: `src/pages/main/today.svelte`
- 删除: `src/pages/main/recent.svelte`
- 删除: `src/pages/main/favorites.svelte`
- 删除: `src/pages/main/settings.svelte`
- 删除: `src/pages/main/about.svelte`
- 删除: `src/pages/main/bing.svelte`
- 删除: `src/lib/components/BingWallpaper.svelte`
- 删除: `src/lib/components/Toolbar.svelte`

**背景:** 这些文件已被新组件完全替代。删除前需确认无其他文件引用它们。

- [ ] **Step 1: 搜索所有可能引用旧文件的地方**

用 grep 搜索以下导入：
- `from "./today` / `from "./recent` / `from "./favorites` / `from "./settings` / `from "./about` / `from "./bing`
- `BingWallpaper` / `Toolbar`

确认除了 `index.svelte`（已重写）之外没有其他引用。

- [ ] **Step 2: 删除旧文件**

```bash
rm src/pages/main/today.svelte
rm src/pages/main/recent.svelte
rm src/pages/main/favorites.svelte
rm src/pages/main/settings.svelte
rm src/pages/main/about.svelte
rm src/pages/main/bing.svelte
rm src/lib/components/BingWallpaper.svelte
rm src/lib/components/Toolbar.svelte
```

- [ ] **Step 3: 运行开发服务器确认无编译错误**

Run: `npm run dev`
Expected: 无 import 报错

- [ ] **Step 4: 检查 `WallpaperHero.svelte` 是否还被引用**

搜索 `from` 包含 `WallpaperHero` 的文件。如果只有旧文件引用它，也可以删除。如果新组件仍在引用（`TodayHero` 是从中复制的，不引用它），则删除。

- [ ] **Step 5: 提交**

```bash
git add -A
git commit -m "chore: 清理被替代的旧页面和组件文件"
```

---

## Task 9: 端到端验证

**Files:** 无新增

- [ ] **Step 1: 启动完整应用**

Run: `cargo tauri dev`（从项目根目录）
Expected: 窗口打开，340x800，显示新首页

- [ ] **Step 2: 验证首页核心功能**

逐项检查：
1. Hero 大图加载并显示标题、日期、版权信息
2. Hero 操作按钮（设为壁纸、下载、收藏、打开链接）都能工作
3. 今日合集 section 显示，默认 4 张
4. 如果今日超过 4 张，"查看全部"按钮可见
5. 点击"查看全部"原地展开，按钮变为"收起"
6. 点击"收起"回缩到 4 张
7. Recent section 显示，默认 3 条，可展开
8. Favorites section 显示，默认 3 条，可展开
9. 右上角齿轮打开设置面板
10. 设置面板各项可操作，关闭回到首页
11. 刷新按钮重新加载所有数据

- [ ] **Step 3: 修复发现的问题**

如果验证中发现任何问题，创建修复提交。

- [ ] **Step 4: 最终提交**

```bash
git add -A
git commit -m "fix: 首页重构验证后的修复"
```

---

## 自查清单

- **Spec 覆盖:** 方案 E 的所有要素（Hero、今日合集 4+展开、Recent 预览、Favorites 预览、Settings 轻量入口）都有对应 Task
- **占位符:** 无 TBD/TODO
- **类型一致性:** `BingImage`/`FavoriteItem`/`UserConfig` 都使用现有 `typing.d.ts` 中的定义，`SchedulerPhoto` 对应后端 `scheduler::SchedulerPhoto`
- **风格约束:** 全部使用 `rounded`（不用 `rounded-xl`）、`gap-2/3/4`、朴素 Tailwind，无多余装饰
