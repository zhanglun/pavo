[English](./README.en.md)

<p align="center">
  <img src="./pavo.png" width="190px"/>
</p>
<h1 align="center">Pavo</h1>

Pavo 是一个跨平台桌面壁纸应用，基于 Tauri v2 构建。支持多地区 Bing 每日壁纸轮播、自动切换壁纸，以及收藏管理。

## 功能

- **多地区今日壁纸** — 浏览全球不同地区的 Bing 每日壁纸
- **自动轮播** — 按设定间隔（10 分钟 ~ 24 小时）自动切换壁纸，支持顺序和随机模式
- **壁纸收藏** — 收藏喜欢的壁纸，数据跨重启持久保存
- **无边框窗口** — 极简界面，自动定位到系统托盘附近
- **浅色/深色主题** — 跟随系统或手动切换
- **自动更新** — 应用启动时自动检查并提示更新
- **本地缓存** — 壁纸图片自动保存到本地，支持离线查看

## 截图

<p align="center">
<img src="./screenshot-1.png" width="45%" />
<img src="./screenshot-2.png" width="45%" />
</p>

## 技术栈

- **[Tauri v2](https://v2.tauri.app/)** — Rust + WebView 跨平台桌面框架
- **[Svelte 5](https://svelte.dev/)** — 前端框架（Runes 响应式）
- **[Rust](https://www.rust-lang.org/)** — 后端逻辑
- **[Tailwind CSS](https://tailwindcss.com/)** — 样式
- **TypeScript** — 类型安全

## 快速开始

### 前置条件

- [Node.js](https://nodejs.org/)（推荐通过 [nvm](https://github.com/nvm-sh/nvm) 安装）
- [pnpm](https://pnpm.io/installation)
- [Rust](https://www.rust-lang.org/tools/install)
- 按照 [Tauri v2 环境配置](https://v2.tauri.app/start/prerequisites/) 安装平台依赖

### 开发

```bash
pnpm install
pnpm tauri dev
```

### 构建

```bash
pnpm tauri build
```

更多信息请参考 [Tauri 发布指南](https://v2.tauri.app/distribute/)

## 发布流程

运行 `pnpm release`，选择版本号，脚本会自动更新版本、创建 tag、推送到 `release` 分支，CI 自动构建并发布 GitHub Release。

## Repo Activity

![Alt](https://repobeats.axiom.co/api/embed/ac6f91c5371bcea9fdb92dcc7da9479f50423d81.svg "Repobeats analytics image")

### More Projects

- [Lettura](https://github.com/zhanglun/lettura) — 免费开源的 macOS/Windows RSS 阅读器
