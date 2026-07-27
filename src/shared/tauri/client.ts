import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { SchedulerPhoto } from "../../entities/wallpaper/model/types";
import type { FavoriteItem, RotateMode, ThemePreference, UserConfig } from "../../entities/settings/model/types";

export type SetWallpaperResult = { confirmed: boolean };

export const tauri = {
  wallpapers: {
    getTodayCollection: () => invoke<SchedulerPhoto[]>("get_today_collection"),
    getRecent: (days: number) => invoke<SchedulerPhoto[]>("get_recent_wallpapers", { days }),
    forceRefresh: () => invoke<SchedulerPhoto[]>("force_refresh"),
    setAsDesktop: (url: string, operationId: string) => invoke<SetWallpaperResult>("set_as_desktop", { service: "Bing", url, operationId }),
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
