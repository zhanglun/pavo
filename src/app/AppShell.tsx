import { useState } from "react";
import { useThemePreference } from "../features/theme/model/useThemePreference";
import { hideWindow } from "../shared/platform/window";
import { checkForUpdates } from "../shared/platform/updater";
import { useTauriEvents } from "./useTauriEvents";
import { TodayPage } from "../pages/today/TodayPage";
export function AppShell() {
  const [view, setView] = useState<"today" | "history" | "favorites" | "settings">("today");
  useThemePreference();
  useTauriEvents({ onRefresh: () => undefined, onSettings: () => setView("settings"), onUpdate: () => { void checkForUpdates(); } });
  return <main aria-label="Pavo" role="application"><header><span data-tauri-drag-region>Pavo</span><button aria-label="刷新">↻</button><button aria-label="设置" onClick={() => setView("settings")}>⚙</button><button aria-label="隐藏到托盘" onClick={() => { void hideWindow(); }}>−</button></header><nav>{(["today", "history", "favorites"] as const).map((item) => <button key={item} role="tab" aria-selected={view === item} onClick={() => setView(item)}>{item === "today" ? "今日" : item === "history" ? "历史" : "收藏"}</button>)}</nav>{view === "today" ? <TodayPage /> : <section>{view === "settings" ? "设置" : view === "history" ? "历史" : "收藏"}</section>}</main>;
}
