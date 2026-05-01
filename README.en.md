[中文](./README.md)

<p align="center">
  <img src="./pavo.png" width="190px"/>
</p>
<h1 align="center">Pavo</h1>

Pavo is a cross-platform desktop wallpaper app built with Tauri v2. It features multi-region Bing daily wallpapers, auto-rotating wallpapers, a desktop info overlay, and favorites management.

## Features

- **Multi-region Daily Wallpapers** — Browse Bing daily wallpapers from different regions worldwide
- **Auto Rotation** — Automatically change wallpapers at intervals from 10 minutes to 24 hours, in sequential or random mode
- **Favorites** — Save your favorite wallpapers with persistent storage across restarts
- **Frameless Window** — Minimal UI that auto-positions near the system tray
- **Light/Dark Theme** — Follow the system preference or switch manually
- **Auto Update** — Automatically checks for updates on launch
- **Local Cache** — Wallpaper images are cached locally for offline viewing

## Screenshots

<p align="center">
<img src="./screenshot-1.png" width="45%" />
<img src="./screenshot-2.png" width="45%" />
</p>

## Tech Stack

- **[Tauri v2](https://v2.tauri.app/)** — Rust + WebView cross-platform desktop framework
- **[Svelte 5](https://svelte.dev/)** — Frontend framework (Runes reactivity)
- **[Rust](https://www.rust-lang.org/)** — Backend logic
- **[Tailwind CSS](https://tailwindcss.com/)** — Styling
- **TypeScript** — Type safety

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (recommended to install via [nvm](https://github.com/nvm-sh/nvm))
- [pnpm](https://pnpm.io/installation)
- [Rust](https://www.rust-lang.org/tools/install)
- Follow the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for platform-specific dependencies

### Development

```bash
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

For more details, see the [Tauri distribution guide](https://v2.tauri.app/distribute/).

## Release Process

This project uses [Release Please](https://github.com/googleapis/release-please) for automated versioning and releases:

1. Merge conventional commits (`feat`/`fix`) into `main`
2. Release Please creates a Release PR and bumps the version
3. Merging the Release PR triggers a GitHub Release
4. CI builds and uploads multi-platform installers automatically

## Repo Activity

![Alt](https://repobeats.axiom.co/api/embed/ac6f91c5371bcea9fdb92dcc7da9479f50423d81.svg "Repobeats analytics image")

### More Projects

- [Lettura](https://github.com/zhanglun/lettura) — A free and open-source RSS reader for macOS and Windows
