import { useCallback, useEffect, useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import type { FavoriteItem } from "../../../entities/settings/model/types";
import type { Wallpaper } from "../../../entities/wallpaper/model/types";

const toFavorite = (wallpaper: Wallpaper): FavoriteItem => ({
  filename: wallpaper.filename,
  url: wallpaper.imageUrl,
  title: wallpaper.title,
  startdate: wallpaper.date,
  copyright: wallpaper.copyright,
  copyrightlink: wallpaper.sourceUrl,
});

export function useFavorites() {
  const [favoriteIds, setFavoriteIds] = useState<Set<string>>(new Set());
  const reload = useCallback(async () => {
    try {
      const items = await tauri.favorites.list();
      setFavoriteIds(new Set(items.map((item) => item.url)));
    } catch {
      setFavoriteIds(new Set());
    }
  }, []);

  useEffect(() => { void reload(); }, [reload]);

  const toggle = async (wallpaper: Wallpaper) => {
    const exists = favoriteIds.has(wallpaper.imageUrl);
    await (exists ? tauri.favorites.remove(wallpaper.imageUrl) : tauri.favorites.add(toFavorite(wallpaper)));
    setFavoriteIds((current) => {
      const next = new Set(current);
      exists ? next.delete(wallpaper.imageUrl) : next.add(wallpaper.imageUrl);
      return next;
    });
  };

  return { favoriteIds, reload, toggle };
}
