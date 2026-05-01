# Bing Daily Wallpaper Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reposition Pavo as a minimalist Bing Daily Wallpaper desktop app with `Today / Recent / Favorites / Settings`, persistent favorites, explicit 7/14-day history, and product-focused daily update behavior.

**Architecture:** Keep the existing Tauri + Svelte structure, but split the current Bing-centric surface into dedicated Today/Recent/Favorites pages and replace the exposed `shuffle` concept with a daily-update-oriented configuration model. Reuse the existing Bing fetch and wallpaper application pipeline, while extending the Rust config and command layer to persist favorites metadata and provide product-specific queries for today, recent history, and favorites.

**Tech Stack:** Svelte 5, Flowbite-Svelte, TypeScript, Tauri v2, Rust, TOML config persistence, Bing `HPImageArchive` API.

---

## File Structure

### Create

- `src/pages/main/today.svelte` - dedicated Today hero page for the current wallpaper.
- `src/pages/main/recent.svelte` - explicit recent-history page with 7/14-day filtering.
- `src/pages/main/favorites.svelte` - persistent favorites surface with empty state.
- `src/lib/components/WallpaperActions.svelte` - shared action row that supports set wallpaper, save locally, favorite/unfavorite, and open in Bing.
- `src/lib/components/WallpaperHero.svelte` - Today-specific hero presentation.
- `src-tauri/src/favorites.rs` - favorite item data model and favorite-specific helpers.
- `docs/superpowers/plans/2026-04-27-bing-daily-wallpaper-implementation.md` - this plan.

### Modify

- `src/pages/main/index.svelte` - replace `Bing / Settings / About` with `Today / Recent / Favorites / Settings` navigation.
- `src/pages/main/settings.svelte` - rewrite product-facing settings and move About/update/log into secondary placement.
- `src/pages/main/about.svelte` - slim secondary product/app info content rendered from Settings.
- `src/pages/main/bing.svelte` - remove after Today/Recent migration is complete.
- `src/lib/components/BingWallpaper.svelte` - reduce responsibility or replace with a recent-card-specific presentation.
- `src/lib/components/Toolbar.svelte` - replace or refactor into the richer actions component.
- `src/lib/updater.ts` - align update wording/flow with the new Settings placement.
- `src/typing.d.ts` - update frontend types for config, favorites, today/recent payloads.
- `src-tauri/src/config.rs` - add product-oriented config schema and favorites persistence.
- `src-tauri/src/cmd.rs` - add commands for favorites/today/recent and remove old product-facing assumptions.
- `src-tauri/src/scheduler.rs` - support 14-day history and query helpers for today/recent filtering.
- `src-tauri/src/services/bing.rs` - keep Bing download/apply flow but support reliable metadata/file reuse.
- `src-tauri/src/services/mod.rs` - fix download overwrite behavior.
- `src-tauri/src/background.rs` - replace exposed shuffle behavior with daily-update semantics.
- `src-tauri/src/shuffle_thread.rs` - repurpose or replace with daily-update worker logic.
- `src-tauri/src/tray.rs` - adjust tray wording and About routing assumptions.
- `src-tauri/src/main.rs` - wire new commands and any startup/update semantics.
- `README.md` - update product positioning after implementation.

### Test / Verify

- `src-tauri/src/services/mod.rs` existing Rust test block - extend if backend helper logic gains deterministic units.
- `pnpm exec svelte-check`
- `cargo check`
- `cargo clippy`
- `pnpm tauri build`

---

### Task 1: Redefine Product Config And Favorites Persistence

**Files:**
- Modify: `src-tauri/src/config.rs`
- Modify: `src/typing.d.ts`
- Create: `src-tauri/src/favorites.rs`

- [ ] **Step 1: Write the failing Rust config tests**

