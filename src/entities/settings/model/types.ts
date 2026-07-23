export type ThemePreference = "system" | "light" | "dark";
export type RotateMode = "Sequential" | "Random";
export type FavoriteItem = { filename: string; url: string; title: string; startdate: string; copyright: string; copyrightlink: string; local_path?: string | null };
export type UserConfig = { auto_daily_update: boolean; history_range_days: number; favorites: FavoriteItem[]; auto_rotate: boolean; rotate_interval_minutes: number; rotate_mode: RotateMode; auto_start: boolean; cache_retention_days: number; theme_preference: ThemePreference };
