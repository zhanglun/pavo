import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { MonitorSnapshot } from "../../entities/monitor/model/types";
import type { SchedulerPhoto } from "../../entities/wallpaper/model/types";
import type { FavoriteItem, RotateMode, ThemePreference, UserConfig } from "../../entities/settings/model/types";

export const tauri = {
  wallpapers: {
    getTodayCollection: () => invoke<SchedulerPhoto[]>("get_today_collection"),
    getRecent: (days: number) => invoke<SchedulerPhoto[]>("get_recent_wallpapers", { days }),
    forceRefresh: () => invoke<SchedulerPhoto[]>("force_refresh"),
    // monitorId 缺省 = 所有屏幕（不传字段）；指定 = 逐屏设置
    setAsDesktop: (url: string, monitorId?: string) =>
      invoke<string>("set_as_desktop", monitorId ? { service: "Bing", url, monitorId } : { service: "Bing", url }),
    listMonitors: () => invoke<MonitorSnapshot>("list_monitors"),
    download: (url: string) => invoke<string>("download", { service: "Bing", url }),
  },
  favorites: {
    list: () => invoke<FavoriteItem[]>("list_favorites"),
    add: (item: FavoriteItem) => invoke<UserConfig>("add_favorite", { item }),
    remove: (url: string) => invoke<UserConfig>("remove_favorite", { url }),
  },
  settings: {
    get: () => invoke<UserConfig>("get_config"),
    setThemePreference: (preference: ThemePreference) => invoke<UserConfig>("set_theme_preference", { preference }),
    setAutoDailyUpdate: (enabled: boolean) => invoke<UserConfig>("set_auto_daily_update", { enabled }),
    setHistoryRangeDays: (days: number) => invoke<UserConfig>("set_history_range_days", { days }),
    setAutoRotate: (enabled: boolean) => invoke<UserConfig>("set_auto_rotate", { enabled }),
    setRotateInterval: (minutes: number) => invoke<UserConfig>("set_rotate_interval", { minutes }),
    setRotateMode: (mode: RotateMode) => invoke<UserConfig>("set_rotate_mode", { mode }),
    setAutoStart: (enabled: boolean) => invoke<UserConfig>("set_auto_start", { enabled }),
    setCacheRetentionDays: (days: number) => invoke<UserConfig>("set_cache_retention_days", { days }),
    revealLog: () => invoke<void>("reveal_log_file"),
  },
  events: { listen },
};
