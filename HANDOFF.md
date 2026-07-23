# Pavo React「每日册页」重构交接文档

更新时间：2026-07-23
当前状态：React 重构、旧前端清理及 macOS/Windows 双平台验收已完成，进入最终验证和合并准备阶段

## 1. 工作位置

- 仓库：`/Users/zhanglun/Documents/mine/pavo`
- 开发 worktree：`/Users/zhanglun/Documents/mine/pavo/.worktrees/react-daily-folio`
- 分支：`feature/react-daily-folio`
- 相对 `main`：以当前分支实际提交数为准
- 当前 worktree：修改本文档前为干净状态

后续开发和验收请在 worktree 中进行：

```bash
cd /Users/zhanglun/Documents/mine/pavo/.worktrees/react-daily-folio
```

## 2. 已确认的产品与技术原则

- 技术栈：React 19、TypeScript、Vite、Tauri v2。
- 原则：“功能等价为底线，但界面完全按每日册页重做”。
- 首发兼顾 macOS 与 Windows。
- 气质：优雅、高端、克制、低噪音。
- 核心动作：设为桌面。
- 主题：跟随系统 / 浅色 / 暗色，写入 Rust 配置并使用本地镜像减少启动闪烁。
- 不使用 Radix Primitives、重量级组件库、全局状态库、Tailwind 或完整 FSD。
- 目录采用轻量 `app / pages / entities / features / shared` 分层。
- 不恢复旧 TodayHero 自动预览轮播；设置中的自动更换壁纸是独立产品功能，继续保留。

完整设计规格：

`docs/superpowers/specs/2026-07-22-react-daily-folio-redesign-design.md`

最终验收清单：

`docs/react-daily-folio-acceptance-checklist.md`

## 3. 已完成范围

### React 架构与旧前端清理

- Vite 入口已迁移至 React。
- 页面、领域模型、功能与平台调用已按轻量分层整理。
- Tauri command、事件、更新器、Shell 和窗口能力均通过类型化封装调用。
- 旧 Svelte 页面、组件和相关无引用工具已删除。
- Svelte、Tailwind、Flowbite 及对应 Vite 依赖已移除。
- README 与架构文档已改为 React 技术栈。

### 今日册页

- 使用非对称日历版式：月份、年份、超大日号和星期。
- 主图、标题、版权摘要、完整介绍和操作区均已实现。
- 作品说明区使用固定版面，切换地区不会推动下方内容。
- 完整介绍使用覆盖式纸卡，支持关闭、Esc 和焦点恢复。
- 地区入口使用双列摄影联页，显示全部可用地区。
- 今日数据不足时读取近 7 天区域回退；界面不展示“近期内容”等内部策略文案，以真实日期标记条目。
- 主图与地区回退均按地区和图片身份去重。
- 支持设为桌面、收藏、下载原图和在 Bing 中查看。

### 历史档案

- 使用单一日期归档列表，不再重复渲染同一视觉的地区变体。
- 历史范围读取 7/14 天设置。
- 每项支持收藏、设为桌面、下载原图和在 Bing 中查看。
- 次级操作统一收纳在安静的 `⋯` 菜单。

### 收藏册页

- 使用双列私人册页。
- 使用心形图标取消收藏，不显示“取消收藏”操作文字。
- 收藏变更可在今日、历史和收藏间同步。
- 移除失败时保留卡片并显示错误反馈。
- 支持设为桌面、下载原图和在 Bing 中查看。

### 设置与系统能力

- 三档主题及系统主题联动。
- 开机启动。
- 每日自动更新。
- 手动检查更新、下载安装、重启。
- 自动更换壁纸、间隔和顺序模式。
- 历史范围与缓存保留。
- 日志、版本、GitHub 和反馈入口。
- 更新流程具有全局并发互斥，避免按钮与托盘重复触发。

### 交互与稳定性

- Toast 使用浮层，不进入页面文档流。
- 壁纸设置过程显示持续进度，结束后更新结果。
- Tooltip、菜单键盘操作、焦点恢复和细滚动条已实现。
- 设置、收藏、系统操作和网络失败均有反馈。
- Bing 联网单元测试已替换为固定 fixture，不再依赖实时接口。
- “查看来源”已更名为“在 Bing 中查看”，准确表达 Bing API 返回搜索链接的事实。

## 4. 最近阶段提交

```text
1cf1fc0 fix(ui): 明确 Bing 来源链接语义
f3f2caf feat(today): 落地非对称日历日期版式
a81aae0 feat(today): 将地区入口重做为摄影联页
e4ae7fb fix(today): 稳定地区切换并重做册页索引
9880777 feat(today): 补充版权说明就地展开，对齐设计稿
1fa4387 fix(today): 消除切换地区的主体抖动，重做地区栏选中态
88706d7 chore: 清理旧 Svelte 代码与依赖，文档改为 React
6332fcb test(services): 用固定 fixture 替换不稳定的 Bing 联网测试
da9c790 fix: 补齐关键操作反馈与启动稳定性
```

## 5. 当前验证证据

最近一次前端完整验证：

- `pnpm test`：14 个测试文件、49 项测试全部通过。
- `pnpm typecheck`：通过。
- `pnpm build`：通过，79 个模块完成生产构建。
- Impeccable 设计规则扫描：今日页无告警。
- `git diff --check`：通过。

最近一次 Rust 完整验证：

- `cargo test`：3 个测试套件、32 项测试通过。
- `cargo check`：通过。

双平台运行证据：

- `pnpm tauri dev`：macOS 可冷启动并显示真实数据。
- macOS 最终人工功能验收：全部通过。
- Windows 实机冒烟与功能验收：全部通过。

建议最终复验：

```bash
pnpm test
pnpm typecheck
pnpm build
cd src-tauri && cargo test && cargo check
cd .. && pnpm tauri dev
```

为节省时间，日常小改优先运行相关测试与 `pnpm typecheck`；合并或发布前再执行完整构建和 Rust 验证。

## 6. 双平台验收结果

- macOS 最终人工功能验收：全部通过，无失败项或明确延期项。
- Windows 实机冒烟与功能验收：全部通过，无失败项或明确延期项。
- 已覆盖壁纸设置、下载、收藏、历史、主题、托盘、开机启动、更新、自动更换壁纸、异常恢复及系统路径。

## 7. 当前未完成项

### 合并与发布准备

- 更新最终产品截图。
- 决定是否将根目录未跟踪的 `pavo-redesign-comparison.html` 纳入版本管理。
- 完成最终全量验证。
- 决定合并、PR 或发布方式。

## 8. 推荐后续顺序

1. 执行前端构建、Rust 全量测试与最终静态检查。
2. 更新截图和发布文档。
3. 决定设计 HTML 的版本管理方式。
4. 合并 `feature/react-daily-folio` 或创建 PR。

## 9. 交接约束

- 每次修改代码前必须先取得用户确认；前一次授权不自动覆盖下一次修改。
- 阶段任务完成后自审、验证并提交。
- Git 提交信息使用中文，不包含 AI 相关措辞。
- 避免无意义地重复 Build，但合并和发布前不得跳过完整验证。
- 不在未验证实际输出时声称修复完成。
- 主目录中的设计 HTML 属于独立未跟踪文件，提交功能分支时不要误带其他用户文件。
