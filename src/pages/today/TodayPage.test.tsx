import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect, test, vi } from "vitest";
import { ToastProvider } from "../../shared/ui/toast/ToastProvider";
import { tauri } from "../../shared/tauri/client";
import { TodayPage } from "./TodayPage";

vi.mock("../../shared/tauri/client", () => ({
  tauri: { wallpapers: { getTodayCollection: vi.fn(), getRecent: vi.fn(), setAsDesktop: vi.fn(), download: vi.fn() } },
}));
vi.mock("../../shared/platform/shell", () => ({ openExternal: vi.fn() }));

const photo = (filename: string, title: string, region = "zh-CN") => ({
  filename, regions: [region], urls: [`https://example.test/${filename}`], titles: [title],
  startdates: ["20260723"], copyrights: [`${title}的介绍`], copyrightlinks: ["https://example.test/source"],
});

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(tauri.wallpapers.setAsDesktop).mockResolvedValue("ok");
  vi.mocked(tauri.wallpapers.download).mockResolvedValue("ok");
});

test("renders the daily folio and sets the selected wallpaper", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([
    photo("cn.jpg", "山谷"), photo("us.jpg", "海岸", "en-US"),
  ]);
  const user = userEvent.setup();
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await screen.findByRole("heading", { name: "山谷" });
  await user.click(screen.getByRole("button", { name: "设为桌面" }));
  expect(tauri.wallpapers.setAsDesktop).toHaveBeenCalledWith("https://example.test/cn.jpg");
  expect(screen.getByRole("button", { name: "美国" })).toBeVisible();
});

test("falls back to recent wallpapers when todays collection is insufficient", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([photo("only.jpg", "唯一")]);
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([photo("recent.jpg", "近期")]);
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await waitFor(() => expect(tauri.wallpapers.getRecent).toHaveBeenCalledWith(7));
  expect(await screen.findByText("近期内容")).toBeVisible();
});
