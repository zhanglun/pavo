import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { tauri } from "../../shared/tauri/client";
import { ToastProvider } from "../../shared/ui/toast/ToastProvider";
import { HistoryPage } from "./HistoryPage";

vi.mock("../../shared/tauri/client", () => ({ tauri: {
  settings: { get: vi.fn() },
  wallpapers: { getRecent: vi.fn(), setAsDesktop: vi.fn(), download: vi.fn() },
} }));
vi.mock("../../shared/platform/shell", () => ({ openExternal: vi.fn() }));

const photo = { filename: "mountain.jpg", regions: ["中国"], urls: ["https://example.com/a.jpg"], titles: ["山谷"], startdates: ["20260723"], copyrights: ["摄影者"], copyrightlinks: ["https://example.com/source"] };

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(tauri.settings.get).mockResolvedValue({ history_range_days: 14 } as never);
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([photo]);
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
  expect(screen.getByRole("menuitem", { name: "介绍与来源 ↗" })).toBeInTheDocument();
});

test("shows a restrained retry state when history cannot load", async () => {
  vi.mocked(tauri.wallpapers.getRecent).mockRejectedValue(new Error("offline"));
  render(<ToastProvider><HistoryPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await waitFor(() => expect(screen.getByText(/暂时无法读取/)).toBeInTheDocument());
});
