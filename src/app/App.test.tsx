import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi } from "vitest";
import { App } from "./App";

const { appConfig } = vi.hoisted(() => ({ appConfig: { auto_daily_update: true, history_range_days: 7, favorites: [], auto_rotate: false, rotate_interval_minutes: 60, rotate_mode: "Sequential", auto_start: false, cache_retention_days: 14, theme_preference: "system" } }));

vi.mock("../shared/tauri/client", () => ({ tauri: {
  favorites: { list: vi.fn().mockResolvedValue([]), add: vi.fn(), remove: vi.fn() },
  wallpapers: { getTodayCollection: vi.fn().mockResolvedValue([]), getRecent: vi.fn().mockResolvedValue([]), forceRefresh: vi.fn(), setAsDesktop: vi.fn(), download: vi.fn() },
  settings: { get: vi.fn().mockResolvedValue(appConfig), setThemePreference: vi.fn(), setAutoDailyUpdate: vi.fn(), setHistoryRangeDays: vi.fn(), setAutoRotate: vi.fn(), setRotateInterval: vi.fn(), setRotateMode: vi.fn(), setAutoStart: vi.fn(), setCacheRetentionDays: vi.fn(), revealLog: vi.fn() },
  events: { listen: vi.fn() },
} }));
vi.mock("../shared/platform/autostart", () => ({ getAutostartEnabled: vi.fn().mockResolvedValue(false) }));

test("renders the Pavo application shell", async () => {
  render(<App />);
  expect(screen.getByRole("application", { name: "Pavo" })).toBeInTheDocument();
  expect(await screen.findByText("今天还没有可用的壁纸。")).toBeInTheDocument();
});

test("switches between the archive and favorites folios", async () => {
  const user = userEvent.setup();
  render(<App />);
  await user.click(screen.getByRole("tab", { name: "历史" }));
  expect(await screen.findByRole("heading", { name: "历史档案" })).toBeInTheDocument();
  await user.click(screen.getByRole("tab", { name: "收藏" }));
  expect(await screen.findByRole("heading", { name: "收藏册页" })).toBeInTheDocument();
});

test("opens settings outside the primary tabs and returns to today", async () => {
  const user = userEvent.setup();
  render(<App />);
  await user.click(screen.getByRole("button", { name: "设置" }));
  expect(await screen.findByRole("heading", { name: "设置" })).toBeInTheDocument();
  expect(screen.queryByRole("tablist")).not.toBeInTheDocument();
  await user.click(screen.getByRole("button", { name: "返回今日" }));
  expect(screen.getByRole("tab", { name: "今日" })).toHaveAttribute("aria-selected", "true");
});