Add Rust unit tests at the bottom of `src-tauri/src/config.rs` that encode the new persistence contract.

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_legacy_config_without_new_fields() {
    let input = r#"
auto_shuffle = true
interval = 30
show_layer = false
"#;

    let parsed: PavoConfig = toml::from_str(input).unwrap_or_else(|_| PavoConfig::new());
    assert_eq!(parsed.show_layer, false);
    assert_eq!(parsed.history_range_days, 7);
    assert!(parsed.favorites.is_empty());
  }

  #[test]
  fn serialize_favorites_with_metadata() {
    let cfg = PavoConfig {
      auto_daily_update: true,
      history_range_days: 14,
      show_layer: false,
      favorites: vec![FavoriteItem {
        filename: "OHR.Sample.jpg".into(),
        url: "https://www.bing.com/th?id=OHR.Sample.jpg".into(),
        title: "Sample".into(),
        startdate: "20260427".into(),
        copyright: "Copyright".into(),
        copyrightlink: "https://www.bing.com".into(),
        local_path: None,
      }],
    };

    let text = toml::to_string(&cfg).unwrap();
    assert!(text.contains("history_range_days = 14"));
    assert!(text.contains("filename = \"OHR.Sample.jpg\""));
  }
}
```

- [ ] **Step 2: Run Rust tests to verify they fail first**

Run:

```bash
cargo test config::tests --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL because `FavoriteItem`, `auto_daily_update`, and `history_range_days` do not exist yet.

- [ ] **Step 3: Implement the new config schema**

Replace the old shuffle-oriented schema with a product-oriented one.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteItem {
  pub filename: String,
  pub url: String,
  pub title: String,
  pub startdate: String,
  pub copyright: String,
  pub copyrightlink: String,
  pub local_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PavoConfig {
  #[serde(default)]
  pub auto_daily_update: bool,
  #[serde(default = "default_history_range_days")]
  pub history_range_days: u8,
  #[serde(default)]
  pub show_layer: bool,
  #[serde(default)]
  pub favorites: Vec<FavoriteItem>,
}

fn default_history_range_days() -> u8 {
  7
}

impl PavoConfig {
  pub fn new() -> Self {
    Self {
      auto_daily_update: true,
      history_range_days: 7,
      show_layer: false,
      favorites: vec![],
    }
  }
}
```

Also add helpers such as `set_auto_daily_update`, `set_history_range_days`, `add_favorite`, and `remove_favorite_by_filename` in `config.rs`.

Update `src/typing.d.ts` to mirror the same shape:

```ts
interface FavoriteItem {
  filename: string;
  url: string;
  title: string;
  startdate: string;
  copyright: string;
  copyrightlink: string;
  local_path?: string | null;
}

interface UserConfig {
  auto_daily_update: boolean;
  history_range_days: number;
  show_layer: boolean;
  favorites: FavoriteItem[];
}
```

- [ ] **Step 4: Run Rust config tests again**

Run:

```bash
cargo test config::tests --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

- [ ] **Step 5: Run type and compile verification for the changed schema**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS, or only unrelated pre-existing issues.

---

### Task 2: Add Backend Queries For Today, Recent, And Favorites

**Files:**
- Modify: `src-tauri/src/scheduler.rs`
- Modify: `src-tauri/src/cmd.rs`
- Modify: `src-tauri/src/main.rs`
- Modify: `src/typing.d.ts`

- [ ] **Step 1: Write the failing scheduler tests for 7/14-day filtering**

Add focused Rust tests that operate on synthetic `SchedulerPhoto` values.

```rust
#[cfg(test)]
mod scheduler_tests {
  use super::*;

  fn photo(filename: &str, startdate: &str) -> SchedulerPhoto {
    SchedulerPhoto {
      filename: filename.into(),
      regions: vec!["zh-CN".into()],
      urls: vec![format!("https://example.com/{filename}")],
      titles: vec![filename.into()],
      startdates: vec![startdate.into()],
      copyrights: vec!["Copyright".into()],
      copyrightlinks: vec!["https://www.bing.com".into()],
    }
  }

  #[test]
  fn filter_recent_days_limits_result_count() {
    let list = vec![
      photo("a", "20260427"), photo("b", "20260426"), photo("c", "20260420"), photo("d", "20260410"),
    ];

    let recent = Scheduler::filter_recent_days(&list, 7, "20260427");
    assert_eq!(recent.len(), 3);
  }
}
```

- [ ] **Step 2: Run the scheduler tests to verify they fail**

Run:

```bash
cargo test scheduler_tests --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL because `filter_recent_days` does not exist yet.

- [ ] **Step 3: Implement query helpers and commands**

Extend `scheduler.rs` with helpers like these:

```rust
impl Scheduler {
  pub fn filter_recent_days(list: &[SchedulerPhoto], days: u8, today: &str) -> Vec<SchedulerPhoto> {
    list.iter()
      .filter(|photo| within_recent_days(photo.startdates.first().map(String::as_str).unwrap_or_default(), today, days))
      .cloned()
      .collect()
  }

  pub fn pick_today(list: &[SchedulerPhoto], today: &str) -> Option<SchedulerPhoto> {
    list.iter()
      .find(|photo| photo.startdates.first().map(String::as_str) == Some(today))
      .cloned()
      .or_else(|| list.first().cloned())
  }
}
```

Then add commands in `cmd.rs`:

```rust
#[tauri::command]
pub async fn get_today_wallpaper() -> Option<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.ok()?;
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::pick_today(&list, &today)
}

