import { useEffect, useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import type { ThemePreference } from "../../../entities/settings/model/types";

function resolve(preference: ThemePreference) { return preference === "system" ? (typeof window !== "undefined" && window.matchMedia?.("(prefers-color-scheme: dark)").matches ? "dark" : "light") : preference; }
export function useThemePreference() {
  const [preference, setPreference] = useState<ThemePreference>("system");
  const apply = (value: ThemePreference) => { document.documentElement.dataset.theme = resolve(value); };
  useEffect(() => { tauri.settings.get().then((config) => { setPreference(config.theme_preference); localStorage.setItem("pavo.theme-preference", config.theme_preference); apply(config.theme_preference); }).catch(() => undefined); }, []);
  useEffect(() => {
    const syncPreference = (event: Event) => {
      const value = (event as CustomEvent<ThemePreference>).detail;
      setPreference(value);
      apply(value);
    };
    window.addEventListener("pavo:theme-preference-changed", syncPreference);
    return () => window.removeEventListener("pavo:theme-preference-changed", syncPreference);
  }, []);
  useEffect(() => {
    if (preference !== "system" || !window.matchMedia) return;
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const sync = () => apply("system");
    media.addEventListener?.("change", sync);
    return () => media.removeEventListener?.("change", sync);
  }, [preference]);
  const savePreference = async (value: ThemePreference) => { const config = await tauri.settings.setThemePreference(value); setPreference(config.theme_preference); localStorage.setItem("pavo.theme-preference", config.theme_preference); apply(config.theme_preference); };
  return { preference, resolvedTheme: resolve(preference), savePreference, loading: false };
}
