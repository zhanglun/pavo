// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  interface FavoriteItem {
    filename: string;
    url: string;
    title: string;
    startdate: string;
    copyright: string;
    copyrightlink: string;
    local_path?: string | null;
  }
  interface UserConfig {
    auto_daily_update: boolean;
    history_range_days: number;
    favorites: FavoriteItem[];
    auto_rotate: boolean;
    rotate_interval_minutes: number;
    rotate_mode: "Sequential" | "Random";
  }
  type BingImage = {
    copyrights: string[];
    copyrightlinks: string[];
    startdates: string[];
    titles: string[];
    urls: string[];
  };

  type SchedulerPhoto = {
    filename: string;
    regions: string[];
    urls: string[];
    titles: string[];
    startdates: string[];
    copyrights: string[];
    copyrightlinks: string[];
  };

  type WallpaperActionProps = {
    service: string;
    url: string;
    href: string;
    filename: string;
    favorite: boolean;
  };
}

export {};
