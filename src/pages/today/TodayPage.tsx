import { useEffect, useState } from "react";
import { normalizeCollection } from "../../entities/wallpaper/model/normalize";
import type { Wallpaper } from "../../entities/wallpaper/model/types";
import { WallpaperImage } from "../../entities/wallpaper/ui/WallpaperImage";
import { RegionRail } from "../../entities/wallpaper/ui/RegionRail";
import { tauri } from "../../shared/tauri/client";
import { formatFolioDate } from "../../shared/utils/date";
import { useSetWallpaper } from "../../features/set-wallpaper/model/useSetWallpaper";
import { useDownloadWallpaper } from "../../features/download/model/useDownloadWallpaper";

export function TodayPage() {
  const [items, setItems] = useState<Wallpaper[]>([]); const [selected, setSelected] = useState<Wallpaper>();
  const setWallpaper = useSetWallpaper(); const download = useDownloadWallpaper();
  useEffect(() => { tauri.wallpapers.getTodayCollection().then(collection => { const next = collection.flatMap(normalizeCollection); setItems(next); setSelected(next[0]); }); }, []);
  if (!selected) return <section>加载中…</section>;
  const date = formatFolioDate(selected.date);
  return <section><p>{date.year} · {date.monthDay} · {date.weekday}</p><WallpaperImage wallpaper={selected} /><h1>{selected.title}</h1><p>{selected.copyright}</p><button disabled={setWallpaper.pending} onClick={() => setWallpaper.setWallpaper(selected.imageUrl)}>设为桌面</button><button disabled={download.pending} onClick={() => download.download(selected.imageUrl)}>下载原图</button><RegionRail items={items} selectedId={selected.id} onSelect={setSelected} /></section>;
}