#[tauri::command]
pub async fn get_recent_wallpapers(days: u8) -> Vec<scheduler::SchedulerPhoto> {
  let mut scheduler = scheduler::SCHEDULER.lock().await;
  let list = scheduler.batch_fetch().await.unwrap_or_default();
  let today = chrono::Local::now().format("%Y%m%d").to_string();
  scheduler::Scheduler::filter_recent_days(&list, days, &today)
}

#[tauri::command]
pub async fn list_favorites() -> Vec<config::FavoriteItem> {
  config::PavoConfig::get_config().favorites
}

#[tauri::command]
pub async fn add_favorite(item: config::FavoriteItem) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().add_favorite(item)).unwrap()
}

#[tauri::command]
pub async fn remove_favorite(filename: String) -> serde_json::Value {
  serde_json::to_value(config::PavoConfig::get_config().remove_favorite_by_filename(&filename)).unwrap()
}
```

Expose the commands in `main.rs` via `tauri::generate_handler!`.

- [ ] **Step 4: Expand Bing fetching to support a real 14-day slice**

Update `fetch_list_with_region()` so it fetches a third range.

```rust
let res1 = bing::Wallpaper::new(0, 8, Some(region.clone())).await?;
let res2 = bing::Wallpaper::new(7, 8, Some(region.clone())).await?;
let res3 = bing::Wallpaper::new(14, 8, Some(region.clone())).await?;

let images1 = res1.json.images;
let images2 = res2.json.images;
let images3 = res3.json.images;

let mut res: Vec<SchedulerPhoto> = images1
  .into_iter()
  .chain(images2.into_iter())
  .chain(images3.into_iter())
  .map(|i| -> Result<SchedulerPhoto, Box<dyn std::error::Error + Send + Sync>> {
    let filename = bing::Images::get_filename(&i.url)?;
    Ok(SchedulerPhoto {
      filename,
      urls: vec![["https://www.bing.com", &i.url].concat()],
      regions: vec![region.clone()],
      titles: vec![i.title],
      startdates: vec![i.startdate],
      copyrights: vec![i.copyright],
      copyrightlinks: vec![i.copyrightlink],
    })
  })
  .collect::<Result<_, _>>()?;
```

- [ ] **Step 5: Run backend verification again**

Run:

```bash
cargo test scheduler_tests --manifest-path src-tauri/Cargo.toml && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

---

### Task 3: Replace Exposed Shuffle Semantics With Daily Update Semantics

**Files:**
- Modify: `src-tauri/src/background.rs`
- Modify: `src-tauri/src/shuffle_thread.rs`
- Modify: `src-tauri/src/cmd.rs`
- Modify: `src/pages/main/settings.svelte`

- [ ] **Step 1: Write the failing behavior test for the new config naming**

Add a lightweight config test proving `auto_daily_update` is the persisted flag rather than `auto_shuffle`.

```rust
#[test]
fn write_config_persists_auto_daily_update_flag() {
  let cfg = PavoConfig {
    auto_daily_update: true,
    history_range_days: 7,
    show_layer: false,
    favorites: vec![],
  };
  let text = toml::to_string(&cfg).unwrap();
  assert!(text.contains("auto_daily_update = true"));
}
```

- [ ] **Step 2: Run the test to verify the old naming is gone**

Run:

```bash
cargo test config::tests --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL until all write paths use the new field.

- [ ] **Step 3: Implement daily-update-oriented command names and worker behavior**

Rename the exposed command and message flow away from shuffle language.

```rust
pub enum AsyncProcessMessage {
  StartDailyUpdate,
  StopDailyUpdate,
  PreviousPhoto,
  NextPhoto,
}

