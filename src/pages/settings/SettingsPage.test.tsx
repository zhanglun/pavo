import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect, test, vi } from "vitest";
import type { UserConfig } from "../../entities/settings/model/types";
import { getAutostartEnabled } from "../../shared/platform/autostart";
import { tauri } from "../../shared/tauri/client";
import { ToastProvider } from "../../shared/ui/toast/ToastProvider";
import { SettingsPage } from "./SettingsPage";

vi.mock("../../shared/platform/autostart", () => ({ getAutostartEnabled: vi.fn() }));
vi.mock("../../shared/platform/updater", () => ({ checkForUpdates: vi.fn() }));
vi.mock("../../shared/platform/shell", () => ({ openExternal: vi.fn() }));
vi.mock("../../shared/tauri/client", () => ({ tauri: { settings: {
  get: vi.fn(), setThemePreference: vi.fn(), setAutoDailyUpdate: vi.fn(), setHistoryRangeDays: vi.fn(), setAutoRotate: vi.fn(), setRotateInterval: vi.fn(), setRotateMode: vi.fn(), setAutoStart: vi.fn(), setCacheRetentionDays: vi.fn(), revealLog: vi.fn(),
} } }));

const config = (overrides: Partial<UserConfig> = {}): UserConfig => ({ auto_daily_update: true, history_range_days: 7, favorites: [], auto_rotate: false, rotate_interval_minutes: 60, rotate_mode: "Sequential", auto_start: false, cache_retention_days: 14, theme_preference: "system", ...overrides });

beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear();
  vi.mocked(tauri.settings.get).mockResolvedValue(config());
  vi.mocked(getAutostartEnabled).mockResolvedValue(false);
});

const renderPage = () => render(<ToastProvider><SettingsPage onBack={vi.fn()} onHistoryRangeChanged={vi.fn()} /></ToastProvider>);

test("uses restrained text states and reveals rotation details only when enabled", async () => {
  vi.mocked(tauri.settings.setAutoRotate).mockResolvedValue(config({ auto_rotate: true }));
  const user = userEvent.setup(); renderPage();
  const toggle = await screen.findByRole("button", { name: "自动轮播壁纸" });
  expect(toggle).toHaveAttribute("aria-pressed", "false");
  expect(toggle).toHaveTextContent("已关闭");
  expect(screen.queryByLabelText("切换间隔")).not.toBeInTheDocument();
  await user.click(toggle);
  expect(await screen.findByLabelText("切换间隔")).toBeVisible();
});

test("updates the theme mirror only after Rust persistence succeeds", async () => {
  vi.mocked(tauri.settings.setThemePreference).mockResolvedValue(config({ theme_preference: "dark" }));
  const changed = vi.fn();
  window.addEventListener("pavo:theme-preference-changed", changed);
  const user = userEvent.setup(); renderPage();
  await user.click(await screen.findByRole("radio", { name: "深色" }));
  await waitFor(() => expect(localStorage.getItem("pavo.theme-preference")).toBe("dark"));
  expect(document.documentElement.dataset.theme).toBe("dark");
  expect(changed).toHaveBeenCalledOnce();
  window.removeEventListener("pavo:theme-preference-changed", changed);
});

test("keeps the old theme when persistence fails", async () => {
  vi.mocked(tauri.settings.setThemePreference).mockRejectedValue(new Error("write failed"));
  const user = userEvent.setup(); renderPage();
  await user.click(await screen.findByRole("radio", { name: "深色" }));
  expect(localStorage.getItem("pavo.theme-preference")).toBeNull();
  expect(screen.getByRole("radio", { name: "跟随系统" })).toBeChecked();
});

test("offers only the approved archive and retention values", async () => {
  renderPage();
  expect(await screen.findByLabelText("历史范围")).toHaveTextContent("7 天14 天");
  expect(screen.getByLabelText("缓存保留")).toHaveTextContent("3 天7 天14 天30 天");
});
