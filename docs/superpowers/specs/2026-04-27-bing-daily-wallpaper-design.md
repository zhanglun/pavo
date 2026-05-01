# Bing Daily Wallpaper Product Design

## Positioning

Pavo V1 is positioned as `一个极简的 Bing Daily Wallpaper 桌面工具`.

The product promise is intentionally narrow:

- Show today's Bing wallpaper clearly and immediately.
- Let the user set it as wallpaper with one click.
- Let the user browse recent 7 or 14 days of Bing history.
- Update daily without requiring manual intervention.
- Cache images locally for reliable reuse.
- Let the user favorite wallpapers they want to keep.

This positioning explicitly avoids turning Pavo into a general wallpaper platform.

## Product Scope

### In Scope for V1

- `Today` primary landing experience for today's Bing wallpaper.
- `Recent` browsing for recent 7-day or 14-day Bing history.
- `Favorites` for user-curated saved wallpapers.
- `Settings` focused on the daily wallpaper workflow.
- One-click wallpaper application.
- Daily automatic refresh behavior for the latest Bing wallpaper.
- Local cache reuse.
- Local metadata persistence for favorites and recent items that must survive app restart.
- Tray integration as a secondary access path.
- Keep the current `Download` action, but frame it as a local-save or cache-adjacent action rather than a core product promise.

### Out of Scope for V1

- Independent wallpaper assignment per display.
- Multiple wallpaper sources such as Unsplash.
- AI-generated wallpapers.
- Mood-driven or weather-driven wallpaper logic.
- Search-oriented discovery or source aggregation.
- Complex filtering systems.

### V2 Candidates

- Per-display independent wallpaper assignment.
- Bing market or region selection exposed in product UI.
- Richer cache controls.
- Better sorting and management inside Favorites.
- Launch-at-login as a user-facing preference if it is not included in the first implementation pass.

## Information Architecture

Pavo should move from the current tool-oriented structure to a product-oriented structure.

### V1 Top-Level Navigation

- `Today`
- `Recent`
- `Favorites`
- `Settings`

### Navigation Rationale

- `Today` is the primary destination and should be the default entry.
- `Recent` supports lightweight browsing and replay of recent Bing images.
- `Favorites` gives the user a durable personal collection.
- `Settings` should contain configuration, not compete with content surfaces.
- `About` should not remain a first-level destination. It should move into Settings or a secondary menu.

## Screen Design

### Today

`Today` is the main promise of the product and should optimize for instant comprehension and instant action.

#### Goals

- Make it obvious what today's wallpaper is.
- Make `Set as Wallpaper` the dominant action.
- Keep the interface calm and minimal.

#### Content

- Large hero wallpaper preview.
- Primary button: `设为壁纸`.
- Secondary actions: `收藏`, `保存到本地`, `在 Bing 打开`.
- Metadata: title, date, copyright.

#### Rules

- Do not overload the screen with advanced controls.
- Do not make Today look like a generic gallery.
- Today is not just the first item in a list; it is a dedicated recommendation surface.

### Recent

`Recent` is the browsing surface for short-term history.

#### Goals

- Let the user review the latest Bing wallpapers quickly.
- Keep the browsing range intentional and bounded.

#### Content

- Scope switcher: `最近 7 天` / `最近 14 天`.
- Grid or card list of recent wallpapers.
- Per-card actions: `设为壁纸`, `收藏`, `保存到本地`, `在 Bing 打开`.

#### Rules

- Recent should explicitly communicate time range, not just show an undifferentiated list.
- Recent should support discovery, not archival management.

### Favorites

`Favorites` is the lightweight persistence surface for user intent.

#### Goals

- Let users keep wallpapers they care about.
- Make revisiting favorite wallpapers faster than searching history again.

#### Content

- Saved wallpaper cards only.
- Per-card actions: `设为壁纸`, `取消收藏`, `保存到本地`, `在 Bing 打开`.
- Empty state that points users back to Today or Recent.

#### Rules

- Favorites should be complete enough to feel real in V1.
- It should not be hidden behind filters inside Recent.
- It does not need advanced organization in V1.

### Settings

`Settings` should support the daily wallpaper workflow and stop reading like a technical control panel.

#### First-Layer Settings

- `每日自动更新`
- `历史范围` when relevant to visible history behavior
- `缓存位置` plus cache behavior explanation

#### Advanced Settings

- `显示桌面信息层`
- `检查更新`
- `日志文件`

#### Rules

- Avoid exposing minute-level interval language as a primary concept.
- Avoid technical jargon that weakens the product promise.
- Keep advanced or diagnostic controls secondary.
- Explain `每日自动更新` in product language as updating to the latest Bing wallpaper while the app is running, rather than as a generic shuffle interval.

## Feature Naming Changes

The product language should shift from implementation terminology to user-facing value.

- `Bing` top-level page should be split into `Today` and `Recent`.
- `Shuffle` should be reframed as `每日自动更新`.
- `Show desktop layer` should become `显示桌面信息层`.
- `Download` should remain available, but the user-facing explanation should emphasize local saving rather than internal implementation language.
- `About` should move out of primary navigation.

## Interaction Principles

- Opening the app should immediately answer: what is today's wallpaper?
- First launch should trigger an immediate fetch path for today's wallpaper instead of relying on a later background refresh.
- The primary action should always be setting the wallpaper.
- Browsing should feel short-range and intentional.
- Favoriting should be available wherever wallpaper cards are presented.
- Advanced controls should not interrupt the default daily flow.
- If the app is offline, Recent and Favorites should render from whatever local metadata and cached files are already available rather than blocking the entire surface.

## Technical Mapping to Current Codebase

