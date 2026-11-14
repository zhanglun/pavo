import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { tauri } from "../../shared/tauri/client";
import { ToastProvider } from "../../shared/ui/toast/ToastProvider";
import { HistoryPage } from "./HistoryPage";

vi.mock("../../shared/tauri/client", () => ({ tauri: {
  settings: { get: vi.fn() },
  wallpapers: { getRecent: vi.fn(), setAsDesktop: vi.fn(), download: vi.fn(), listMonitors: vi.fn() },
} }));
vi.mock("../../shared/platform/shell", () => ({ openExternal: vi.fn() }));

const photo = { filename: "mountain.jpg", regions: ["中国"], urls: ["https://example.com/a.jpg"], titles: ["山谷"], startdates: ["20260723"], copyrights: ["摄影者"], copyrightlinks: ["https://example.com/source"] };

beforeEach(() => {
  vi.clearAllMocks();
  // fixture 的 startdate 固定为 20260723；冻结系统日期使历史窗口判定与 fixture 对齐
  vi.useFakeTimers({ now: new Date("2026-07-23T00:00:00"), shouldAdvanceTime: true });
  vi.mocked(tauri.settings.get).mockResolvedValue({ history_range_days: 14 } as never);
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([photo]);
  vi.mocked(tauri.wallpapers.listMonitors).mockResolvedValue({ per_monitor: false, monitors: [] });
});

afterEach(() => {
  vi.useRealTimers();
});

test("loads the configured archive range and keeps actions behind the quiet menu", async () => {
  render(<ToastProvider><HistoryPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  expect(await screen.findByRole("heading", { name: "历史档案" })).toBeInTheDocument();
  expect(tauri.wallpapers.getRecent).toHaveBeenCalledWith(14);
  expect(screen.getByText("山谷")).toBeInTheDocument();
  expect(screen.queryByText("下载原图")).not.toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "更多操作：山谷" }));
  expect(screen.getByRole("menuitem", { name: "设为桌面" })).toBeInTheDocument();
  expect(screen.getByRole("menuitem", { name: "下载原图" })).toBeInTheDocument();
  expect(screen.getByRole("menuitem", { name: "在 Bing 中查看 ↗" })).toBeInTheDocument();
});

test("shows a restrained retry state when history cannot load", async () => {
  vi.mocked(tauri.wallpapers.getRecent).mockRejectedValue(new Error("offline"));
  render(<ToastProvider><HistoryPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await waitFor(() => expect(screen.getByText(/暂时无法读取/)).toBeInTheDocument());
});

test("offers per-monitor targets in the quiet menu when the system supports it", async () => {
  vi.mocked(tauri.wallpapers.listMonitors).mockResolvedValue({
    per_monitor: true,
    monitors: [
      { id: "m-1", name: "DELL", index: 0, width: 3840, height: 2160, is_primary: true, current_filename: null },
      { id: "m-2", name: "BenQ", index: 1, width: 1920, height: 1080, is_primary: false, current_filename: null },
    ],
  });

  render(<ToastProvider><HistoryPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  fireEvent.click(await screen.findByRole("button", { name: "更多操作：山谷" }));

  // 快照在菜单打开后异步到达
  expect(await screen.findByText("设为指定屏幕")).toBeInTheDocument();
  expect(screen.getByRole("menuitem", { name: "DELL · 3840×2160 · 主" })).toBeInTheDocument();
  expect(screen.getByRole("menuitem", { name: "BenQ · 1920×1080" })).toBeInTheDocument();
});

test("renders one archive card for regional variants of the same visual", async () => {
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([{
    filename: "OHR.Shared_ZH-CN.jpg",
    regions: ["zh-CN", "en-US"],
    urls: ["https://example.com/shared-cn.jpg", "https://example.com/shared-us.jpg"],
    titles: ["中国共享", "美国共享"],
    startdates: ["20260723", "20260723"],
    copyrights: ["摄影者", "Photographer"],
    copyrightlinks: ["source-cn", "source-us"],
  }]);

  render(<ToastProvider><HistoryPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);

  expect(await screen.findByText("中国共享")).toBeVisible();
  expect(screen.queryByText("美国共享")).not.toBeInTheDocument();
});
