# Pavo React「每日册页」重构交接文档

更新时间：2026-07-23  
当前状态：React 重构主体完成，等待最终功能等价验收与旧 Svelte 清理

## 1. 工作位置

- 仓库：`/Users/zhanglun/Documents/mine/pavo`
- 开发 worktree：`/Users/zhanglun/Documents/mine/pavo/.worktrees/react-daily-folio`
- 分支：`feature/react-daily-folio`
- 最新提交：`da9c790 fix: 补齐关键操作反馈与启动稳定性`
- 当前 worktree：除本交接文档外，无其他未提交修改

后续开发请在 worktree 中进行：

```bash
cd /Users/zhanglun/Documents/mine/pavo/.worktrees/react-daily-folio
```

## 2. 重构目标与已确认原则

- 技术栈迁移至 React 19、TypeScript、Vite、Tauri v2。
- 以“功能等价为底线，但界面完全按每日册页重做”为原则。
- 首发兼顾 macOS 与 Windows。
- UI 气质：优雅、高端、克制、低噪音。
- 核心动作始终是“设为桌面”。
- 主题持久化为“跟随系统 / 浅色 / 暗色”三档。
- 不引入 Radix Primitives、重量级组件库或全局状态库。
- 采用 FSD-Lite，而非完整 FSD。
- 用户已明确不恢复旧的 TodayHero 自动预览轮播。

完整设计规格见：

`docs/superpowers/specs/2026-07-22-react-daily-folio-redesign-design.md`

## 3. 已完成范围

### React 基础架构

- Vite 入口已切换至 React。
- 页面按 `app / pages / entities / features / shared` 分层。
- 使用 CSS Modules 和全局设计 Token。
- Tauri command、事件及平台插件分别通过统一封装调用。

### 今日

- 已实现每日册页主视觉、日期、主图、标题及介绍。
- 支持多地区壁纸选择。
- 当今日接口只有少量数据时，可读取近期内容回退，并明确标注“近期内容”。
- 地区壁纸按地区与稳定标识去重，不再重复展示同一张主图。
- 支持设为桌面、收藏、下载原图、查看介绍与来源。
- 次级操作收纳在重新设计的 `⋯` 菜单中。

### 历史

- 已改为按日期归档的单一列表。
- 数据按稳定标识去重，避免重复渲染。
- 每项支持收藏及 `⋯` 菜单操作。
- 历史范围读取设置中的 7/14 天选项。

### 收藏

- 已改为双列私人册页。
- 使用心形图标取消收藏，不显示“取消收藏”文字。
- 取消收藏失败时保留原卡片并显示错误反馈。
- 收藏变更可在今日、历史和收藏之间同步。

### 设置

- 已覆盖三档主题、开机启动、每日自动更新、手动检查更新、壁纸轮播、历史范围、缓存保留、日志、版本、GitHub 与问题反馈入口。
- 补充应用身份和缓存位置。
- 手动更新支持检查结果、安装确认、下载安装及重启。
- 更新流程有全局并发互斥，防止按钮与托盘同时触发多个更新流程。

### 稳定性与交互

- Toast 使用浮层，不进入页面文档流，不再引起额外滚动条。
- 壁纸设置期间显示持续进度提示，完成或失败后替换结果。
- Toast 切换会清理旧计时器。
- Tooltip 已实现。
- 滚动条已调整为细样式。
- 固定主题会在 React 挂载前同步应用，减少启动闪烁。
- 设置保存失败、收藏失败和系统操作失败均有反馈。

## 4. 最近阶段提交

```text
da9c790 fix: 补齐关键操作反馈与启动稳定性
8894a07 fix(ui): 修正提示浮层与历史重复内容
ed51c25 fix(today): 修正近期地区壁纸重复展示
3a31213 fix(ui): 去重壁纸数据并补齐交互提示
cd7a245 fix: 校正每日壁纸排序与收藏标识
eb24424 fix(ui): 对齐每日册页视觉并移除透明窗口
```

## 5. 当前验证证据

最近一次完整验证结果：

