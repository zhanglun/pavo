import type { ThemePreference } from "../../../entities/settings/model/types";

const isThemePreference = (value: string | null): value is ThemePreference => value === "system" || value === "light" || value === "dark";

export function applyInitialTheme() {
  const stored = localStorage.getItem("pavo.theme-preference");
  const preference: ThemePreference = isThemePreference(stored) ? stored : "system";
  const dark = preference === "dark" || (preference === "system" && window.matchMedia?.("(prefers-color-scheme: dark)").matches);
  document.documentElement.dataset.theme = dark ? "dark" : "light";
}
