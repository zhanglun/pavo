import { useEffect, useMemo, useState } from "react";
import { selectRecentWallpapers, selectTodayWallpapers } from "../../entities/wallpaper/model/normalize";
import type { Wallpaper } from "../../entities/wallpaper/model/types";
import { WallpaperImage } from "../../entities/wallpaper/ui/WallpaperImage";
import { RegionRail } from "../../entities/wallpaper/ui/RegionRail";
import { useDownloadWallpaper } from "../../features/download/model/useDownloadWallpaper";
import { useSetWallpaper } from "../../features/set-wallpaper/model/useSetWallpaper";
import { openExternal } from "../../shared/platform/shell";
import { tauri } from "../../shared/tauri/client";
import { Menu } from "../../shared/ui/menu/Menu";
import { formatFolioDate, localDateKey } from "../../shared/utils/date";
import styles from "./TodayPage.module.css";

type Props = {
  favoriteIds: Set<string>;
  onToggleFavorite: (wallpaper: Wallpaper) => void | Promise<void>;
  refreshSignal: number;
};

export function TodayPage({ favoriteIds, onToggleFavorite, refreshSignal }: Props) {
  const [items, setItems] = useState<Wallpaper[]>([]);
  const [selectedId, setSelectedId] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [isFallback, setIsFallback] = useState(false);
  const setWallpaper = useSetWallpaper();
  const download = useDownloadWallpaper();

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(false);
    (async () => {
      try {
        const today = await tauri.wallpapers.getTodayCollection();
        const todayItems = selectTodayWallpapers(today, localDateKey());
        const fallback = todayItems.length <= 1;
        const next = fallback
          ? selectRecentWallpapers(await tauri.wallpapers.getRecent(7), localDateKey(), 7).slice(0, 10)
          : todayItems;
        if (!active) return;
        setItems(next);
        setSelectedId((current) => next.some((item) => item.id === current) ? current : (next[0]?.id ?? ""));
        setIsFallback(fallback);
      } catch {
        if (active) setError(true);
      } finally {
        if (active) setLoading(false);
      }
    })();
    return () => { active = false; };
  }, [refreshSignal]);

  const selected = useMemo(() => items.find((item) => item.id === selectedId) ?? items[0], [items, selectedId]);
  if (loading) return <section className={styles.state}>正在整理今日册页…</section>;
  if (error) return <section className={styles.state}>今日壁纸暂时无法读取，请刷新重试。</section>;
  if (!selected) return <section className={styles.state}>今天还没有可用的壁纸。</section>;

  const date = formatFolioDate(selected.date);
  const favorite = favoriteIds.has(selected.imageUrl);
  return <section className={styles.page} aria-labelledby="today-title">
    <header className={styles.dateSeal}>
      <span>{date.monthDay}</span><span>{date.year} · {date.weekday}</span>
      {isFallback && <em>近期内容</em>}
    </header>
    <div className={styles.hero}><WallpaperImage wallpaper={selected} /></div>
    <div className={styles.story}>
      <p className={styles.region}>{selected.region}</p>
      <h1 id="today-title">{selected.title}</h1>
      <p className={styles.description}>{selected.copyright}</p>
    </div>
    <div className={styles.actions}>
      <button className={styles.primary} disabled={setWallpaper.pending} onClick={() => void setWallpaper.setWallpaper(selected.imageUrl)}>设为桌面</button>
      <button className={styles.iconButton} aria-label={favorite ? `取消收藏：${selected.title}` : `收藏：${selected.title}`} onClick={() => void onToggleFavorite(selected)}>{favorite ? "♥" : "♡"}</button>
      <Menu label={`更多操作：${selected.title}`} items={[
        { id: "download", label: "下载原图", disabled: download.pending, onSelect: () => void download.download(selected.imageUrl) },
        { id: "source", label: "介绍与来源 ↗", disabled: !selected.sourceUrl, onSelect: () => void openExternal(selected.sourceUrl) },
      ]} />
    </div>
    <div className={styles.regions}><p>今日各地</p><RegionRail items={items} selectedId={selected.id} onSelect={(item) => setSelectedId(item.id)} /></div>
  </section>;
}