#[tauri::command]
pub async fn set_auto_daily_update(
  enabled: bool,
  state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), ()> {
  let pavo_config = config::PavoConfig::get_config();
  pavo_config.set_auto_daily_update(enabled);
  let sender = state.sender.lock().await;
  sender
    .send(if enabled {
      AsyncProcessMessage::StartDailyUpdate
    } else {
      AsyncProcessMessage::StopDailyUpdate
    })
    .await
    .map_err(|_| ())
}
```

In `background.rs` / `shuffle_thread.rs`, preserve the worker structure but make the user-facing semantics center on updating to the latest Bing wallpaper, not random cycling. If the existing thread is still reused internally in the first pass, rename it during the same refactor so the code no longer advertises shuffle behavior.

- [ ] **Step 4: Update the Settings screen contract**

Change the frontend invocation path from `set_auto_shuffle` to `set_auto_daily_update` and remove minute-based interval controls from the first visible layer.

```ts
async function updateAutoDailyUpdate(enabled: boolean) {
  await invoke("set_auto_daily_update", { enabled });
}
```

- [ ] **Step 5: Run compile verification for the renamed command surface**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

---

### Task 4: Fix Download Reliability And Shared Frontend Types

**Files:**
- Modify: `src-tauri/src/services/mod.rs`
- Modify: `src/lib/components/Toolbar.svelte`
- Modify: `src/typing.d.ts`

- [ ] **Step 1: Write the failing Rust test for overwrite-safe downloads**

Extend `src-tauri/src/services/mod.rs` tests with a helper-oriented test if you extract a file-open helper.

```rust
#[test]
fn favorite_metadata_type_keeps_local_path_optional() {
  let item = crate::config::FavoriteItem {
    filename: "demo".into(),
    url: "https://example.com/demo".into(),
    title: "Demo".into(),
    startdate: "20260427".into(),
    copyright: "Copyright".into(),
    copyrightlink: "https://www.bing.com".into(),
    local_path: None,
  };
  assert!(item.local_path.is_none());
}
```

- [ ] **Step 2: Run the relevant backend tests**

Run:

```bash
cargo test services --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL or compile failure until the helper changes settle.

- [ ] **Step 3: Make download overwrite instead of append**

Replace append mode in `download_file()`.

```rust
let mut file = OpenOptions::new()
  .create(true)
  .write(true)
  .truncate(true)
  .open(path)
  .await
  .map_err(|e| format!("Failed to open file '{}': {}", path.display(), e))?;
```

At the same time, make sure the shared action component keeps these props:

```ts
type WallpaperActionProps = {
  service: string;
  url: string;
  href: string;
  filename: string;
  favorite: boolean;
};
```

- [ ] **Step 4: Re-run backend and frontend verification**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

---

### Task 5: Replace The Main Navigation With Today / Recent / Favorites / Settings

**Files:**
- Modify: `src/pages/main/index.svelte`
- Create: `src/pages/main/today.svelte`
- Create: `src/pages/main/recent.svelte`
- Create: `src/pages/main/favorites.svelte`
- Modify: `src/pages/main/about.svelte`

- [ ] **Step 1: Write the failing UI shell by importing pages that do not exist yet**

Update `index.svelte` imports first.

```ts
import Today from "./today.svelte";
import Recent from "./recent.svelte";
import Favorites from "./favorites.svelte";
import Settings from "./settings.svelte";

let current = $state("Today");
```

- [ ] **Step 2: Run Svelte type-check to verify the new pages are missing**

Run:

```bash
pnpm exec svelte-check
```

Expected: FAIL because the new page files do not exist yet.

- [ ] **Step 3: Create the new page shells and wire the tabs**

Use this `index.svelte` shape:

```svelte
<Tabs tabStyle="underline" contentClass="flex-1 h-0">
  <TabItem open={current === "Today"} title="Today" onclick={() => (current = "Today")}>
    <Today />
  </TabItem>
  <TabItem open={current === "Recent"} title="Recent" onclick={() => (current = "Recent")}>
    <Recent />
  </TabItem>
  <TabItem open={current === "Favorites"} title="Favorites" onclick={() => (current = "Favorites")}>
    <Favorites />
  </TabItem>
  <TabItem open={current === "Settings"} title="Settings" onclick={() => (current = "Settings")}>
    <Settings />
  </TabItem>
</Tabs>
```

