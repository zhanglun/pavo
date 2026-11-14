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
vi.mock("../../shared/tauri/client", () => ({ tauri: {
  settings: {
    get: vi.fn(), setThemePreference: vi.fn(), setAutoDailyUpdate: vi.fn(), setHistoryRangeDays: vi.fn(), setAutoRotate: vi.fn(), setRotateInterval: vi.fn(), setRotateMode: vi.fn(), setAutoStart: vi.fn(), setCacheRetentionDays: vi.fn(), revealLog: vi.fn(),
  },
  wallpapers: { listMonitors: vi.fn(), getRecent: vi.fn() },
} }));

const config = (overrides: Partial<UserConfig> = {}): UserConfig => ({ auto_daily_update: true, history_range_days: 7, favorites: [], auto_rotate: false, rotate_interval_minutes: 60, rotate_mode: "Sequential", auto_start: false, cache_retention_days: 14, theme_preference: "system", ...overrides });

beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear();
  vi.mocked(tauri.settings.get).mockResolvedValue(config());
  vi.mocked(getAutostartEnabled).mockResolvedValue(false);
  vi.mocked(tauri.wallpapers.listMonitors).mockResolvedValue({ per_monitor: false, monitors: [] });
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([]);
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

test("keeps the application identity and cache location discoverable", async () => {
  renderPage();

  expect(await screen.findByRole("img", { name: "Pavo" })).toBeVisible();
  expect(screen.getByText("~/.pavo/")).toBeVisible();
  expect(screen.getByText("Pavo", { selector: "strong" })).toBeVisible();
});

test("lists per-monitor wallpaper state when the system supports it", async () => {
  vi.mocked(tauri.wallpapers.listMonitors).mockResolvedValue({
    per_monitor: true,
    monitors: [
      { id: "1", name: "DELL U2723QE", index: 0, width: 3840, height: 2160, is_primary: true, current_filename: "OHR.Sample.jpg" },
      { id: "2", name: "BenQ", index: 1, width: 1920, height: 1080, is_primary: false, current_filename: null },
    ],
  });
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([{ filename: "OHR.Sample.jpg", regions: ["zh-CN"], urls: [], titles: ["山谷"], startdates: ["20260723"], copyrights: [], copyrightlinks: [] }]);

  renderPage();
  expect(await screen.findByRole("heading", { name: "屏幕" })).toBeVisible();
  expect(screen.getByText("DELL U2723QE · 主")).toBeVisible();
  // 标题映射异步到达
  expect(await screen.findByText("山谷")).toBeVisible();
  expect(screen.getByText("未由 Pavo 设置")).toBeVisible();
});

test("hides the screen section when per-monitor is unsupported", async () => {
  vi.mocked(tauri.wallpapers.listMonitors).mockResolvedValue({ per_monitor: false, monitors: [] });
  renderPage();
  await screen.findByRole("heading", { name: "外观" });
  expect(screen.queryByRole("heading", { name: "屏幕" })).not.toBeInTheDocument();
});
