# React 每日册页重构设计规格

## 目标

将 Pavo 的前端从 Svelte 迁移至 React 19，同时完整重做界面为「每日册页（Daily Folio）」风格。现有壁纸、缓存、托盘与配置等 Rust/Tauri 业务能力保持可用；界面结构不受旧 Svelte 布局约束。

## 已确认的全局约束

- 技术栈：React 19、TypeScript、Vite、Tauri v2。
- 首发验收平台：macOS 与 Windows。
- 窗口：默认约 `340 × 800`，最小 `340 × 720`，仅允许纵向拉伸。
- 迁移原则：功能等价为底线，界面按每日册页完全重做。
- 不使用 Svelte、Tailwind、Flowbite、MUI、Ant Design 或 Radix Primitives。
- 通用交互组件自行实现；`Menu` 必须支持键盘、Esc、外部点击关闭与焦点回归。
- 主题持久化为 `system`、`light`、`dark` 三档；旧配置缺失该字段时默认为 `system`。
- 所有异步失败应有明确反馈；不删除上一次成功加载的数据。

## 设计原则

- 核心动作「设为桌面」是每个壁纸操作区唯一高强调动作。
- 气质应优雅、克制、低噪音：留白优先，避免通用后台组件感。
- 每日内容是主角；各地区每日一图必须完整可见、可切换。
- 下载与来源等次级动作收纳至更多菜单，减少并列按钮。
- 浅色与深色是同一套层级关系，不是简单反色。

## 信息架构

应用采用应用内顶栏与三个主视图，设置由顶栏进入：

```text
AppShell
├── Topbar：窗口拖拽、刷新、设置、隐藏至托盘
├── Navigation：今日 / 历史 / 收藏
├── TodayPage
├── HistoryPage
├── FavoritesPage
└── SettingsPage：由 Topbar 进入，可返回今日
```

关闭窗口的既有行为保持不变：隐藏至托盘。macOS 与 Windows 都使用应用内顶栏；只有无交互的安全空白区域可拖拽窗口。

## 页面规格

### 今日 TodayPage

页面按以下顺序呈现：

1. 克制的日历印章，显示 `2026 · 07.22 · 周三` 形式的日期信息。
2. 当前选中地区的主壁纸。
3. 标题、1–2 行版权/故事摘要与来源入口。
4. 主按钮「设为桌面」、心形收藏按钮，以及包含「下载原图、介绍与来源」的更多菜单。
5. 「今日各地」横向册页，展示当日所有地区壁纸；点击卡片切换主壁纸。

主图只代表已选地区；区域册页不能因空间限制被合并、轮播或省略。

### 历史 HistoryPage

以日期归档展示单一历史列表，取代旧的「近期合集 + 近期壁纸」双区结构。每项显示缩略图、日期、标题与一行版权摘要。

- 右侧保留心形收藏和 `⋯`。
- `⋯` 打开悬浮纸卡菜单，按顺序提供「设为桌面、下载原图、介绍与来源」。
- 菜单不能在列表行内展开，不使用并排的三个操作按钮。
- 继续支持 7 天、14 天历史范围。

### 收藏 FavoritesPage

以双列私人册页呈现收藏壁纸。

- 实心心形代表已收藏；点击即取消收藏，并显示轻量反馈。
- 不展示常驻的「取消收藏」文字按钮。
- `⋯` 提供「设为桌面、下载原图、介绍与来源」。
- 空状态引导用户从今日或历史收藏壁纸。

### 设置 SettingsPage

设置作为独立页面从顶栏齿轮进入，返回后回到今日。分组如下：

1. 外观：主题三档。
2. 启动与更新：开机自启、自动每日更新、检查更新。
3. 壁纸轮播：自动轮播开关；开关关闭时隐藏间隔与顺序设置。
4. 资料库与存储：历史范围、缓存保留时间、打开日志。
5. 支持：版本、GitHub 与反馈入口。

## FSD-Lite 前端结构

```text
src/
  app/                    # React 入口、AppShell、导航、全局 Tauri 事件协调
  pages/                  # today、history、favorites、settings
  entities/                # wallpaper、favorite、settings 类型与纯数据转换
  features/                # set-wallpaper、download、favorite、theme
  shared/
    ui/                    # Button、IconButton、Menu、Switch、Toast、EmptyState
    tauri/                 # 唯一的 Rust command/event 包装层
    platform/              # window、shell、updater 插件封装
    styles/                # tokens.css、global.css
    utils/                 # 日期、格式化等纯工具
```