This design intentionally reuses most of the current Bing-centered implementation.

### Existing Capabilities to Reuse

- Bing image fetching and recent-list basis in `src-tauri/src/scheduler.rs`.
- Wallpaper download and local file management in `src-tauri/src/services/bing.rs`.
- Wallpaper application behavior in `src-tauri/src/services/bing.rs`.
- Toolbar-level wallpaper actions in `src/lib/components/Toolbar.svelte`.
- Update and tray infrastructure already present in the repo.

### Current Codebase Constraints

- The current main navigation is `Bing / Settings / About`, so `Today / Recent / Favorites / Settings` is a structural change rather than a copy-only rename.
- The current `Bing` page is a flat list of cards, not a split `Today` hero plus `Recent` history surface.
- The current settings model is still centered on `auto_shuffle`, `interval`, and `show_layer`; `每日自动更新` will require a semantic change, not just relabeling.
- The current scheduler fetch path does not yet provide a guaranteed 14-day slice, so V1 needs an explicit data-range expansion rather than only a UI switch.
- The current frontend and tray copy is English-only. V1 may rewrite copy directly without introducing a full i18n system.
- The main window is currently narrow and fixed-size, so the `Today` hero and `Recent` card layout should be designed with the existing Tauri window constraints in mind unless window sizing changes are intentionally included.
- The current app stores wallpaper files directly under `~/.pavo/`, so favorites and recent history cannot rely on memory-only metadata if they need to survive restart.

### Product-Layer Refactors Needed

- Split the current `Bing` surface into distinct `Today` and `Recent` experiences.
- Rework `Settings` copy and information architecture to match the new positioning.
- Move `About` out of first-level navigation.
- Reduce emphasis on internal or diagnostic controls.
- Update tray menu wording and event routing so it no longer depends on `About` as a first-level destination.

### New Functional Additions Needed

- Favorites persistence with full metadata stored on disk rather than only relying on in-memory scheduler state.
- Favorites commands and UI actions.
- Explicit 7-day and 14-day recent grouping in the UI and backend filtering logic.
- Dedicated Today presentation rather than only a list-based presentation.
- A data model that can identify the current day's wallpaper independently from the rest of the recent list.

### Backend Semantics To Preserve or Replace

- The current `auto_shuffle` behavior rotates cached wallpapers on an interval. V1 should replace its user-facing meaning with `每日自动更新`, which means updating to the latest Bing wallpaper rather than cycling through history.
- The current scheduler fetches recent Bing entries in batches and merges by filename across multiple markets. V1 should treat this as an implementation detail and explicitly filter the final dataset into 7-day and 14-day views.
- The current local cache lives under `~/.pavo/`. V1 should preserve local caching but may reorganize cached files and metadata if that makes favorites and history more reliable.
- The initial V1 daily-update behavior should be defined as "check at startup and while the app is running" rather than as a promise of exact system-scheduled midnight updates.

## Suggested V1 Delivery Shape

### Must-Have

- Today hero experience.
- Recent 7/14 day browsing.
- Favorite / unfavorite actions.
- Favorite metadata persistence across app restarts.
- Wallpaper application from Today, Recent, and Favorites.
- Daily automatic refresh for the latest Bing wallpaper.
- Local cache retained and reused.
- Download/save action retained across wallpaper cards.

### Should Keep but De-Emphasize

- Tray controls.
- Desktop information layer.
- Update checks.
- Launch-at-login, if it is shipped in the first pass, should remain a secondary setting rather than a core navigation concept.

### Should Not Be Marketed as Core

- Low-level timing or scheduling mechanics.
- Diagnostic or log-related actions.
- General wallpaper management language.

## Success Criteria

The redesign is successful when:

- A new user can understand the app within a few seconds.
- The default path is clearly `Today -> 设为壁纸`.
- Recent history is understandable as a 7-day or 14-day slice.
- Favorites feels like a real saved collection, not an unfinished stub.
- Settings feels product-oriented instead of implementation-oriented.
- The app reads as a Bing daily wallpaper tool, not a general wallpaper experiment.
- The app can restart without losing favorites metadata or breaking the Favorites surface.
- The app can start offline and still show Favorites plus any cached Recent items whose metadata was already persisted.

## Non-Goals and Guardrails

- Do not add new wallpaper sources as part of this redesign.
- Do not expand V1 into a multi-monitor orchestration feature.
- Do not introduce AI, weather, or mood-based branching logic.
- Do not redesign around search-centric discovery.
- Do not refactor backend fetching logic unless it directly supports the new product surfaces.

## Open Implementation Questions

These are implementation questions, not product-scope blockers:

- Whether favorites should live inside `pavo.toml` or in a separate metadata file under `~/.pavo/`.
- Whether Recent should default to 7 days or remember the last chosen range.
- Whether the scheduler should keep multi-market aggregation in V1 or reduce to a single default market first.
- Whether V1 should include launch-at-login immediately or defer it behind the core wallpaper flow.
- Whether V1 should add automatic cleanup rules for non-favorited cached files or keep cache management to explanatory copy only.
- Whether cached image management needs a visible manual control in V1 or just explanatory copy.
- Whether UHD should remain the default cached resolution for all surfaces or whether browsing should use a lighter-weight asset strategy.
- Which Bing market should be treated as the V1 default when no user-facing market picker exists.
- Whether the main window width should stay fixed or be widened slightly to better support the `Today` hero and `Recent` layout.
- Whether update checks should stay automatic, move fully to user-triggered flow, or support both.

## Next Step

After this spec is approved, the next step is to write an implementation plan that decomposes the work into focused tasks across navigation, Today/Recent/Favorites UI, config persistence, and product copy updates.
