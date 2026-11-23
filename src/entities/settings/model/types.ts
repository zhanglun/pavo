export type ThemePreference = "system" | "light" | "dark";
export type RotateMode = "Sequential" | "Random";
/** 壁纸模式：统一（所有屏幕同一张）或逐屏独立 */
export type WallpaperMode = "all" | "per_monitor";
export type FavoriteItem = { filename: string; url: string; title: string; startdate: string; copyright: string; copyrightlink: string; local_path?: string | null };
export type UserConfig = { auto_daily_update: boolean; history_range_days: number; favorites: FavoriteItem[]; auto_rotate: boolean; rotate_interval_minutes: number; rotate_mode: RotateMode; wallpaper_mode: WallpaperMode; auto_start: boolean; cache_retention_days: number; theme_preference: ThemePreference };