页面或组件的样式使用 CSS Modules（`*.module.css`）。全局仅保留设计 token、重置、字体与窗口根样式。每个组件只负责一种视觉/交互职责，避免以单次使用为目的的泛化抽象。

## 数据与状态边界

### 壁纸实体

Rust 当前返回的地区平行数组必须在 `entities/wallpaper` 内一次性规范化；页面不能直接依赖数组下标。

```ts
type Wallpaper = {
  id: string;
  region: string;
  imageUrl: string;
  title: string;
  date: string;
  description: string;
  sourceUrl: string;
};
```

「今日各地」、历史与收藏均消费此统一实体。收藏项的本地路径等扩展字段保留在收藏实体中，不污染通用壁纸展示模型。

### 状态

- Toast 是唯一的 React Context/Provider。
- 主题偏好、设置、收藏分别由独立 Hook 读取和更新；不建立万能全局 Store。
- 选中地区、菜单是否打开、列表展开状态等留在对应页面本地。
- 设为桌面与下载的进行中状态由各 feature 管理，防止重复触发。

### Tauri 桥接

`shared/tauri` 是调用 Rust `invoke` 与订阅 Rust `listen` 的唯一位置。保留当前命令语义：

- 壁纸：`set_as_desktop`、`download`、`get_today_collection`、`get_recent_wallpapers`、`force_refresh`。
- 收藏：`list_favorites`、`add_favorite`、`remove_favorite`。
- 设置：`get_config`、每日更新、历史范围、轮播、开机启动、缓存保留、日志。
- 主题：配置新增 `theme_preference` 并增加对应写入命令。

`shared/platform` 负责 updater、shell 外部链接与窗口隐藏等插件调用，不与 Rust command 包装混合。

## 事件与刷新策略

`AppShell` 统一订阅事件，并确保卸载时取消订阅：

- `wallpapers:cache-refreshed` 与 `window:shown`：刷新今日和历史数据。
- `wallpaper:changed`：只更新当前已设壁纸标记，不进行全量列表刷新。
- `go-to-settings`：进入设置页面。
- `check-for-updates`：调用更新流程。

事件处理不应让窗口从托盘显示时发生不必要的闪动或全局重置。

## 主题策略

主题偏好写入现有 Pavo 本地配置：

- `system`：使用 `prefers-color-scheme`，并监听其变动。
- `light`：固定浅色。
- `dark`：固定深色。

Rust 配置迁移需对旧 TOML 容错：缺失字段、无效字段都回退到 `system`。前端在配置读取前可先以系统主题渲染，读取完成后无闪烁切换到已保存偏好。

## 错误、加载与空状态

- 首次加载显示骨架；无数据展示克制空状态。
- 加载失败显示局部重试入口，同时保留上一次成功数据。
- 设为桌面和下载时，触发按钮显示进行中状态并禁用重复提交。
- 设置不采用乐观写入：Rust 成功持久化后更新 UI；失败则保留旧值并显示 Toast。
- 外部链接、下载与系统操作失败均展示简短错误 Toast；用户主动取消不显示错误。

## 验收与测试

### 自动化

- 使用 Vitest 与 React Testing Library。
- 覆盖地区数组到 `Wallpaper` 的规范化、日期格式化、收藏切换、设为桌面、下载、三档主题、轮播条件显示。
- `Menu` 覆盖键盘触发、Esc 关闭、外部点击关闭、焦点回归与禁用项。
- Rust 为主题配置的默认值、旧配置兼容和写入补充单元测试；既有 Rust 测试持续通过。

### 人工冒烟

在 macOS 与 Windows 分别验证：窗口拖拽/拉伸、隐藏至托盘、托盘打开设置、每日多地区切换、设为桌面、下载、收藏、历史菜单、主题持久化、轮播、更新与日志入口。

## 非目标

- 不重写 Rust 的壁纸下载、缓存、轮播、托盘或更新业务。
- 不引入 Radix、重量级组件库、全局状态库或端到端测试框架。
- 不保留旧的「近期合集」与「近期壁纸」双区信息结构。
