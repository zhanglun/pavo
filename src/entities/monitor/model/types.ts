/** 平台层返回的屏幕运行时描述（每次 list_monitors 实时枚举） */
export type MonitorInfo = {
  id: string;
  name: string;
  index: number;
  width: number;
  height: number;
  is_primary: boolean;
  /** OS 反查的该屏当前壁纸文件名；null 表示不可用 */
  current_filename: string | null;
};

/** 枚举快照：能力位 + 屏幕列表 */
export type MonitorSnapshot = {
  /** false 时（如 Linux/GNOME）逐屏设置不可用，UI 隐藏逐屏入口 */
  per_monitor: boolean;
  monitors: MonitorInfo[];
};
