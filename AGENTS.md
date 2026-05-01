# Pavo Agent Notes

## Project Shape
- Pavo is a Tauri v2 desktop wallpaper app with a Svelte 5 frontend in `src/` and a Rust backend in `src-tauri/src/`.
- The app has two windows and two Vite entrypoints: `index.html` -> `src/pages/main/index.ts` for the main UI, and `underlayer.html` -> `src/pages/layer/index.ts` for the desktop underlay overlay.
- `src-tauri/src/main.rs` wires the app together; Tauri commands live in `src-tauri/src/cmd.rs`; persisted app settings are managed by `src-tauri/src/config.rs`.

## Dev Commands
- Use `pnpm` only. The repo is wired around `pnpm-lock.yaml`, `tauri.conf.json`, and CI.
- Install dependencies with `pnpm install`.
- Run the full app with `pnpm tauri dev`. This is the real development entrypoint; it runs `pnpm dev` first, then starts Tauri.
- Build the full desktop app with `pnpm tauri build`. This runs `pnpm build` first, then bundles the Rust app.
- `pnpm dev` is frontend-only Vite dev server work; do not treat it as a full app check.
- `pnpm build` is frontend-only and writes to `build/`, not Vite's default `dist/`.

## Verification
- There is no configured frontend test runner and no `test` script in `package.json`; do not claim tests were run unless you added them.
- There is no configured ESLint or Prettier setup; avoid inventing lint steps.
- Frontend type checking is available via `pnpm exec svelte-check` even though it is not exposed as a script.
- Rust checks run from `src-tauri/`: `cargo fmt --check`, `cargo clippy`, and `cargo check`.
- For end-to-end verification of app-integrated changes, prefer `pnpm tauri build` over `pnpm build`.

## Toolchain Quirks
- This is Tauri v2, not v1. Permissions are defined in `src-tauri/capabilities/*.json`; when adding new Tauri capabilities or plugins, update capabilities instead of looking for a v1 allowlist.
- `src-tauri/tauri.conf.json` sets `beforeDevCommand` and `beforeBuildCommand`; keep frontend and Tauri flows consistent with those hooks.
- `vite.config.js` fixes the dev server to port `1420`, uses HMR port `1421` when `TAURI_DEV_HOST` is set, and builds both HTML entrypoints.
- The frontend uses Svelte 5 runes (`$state`, `$effect`) rather than legacy Svelte reactivity patterns.
- Tailwind is configured through `tailwind.config.js` and includes Flowbite/Flowbite-Svelte content paths; preserve those when moving UI files.

## Runtime Data
- User config is stored under `~/.pavo/pavo.toml`.
- The app also creates `~/.pavo/logs/Pavo.log`; `reveal_log_file` in `src-tauri/src/cmd.rs` opens that path in the system file manager.

## Release Flow
- CI release automation lives in `.github/workflows/release.yml`.
- Publishing is triggered by pushes to the `release` branch, not `main`.
- CI uses pnpm 8 and builds macOS (arm64 + x64), Ubuntu 22.04, and Windows artifacts through `tauri-apps/tauri-action`.
