import { useCallback, useState } from "react";
import { useFavorites } from "../features/favorite/model/useFavorites";
import { useThemePreference } from "../features/theme/model/useThemePreference";
import { FavoritesPage } from "../pages/favorites/FavoritesPage";
import { HistoryPage } from "../pages/history/HistoryPage";
import { SettingsPage } from "../pages/settings/SettingsPage";
import { TodayPage } from "../pages/today/TodayPage";
import { hideWindow } from "../shared/platform/window";
import { checkForUpdates } from "../shared/platform/updater";
import { tauri } from "../shared/tauri/client";
import { Tooltip } from "../shared/ui/tooltip/Tooltip";
import { useTauriEvents } from "./useTauriEvents";
import styles from "./AppShell.module.css";

const tabs = [
  ["today", "今日"],
  ["history", "历史"],
  ["favorites", "收藏"],
] as const;

export function AppShell() {
  const [view, setView] = useState<"today" | "history" | "favorites" | "settings">("today");
  const [refreshSignal, setRefreshSignal] = useState(0);
  const favorites = useFavorites();
  useThemePreference();
  const reload = useCallback(() => { setRefreshSignal((value) => value + 1); void favorites.reload(); }, [favorites.reload]);
  const openSettings = useCallback(() => setView("settings"), []);
  const update = useCallback(() => { void checkForUpdates(); }, []);
  const forceRefresh = useCallback(async () => { await tauri.wallpapers.forceRefresh(); reload(); }, [reload]);
  useTauriEvents({ onRefresh: reload, onSettings: openSettings, onUpdate: update });
  const content = view === "today"
    ? <TodayPage favoriteIds={favorites.favoriteIds} onToggleFavorite={favorites.toggle} refreshSignal={refreshSignal} />
    : view === "history"
      ? <HistoryPage favoriteIds={favorites.favoriteIds} onToggleFavorite={favorites.toggle} refreshSignal={refreshSignal} />
      : view === "favorites"
        ? <FavoritesPage onToggleFavorite={favorites.toggle} refreshSignal={refreshSignal} />
        : <SettingsPage onBack={() => setView("today")} onHistoryRangeChanged={reload} />;
  return (
    <main className={styles.shell} data-settings={view === "settings"} aria-label="Pavo" role="application">
      <header className={styles.topbar}>
        <span className={styles.brand} data-tauri-drag-region>Pavo</span>
        <Tooltip label="刷新" side="bottom"><button className={styles.windowAction} aria-label="刷新" onClick={() => void forceRefresh()}>↻</button></Tooltip>
        <Tooltip label="设置" side="bottom"><button className={styles.windowAction} aria-label="设置" onClick={openSettings}>⚙</button></Tooltip>
        <Tooltip align="end" label="隐藏到托盘" side="bottom"><button className={styles.windowAction} aria-label="隐藏到托盘" onClick={() => { void hideWindow(); }}>−</button></Tooltip>
      </header>
      {view !== "settings" && (
        <nav className={styles.tabs} aria-label="主要页面" role="tablist">
          {tabs.map(([item, label]) => (
            <button className={styles.tab} key={item} role="tab" aria-selected={view === item} onClick={() => setView(item)}>
              {label}
            </button>
          ))}
        </nav>
      )}
      <section className={styles.content}>{content}</section>
    </main>
  );
}