Create minimal page shells:

```svelte
<!-- today.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let image = $state<BingImage | null>(null);
</script>

<div class="grid gap-4"></div>
```

```svelte
<!-- recent.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let range = $state(7);
  let images = $state<BingImage[]>([]);
</script>

<div class="grid gap-4"></div>
```

```svelte
<!-- favorites.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let favorites = $state<FavoriteItem[]>([]);
</script>

<div class="grid gap-4"></div>
```

- [ ] **Step 4: Update tray routing assumptions**

Replace the About event path with settings-focused routing or another secondary destination.

```ts
listen("go-to-settings", () => {
  current = "Settings";
});
```

- [ ] **Step 5: Run Svelte verification**

Run:

```bash
pnpm exec svelte-check
```

Expected: PASS or only unrelated pre-existing warnings.

---

### Task 6: Implement Today And Recent UI Using Shared Wallpaper Components

**Files:**
- Create: `src/lib/components/WallpaperActions.svelte`
- Create: `src/lib/components/WallpaperHero.svelte`
- Modify: `src/lib/components/BingWallpaper.svelte`
- Modify: `src/pages/main/today.svelte`
- Modify: `src/pages/main/recent.svelte`

- [ ] **Step 1: Write the failing integration by using the new shared action component before it exists**

```svelte
<!-- inside today.svelte -->
<WallpaperHero {image} favorite={favorite} onToggleFavorite={toggleFavorite} />
```

Expected follow-up failure: `WallpaperHero` is not defined yet.

- [ ] **Step 2: Run Svelte type-check to capture the missing component errors**

Run:

```bash
pnpm exec svelte-check
```

Expected: FAIL until the new shared components are created.

- [ ] **Step 3: Implement the shared action row and hero layout**

Create the action surface with text-forward controls.

```svelte
<!-- WallpaperActions.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let { service, url, href, filename, favorite, onToggleFavorite } = $props<{
    service: string;
    url: string;
    href: string;
    filename: string;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  }>();

  async function setAsDesktop() {
    await invoke("set_as_desktop", { service, url });
  }

  async function saveLocally() {
    await invoke("download", { service, url });
  }
</script>
```

And a hero wrapper:

```svelte
<!-- WallpaperHero.svelte -->
<script lang="ts">
  import WallpaperActions from "./WallpaperActions.svelte";
  let { image, favorite, onToggleFavorite } = $props<{
    image: BingImage;
    favorite: boolean;
    onToggleFavorite: () => Promise<void>;
  }>();
</script>
```

Use Today logic like this:

```ts
async function loadToday() {
  image = (await invoke("get_today_wallpaper")) as BingImage | null;
}
```

Use Recent logic like this:

```ts
async function loadRecent() {
  images = (await invoke("get_recent_wallpapers", { days: range })) as BingImage[];
}
```

- [ ] **Step 4: Verify Svelte compilation again**

Run:

```bash
pnpm exec svelte-check
```

Expected: PASS.

---

### Task 7: Implement Favorites Page And Favorite Toggle Flow

**Files:**
- Modify: `src/pages/main/favorites.svelte`
- Modify: `src/pages/main/today.svelte`
- Modify: `src/pages/main/recent.svelte`
- Modify: `src-tauri/src/cmd.rs`

- [ ] **Step 1: Write the failing UI flow by calling favorite commands that do not fully work yet**

```ts
async function toggleFavorite(item: FavoriteItem) {
  if (favoriteSet.has(item.filename)) {
    await invoke("remove_favorite", { filename: item.filename });
  } else {
    await invoke("add_favorite", { item });
  }
}
```

- [ ] **Step 2: Run Svelte and Rust compile checks to expose contract mismatches**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL until payload shapes line up.

- [ ] **Step 3: Implement the full favorites flow**

Use a client-side favorite-set derived from `list_favorites()`:

```ts
let favorites = $state<FavoriteItem[]>([]);
let favoriteSet = $derived(new Set(favorites.map((item) => item.filename)));

async function loadFavorites() {
  favorites = (await invoke("list_favorites")) as FavoriteItem[];
}
```

