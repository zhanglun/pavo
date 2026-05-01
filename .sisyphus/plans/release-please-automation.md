# Pavo 自动化发布流程 — Release Please 集成

## 目标

用 Release Please 替代手动版本号管理 + `release` 分支推送，实现：
- 基于 conventional commits 自动判断版本号（feat→minor, fix→patch, feat!→major）
- 自动同步 `package.json`、`src-tauri/tauri.conf.json`、`src-tauri/Cargo.toml` 三个文件的版本号
- 自动创建 GitHub Release，CI 构建产物直接上传

## 当前状态

- 版本号需在 3 个文件手动同步：`package.json`(v0.0.12)、`src-tauri/tauri.conf.json`(v0.0.12)、`src-tauri/Cargo.toml`(v0.0.12)
- CI（`.github/workflows/release.yml`）通过 push 到 `release` 分支触发，使用 `tauri-apps/tauri-action@v0`
- 已有 `PAVO_TOKEN` secret（PAT），已配置 Tauri auto-updater
- Commits 已使用 conventional 格式

## 目标流程

```
开发者 push feat/fix commit 到 main
  → Release Please 自动创建/更新 Release PR
    （Release PR 包含：版本号变更 + changelog）
  → 开发者 review & merge Release PR
  → Release Please 自动创建 GitHub Release + git tag
  → CI workflow 触发，构建 macOS/Windows/Linux
  → 构建产物上传到 Release Please 创建的 Release
```

**`release` 分支不再需要。**

## 需要的改动

### 1. 新建 `release-please-config.json`

```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "packages": {
    ".": {
      "release-type": "node",
      "bump-minor-pre-major": true,
      "bump-patch-for-minor-pre-major": true,
      "extra-files": [
        {
          "type": "json",
          "path": "src-tauri/tauri.conf.json",
          "jsonpath": "$.version"
        },
        {
          "type": "toml",
          "path": "src-tauri/Cargo.toml",
          "jsonpath": "$.package.version"
        }
      ]
    }
  }
}
```

说明：
- `release-type: "node"` — 自动处理 `package.json` 的版本号
- `extra-files` — 额外同步 `tauri.conf.json`（JSON）和 `Cargo.toml`（TOML）的版本号
- `bump-minor-pre-major` / `bump-patch-for-minor-pre-major` — 在 0.x 版本阶段，feat 仍然 bump minor（不会直接跳到 1.0）
- `Cargo.lock` 不需要手动处理——CI 构建 `cargo build` 时会自动重新生成

### 2. 新建 `.release-please-manifest.json`

```json
{
  ".": "0.0.12"
}
```

这是 Release Please 记录已发布版本的清单文件，初始值设为当前版本。

### 3. 重写 `.github/workflows/release.yml`

当前文件：85 行，单一 job `publish-tauri`，监听 `push to release`。

重写为两个 job：

```yaml
name: "publish"

on:
  push:
    branches:
      - main

env:
  TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
  TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}

permissions:
  contents: write
  pull-requests: write

jobs:
  release-please:
    runs-on: ubuntu-latest
    outputs:
      release_created: ${{ steps.release.outputs.release_created }}
      release_id: ${{ steps.release.outputs.id }}
      tag_name: ${{ steps.release.outputs.tag_name }}
    steps:
      - uses: googleapis/release-please-action@v4
        id: release
        with:
          token: ${{ secrets.PAVO_TOKEN }}

  publish-tauri:
    needs: release-please
    if: ${{ needs.release-please.outputs.release_created }}
    strategy:
      fail-fast: false
      matrix:
        settings:
          - platform: "macos-latest"
            args: "--target aarch64-apple-darwin"
          - platform: "macos-latest"
            args: "--target x86_64-apple-darwin"
          - platform: "ubuntu-22.04"
            args: ""
          - platform: "windows-latest"
            args: ""

    runs-on: ${{ matrix.settings.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: setup node
        uses: actions/setup-node@v4
        with:
          node-version: lts/*

      - name: install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.settings.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

      - name: install dependencies (ubuntu only)
        if: matrix.settings.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

      - uses: pnpm/action-setup@v3
        name: Install pnpm
        with:
          version: 8
          run_install: false

      - name: Get pnpm store directory
        shell: bash
        run: |
          echo "STORE_PATH=$(pnpm store path --silent)" >> $GITHUB_ENV

      - uses: actions/cache@v4
        name: Setup pnpm cache
        with:
          path: ${{ env.STORE_PATH }}
          key: ${{ runner.os }}-pnpm-store-${{ hashFiles('**/pnpm-lock.yaml') }}
          restore-keys: |
            ${{ runner.os }}-pnpm-store-

      - name: Install dependencies
        run: pnpm install

      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.PAVO_TOKEN }}
        with:
          releaseId: ${{ needs.release-please.outputs.release_id }}
          args: ${{ matrix.settings.platform == 'ubuntu-22.04' && '--bundles deb,appimage' || matrix.settings.args }}
```

关键改动：
1. **触发条件**：`push to main` 替代 `push to release`
2. **两个 job**：`release-please`（管理 Release PR + GitHub Release）→ `publish-tauri`（构建上传）
3. **`publish-tauri` 条件**：`if: needs.release-please.outputs.release_created` — 仅在 Release Please 创建了新 Release 时触发
4. **`tauri-action` 使用 `releaseId`**：直接上传到 Release Please 创建的 Release，不再用 `tagName` 自行创建
5. **Token 使用 `PAVO_TOKEN`（PAT）**：`GITHUB_TOKEN` 无法在 merge PR 后触发新 workflow
6. **Ubuntu 限制 bundle 类型**：避免在 Linux 上构建不需要的 bundle 格式（可选，按原样保留也行）

### 4. 可选：删除 `release` 分支

确认新流程正常后，可删除 `release` 分支。

## 版本号规则

| Commit 类型 | 版本变更 | 示例 |
|-------------|---------|------|
| `feat:` | minor | 0.0.12 → 0.1.0 |
| `fix:` | patch | 0.0.12 → 0.0.13 |
| `feat!:` 或 `BREAKING CHANGE` | major | 0.0.12 → 1.0.0 |
| `chore:`, `docs:`, `style:`, `refactor:` | 不触发发布 | — |

## Token 说明

- 必须使用 **PAT**（Personal Access Token），不能用默认的 `GITHUB_TOKEN`
- 原因：merge Release PR 后需要触发 `publish-tauri` workflow，`GITHUB_TOKEN` 无法触发 chain event
- 已有 `PAVO_TOKEN` secret，直接复用
- PAT 需要有 `contents: write` 和 `pull-requests: write` 权限

## 参考项目

- [YanceyOfficial/talon](https://github.com/YanceyOfficial/talon) — Tauri v2 + Release Please 完整集成，结构几乎相同

## 验证清单

发布流程上线后，手动验证：
1. [ ] Push 一个 `feat:` commit 到 main → Release Please 创建 Release PR
2. [ ] Release PR 包含三个文件的版本号变更
3. [ ] Merge Release PR → 自动创建 GitHub Release + tag
4. [ ] CI 构建触发，四个平台的产物上传到 Release
5. [ ] Auto-updater endpoint (`latest.json`) 正确指向新 Release

## 文件变更总结

| 操作 | 文件 | 说明 |
|------|------|------|
| 新建 | `release-please-config.json` | Release Please 配置 |
| 新建 | `.release-please-manifest.json` | 版本清单（初始 0.0.12） |
| 重写 | `.github/workflows/release.yml` | 两阶段 CI |
| 可选 | 删除 `release` 分支 | 确认新流程后 |
