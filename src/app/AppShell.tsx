import { useCallback, useState } from "react";
import { useFavorites } from "../features/favorite/model/useFavorites";
import { useThemePreference } from "../features/theme/model/useThemePreference";
import { TodayPage } from "../pages/today/TodayPage";
import { hideWindow } from "../shared/platform/window";
import { checkForUpdates } from "../shared/platform/updater";
import { tauri } from "../shared/tauri/client";
import { useTauriEvents } from "./useTauriEvents";
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
  return <main aria-label="Pavo" role="application"><header><span data-tauri-drag-region>Pavo</span><button aria-label="刷新" onClick={() => void forceRefresh()}>↻</button><button aria-label="设置" onClick={openSettings}>⚙</button><button aria-label="隐藏到托盘" onClick={() => { void hideWindow(); }}>−</button></header><nav>{(["today", "history", "favorites"] as const).map((item) => <button key={item} role="tab" aria-selected={view === item} onClick={() => setView(item)}>{item === "today" ? "今日" : item === "history" ? "历史" : "收藏"}</button>)}</nav><section>{view === "today" ? <TodayPage favoriteIds={favorites.favoriteIds} onToggleFavorite={favorites.toggle} refreshSignal={refreshSignal} /> : <h1>{view === "settings" ? "设置" : view === "history" ? "历史" : "收藏"}</h1>}</section></main>;
}