- `pnpm test`：14 个测试文件、47 项测试全部通过。
- `pnpm build`：TypeScript 与 Vite 生产构建通过。
- `cargo check`：通过。
- `cargo test -- --skip services::tests::it_works`：30 项通过，1 项过滤。
- `pnpm tauri dev`：macOS 冷启动成功。
- macOS 视觉冒烟：窗口背景、尺寸比例、今日主图及各地区不同缩略图显示正常。

建议复验命令：

```bash
pnpm test
pnpm build
cd src-tauri && cargo test -- --skip services::tests::it_works && cargo check
cd .. && pnpm tauri dev
```

## 6. 已知问题与未完成项

### Rust 联网测试不稳定

完整 `cargo test` 中，`services::tests::it_works` 会请求 Bing 实时接口。法语地区接口曾返回不含预期 `OHR` 片段的数据，导致测试失败：

```text
url missing OHR segment:
https://www.bing.com/HPImageArchive.aspx?...&mkt=fr-FR
```

这不是编译错误，也不阻止应用启动，但应将该测试改为固定 fixture/mock，避免实时网络决定测试结果。

### Windows 尚未实机冒烟

代码和 Tauri 配置已兼顾 Windows，但尚未完成 Windows 实机验证。至少需要验证：

- 窗口尺寸、拖拽、纵向拉伸和隐藏至托盘。
- 托盘重新打开窗口及进入设置。
- 设置壁纸、下载路径、日志位置。
- 三档主题及重启持久化。
- 开机启动真实状态同步。
- 自动更新权限及安装流程。

### 功能等价清单尚待最终人工确认

自动化测试和 macOS 启动已通过，但以下系统级行为不宜完全依赖单元测试，需要逐项人工验收：

- 今日多地区切换及近期内容回退。
- 设为桌面与下载至 `~/Downloads/Pavo`。
- 收藏跨页面即时同步和重启持久化。
- 历史菜单所有操作。
- 窗口隐藏、托盘恢复及托盘菜单。
- 三档主题切换、系统主题联动及重启持久化。
- 开机启动、自动更新、手动更新与打开日志。
- 自动壁纸轮播的启停、间隔及顺序模式。
- 图片加载失败占位与网络失败反馈。

## 7. Svelte 清理状态

旧 Svelte 代码和依赖尚未删除，这是刻意保留的回退与逻辑参考。当前仍能看到：

- `src/pages/main/*.svelte`
- `src/lib/components/*.svelte`
- `src/lib/updater.ts`
- `src/lib/utils/wallpaper.ts`
- `package.json` 中的 Svelte、Tailwind、Flowbite 及相关开发依赖

不要在功能等价人工验收完成前删除。验收通过后，应在单独清理提交中：

1. 对照旧 Svelte 逻辑完成最后一次功能清单检查。
2. 删除旧 Svelte 页面、组件和无引用工具文件。
3. 移除 Svelte Vite 插件及 Svelte、Tailwind、Flowbite 等无用依赖。
4. 清理旧配置、类型声明和构建脚本。
5. 更新 `README.md`、`README.en.md` 和 `docs/architecture.md`，将技术栈与架构说明改为 React。
6. 重新执行前端测试、生产构建、Rust 检查以及 macOS/Windows 冒烟。

## 8. 推荐后续顺序

1. 执行完整功能等价人工验收，并记录通过/失败项。
2. 修正 Bing 实时联网测试，使 Rust 全量测试稳定。
3. 完成 Windows 实机冒烟，修复平台差异。
4. 经用户确认后删除 Svelte 和旧依赖。
5. 更新项目文档与截图。
6. 执行最终全量测试和双平台验收。
7. 决定合并、PR 或发布方式。

## 9. 交接注意事项

- 当前用户要求每次修改代码前都先确认；即使前一次修改已获确认，下一次修改仍需重新确认。
- 阶段任务完成后应自审、验证并提交。
- Git 提交信息使用中文，且不包含 AI 相关措辞。
- 不要为节省时间跳过验证，但避免无意义地重复 Build。
- 不要在未验证实际输出时声称修复完成。
