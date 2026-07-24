import { useEffect, useState } from "react";
import { Heart } from "lucide-react";
import type { FavoriteItem } from "../../entities/settings/model/types";
import type { Wallpaper } from "../../entities/wallpaper/model/types";
import { WallpaperImage } from "../../entities/wallpaper/ui/WallpaperImage";
import { useDownloadWallpaper } from "../../features/download/model/useDownloadWallpaper";
import { useSetWallpaper } from "../../features/set-wallpaper/model/useSetWallpaper";
import { openExternal } from "../../shared/platform/shell";
import { tauri } from "../../shared/tauri/client";
import { Menu } from "../../shared/ui/menu/Menu";
import { useToast } from "../../shared/ui/toast/ToastProvider";
import { Tooltip } from "../../shared/ui/tooltip/Tooltip";
import { formatFolioDate } from "../../shared/utils/date";
import styles from "./FavoritesPage.module.css";

const toWallpaper = (item: FavoriteItem): Wallpaper => ({ id: item.url, filename: item.filename, regionCode: "", region: "", imageUrl: item.url, title: item.title, date: item.startdate, copyright: item.copyright, sourceUrl: item.copyrightlink });

export function FavoritesPage({ onToggleFavorite, refreshSignal }: { onToggleFavorite: (wallpaper: Wallpaper) => void | Promise<void>; refreshSignal: number }) {
  const [items, setItems] = useState<Wallpaper[]>([]);
  const [state, setState] = useState<"loading" | "ready" | "error">("loading");
  const setWallpaper = useSetWallpaper();
  const download = useDownloadWallpaper();
  const { show } = useToast();
  useEffect(() => {
    let active = true;
    setState("loading");
    tauri.favorites.list().then((favorites) => {
      if (active) {
        // 后端按收藏时间升序存储（push 追加），这里反转为降序，最近收藏的排前面。
        setItems(favorites.map(toWallpaper).reverse());
        setState("ready");
      }
    }).catch(() => { if (active) setState("error"); });
    return () => { active = false; };
  }, [refreshSignal]);
  if (state === "loading") return <section className={styles.state}>正在整理收藏册页…</section>;
  if (state === "error") return <section className={styles.state}>收藏暂时无法读取，请稍后重试。</section>;
  return <section className={styles.page} aria-labelledby="favorites-title">
    <header><p>COLLECTION</p><h1 id="favorites-title">收藏册页</h1></header>
    {items.length === 0 ? <div className={styles.empty}><span><Heart size={34} strokeWidth={1} /></span><p>还没有收藏的壁纸</p><small>在今日或历史册页点按心形即可收录</small></div> : <div className={styles.grid}>{items.map((item) => {
      const date = formatFolioDate(item.date);
      return <article className={styles.card} key={item.id}>
        <div className={styles.image}><WallpaperImage wallpaper={item} /></div>
        <div className={styles.meta}><time>{date.year} · {date.monthDay}</time><h2>{item.title}</h2></div>
        <div className={styles.actions}><Tooltip label="取消收藏"><button aria-label={`取消收藏：${item.title}`} onClick={async () => { try { await onToggleFavorite(item); setItems((current) => current.filter((candidate) => candidate.id !== item.id)); } catch { show({ tone: "error", message: "取消收藏失败，请重试" }); } }}><Heart size={13} strokeWidth={1.75} fill="currentColor" /></button></Tooltip><Menu label={`更多操作：${item.title}`} items={[
          { id: "set", label: "设为桌面", disabled: setWallpaper.pending, onSelect: () => void setWallpaper.setWallpaper(item.imageUrl) },
          { id: "download", label: "下载原图", disabled: download.pending, onSelect: () => void download.download(item.imageUrl) },
          { id: "source", label: "在 Bing 中查看 ↗", disabled: !item.sourceUrl, onSelect: () => void openExternal(item.sourceUrl) },
        ]} /></div>
      </article>;
    })}</div>}
  </section>;
}
