import { useEffect, useState } from "react";
import { selectHistoryWallpapers } from "../../entities/wallpaper/model/normalize";
import type { Wallpaper } from "../../entities/wallpaper/model/types";
import { WallpaperImage } from "../../entities/wallpaper/ui/WallpaperImage";
import { useDownloadWallpaper } from "../../features/download/model/useDownloadWallpaper";
import { useSetWallpaper } from "../../features/set-wallpaper/model/useSetWallpaper";
import { openExternal } from "../../shared/platform/shell";
import { tauri } from "../../shared/tauri/client";
import { Menu } from "../../shared/ui/menu/Menu";
import { Tooltip } from "../../shared/ui/tooltip/Tooltip";
import { formatFolioDate, localDateKey } from "../../shared/utils/date";
import styles from "./HistoryPage.module.css";

type Props = { favoriteIds: Set<string>; onToggleFavorite: (wallpaper: Wallpaper) => void | Promise<void>; refreshSignal: number };

export function HistoryPage({ favoriteIds, onToggleFavorite, refreshSignal }: Props) {
  const [items, setItems] = useState<Wallpaper[]>([]);
  const [state, setState] = useState<"loading" | "ready" | "error">("loading");
  const setWallpaper = useSetWallpaper();
  const download = useDownloadWallpaper();
  useEffect(() => {
    let active = true;
    setState("loading");
    (async () => {
      try {
        const config = await tauri.settings.get();
        const recent = await tauri.wallpapers.getRecent(config.history_range_days);
        if (active) { setItems(selectHistoryWallpapers(recent, localDateKey(), config.history_range_days)); setState("ready"); }
      } catch { if (active) setState("error"); }
    })();
    return () => { active = false; };
  }, [refreshSignal]);

  if (state === "loading") return <section className={styles.state}>正在翻阅历史档案…</section>;
  if (state === "error") return <section className={styles.state}>历史档案暂时无法读取，请稍后重试。</section>;
  return <section className={styles.page} aria-labelledby="history-title">
    <header><p>ARCHIVE</p><h1 id="history-title">历史档案</h1></header>
    {items.length === 0 ? <p className={styles.state}>这段时间还没有壁纸记录。</p> : <div className={styles.list}>{items.map((item) => {
      const date = formatFolioDate(item.date);
      const favorite = favoriteIds.has(item.imageUrl);
      return <article className={styles.card} key={item.id}>
        <div className={styles.image}><WallpaperImage wallpaper={item} /></div>
        <div className={styles.copy}><time>{date.year} · {date.monthDay}</time><h2>{item.title}</h2><p>{item.copyright}</p></div>
        <div className={styles.actions}>
          <Tooltip label={favorite ? "取消收藏" : "收藏"}><button aria-label={favorite ? `取消收藏：${item.title}` : `收藏：${item.title}`} onClick={() => void onToggleFavorite(item)}>{favorite ? "♥" : "♡"}</button></Tooltip>
          <Menu label={`更多操作：${item.title}`} items={[
            { id: "set", label: "设为桌面", disabled: setWallpaper.pending, onSelect: () => void setWallpaper.setWallpaper(item.imageUrl) },
            { id: "download", label: "下载原图", disabled: download.pending, onSelect: () => void download.download(item.imageUrl) },
            { id: "source", label: "查看来源 ↗", disabled: !item.sourceUrl, onSelect: () => void openExternal(item.sourceUrl) },
          ]} />
        </div>
      </article>;
    })}</div>}
  </section>;
}