Render the empty state explicitly:

```svelte
{#if favorites.length === 0}
  <div class="rounded-lg border border-dashed p-4 text-sm text-neutral-500">
    还没有收藏。你可以在 Today 或 Recent 里收藏喜欢的 Bing 壁纸。
  </div>
{:else}
  <!-- render cards -->
{/if}
```

- [ ] **Step 4: Re-run type-check and backend compile verification**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

---

### Task 8: Rework Settings, About, Tray, And Product Copy

**Files:**
- Modify: `src/pages/main/settings.svelte`
- Modify: `src/pages/main/about.svelte`
- Modify: `src-tauri/src/tray.rs`
- Modify: `README.md`
- Modify: `src/lib/updater.ts`

- [ ] **Step 1: Write the failing UX state by removing obsolete labels first**

In `settings.svelte`, replace obsolete visible labels before the full implementation is ready.

```svelte
<Checkbox
  bind:checked={config.auto_daily_update}
  on:change={(e) => updateAutoDailyUpdate((e.target as HTMLInputElement).checked)}
>
  每日自动更新
</Checkbox>
<Checkbox
  bind:checked={config.show_layer}
  on:change={(e) => updateShowLayer((e.target as HTMLInputElement).checked)}
>
  显示桌面信息层
</Checkbox>
```

- [ ] **Step 2: Run Svelte type-check to expose any remaining old config references**

Run:

```bash
pnpm exec svelte-check
```

Expected: FAIL until all `auto_shuffle` / `interval` assumptions are removed or relocated.

- [ ] **Step 3: Implement the final product-oriented settings and secondary About content**

Shape Settings around the new product model.

```svelte
<div class="grid gap-4">
  <Checkbox bind:checked={config.auto_daily_update} on:change={(e) => updateAutoDailyUpdate((e.target as HTMLInputElement).checked)}>
    每日自动更新
  </Checkbox>
  <Select bind:value={config.history_range_days} on:change={(e) => updateHistoryRange(Number((e.target as HTMLSelectElement).value))}>
    <option value={7}>最近 7 天</option>
    <option value={14}>最近 14 天</option>
  </Select>
  <div class="text-xs text-neutral-500">缓存位置：~/.pavo/</div>
</div>
```

Update tray wording in `src-tauri/src/tray.rs` so About is no longer a top-level navigation dependency. Keep `Settings`, `Check for Updates`, `Show`, `Hide`, and `Quit`, and rename product-confusing items such as `Previous photo` / `Next photo` to wording that matches the final retained behavior.

Update `README.md` opening copy to match the new product positioning.

- [ ] **Step 4: Run cross-stack verification**

Run:

```bash
pnpm exec svelte-check && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

---

### Task 9: End-To-End Verification And Cleanup

**Files:**
- Verify modified files from all previous tasks

- [ ] **Step 1: Run frontend type-check**

Run:

```bash
pnpm exec svelte-check
```

Expected: PASS.

- [ ] **Step 2: Run Rust verification**

Run:

```bash
cargo fmt --check --manifest-path src-tauri/Cargo.toml && cargo clippy --manifest-path src-tauri/Cargo.toml && cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS.

- [ ] **Step 3: Run full desktop build verification**

Run:

```bash
pnpm tauri build
```

Expected: PASS with both frontend and Tauri packaging succeeding.

- [ ] **Step 4: Review the finished product copy and navigation contract**

Check that these files match the spec:

```text
src/pages/main/index.svelte
src/pages/main/today.svelte
src/pages/main/recent.svelte
src/pages/main/favorites.svelte
src/pages/main/settings.svelte
README.md
```

Confirm that:

```text
- Bing / Settings / About no longer defines the primary IA
- Favorites persists across restart
- Today is the default entry
- Recent supports 7/14-day slices
- Daily update language has replaced shuffle language in the product surface
```

- [ ] **Step 5: Commit**

```bash
git add README.md src src-tauri docs/superpowers/specs/2026-04-27-bing-daily-wallpaper-design.md docs/superpowers/plans/2026-04-27-bing-daily-wallpaper-implementation.md
git commit -m "feat: refocus pavo on bing daily wallpaper"
```
