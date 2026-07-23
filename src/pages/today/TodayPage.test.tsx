import { render, screen, waitFor, within } from "@testing-library/react";
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

test("closes the introduction card when another region is selected", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([
    photo("cn.jpg", "山谷"), photo("us.jpg", "海岸", "en-US"),
  ]);
  const user = userEvent.setup();
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await screen.findByRole("heading", { name: "山谷" });

  await user.click(screen.getByRole("button", { name: "查看完整介绍" }));
  const dialog = screen.getByRole("dialog", { name: "山谷完整介绍" });
  expect(dialog).toBeVisible();
  expect(dialog).toHaveFocus();

  await user.keyboard("{Escape}");
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  await waitFor(() => expect(screen.getByRole("button", { name: "查看完整介绍" })).toHaveFocus());

  await user.click(screen.getByRole("button", { name: "查看完整介绍" }));

  await user.click(screen.getByRole("button", { name: "美国" }));

  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "查看完整介绍" })).toHaveAttribute("aria-expanded", "false");
});

test("renders a numbered folio index with count and current-region semantics", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([
    photo("cn.jpg", "山谷"), photo("us.jpg", "海岸", "en-US"),
  ]);
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);

  const china = await screen.findByRole("button", { name: "中国大陆" });
  const unitedStates = screen.getByRole("button", { name: "美国" });

  expect(screen.getByText("2 地区")).toBeVisible();
  expect(china).toHaveAttribute("aria-current", "true");
  expect(unitedStates).not.toHaveAttribute("aria-current");
  expect(within(china).getByText("01")).toBeVisible();
  expect(within(unitedStates).getByText("02")).toBeVisible();
});

test("falls back to recent wallpapers when todays collection is insufficient", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([photo("only.jpg", "唯一")]);
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([photo("recent.jpg", "近期")]);
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  await waitFor(() => expect(tauri.wallpapers.getRecent).toHaveBeenCalledWith(7));
  expect(await screen.findAllByText("近期内容")).toHaveLength(2);
  expect(screen.getByText("1 幅")).toBeVisible();
  expect(screen.queryByText("今日各地")).not.toBeInTheDocument();
});

test("fallback renders each region once with distinct recent images", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([photo("only.jpg", "唯一")]);
  vi.mocked(tauri.wallpapers.getRecent).mockResolvedValue([
    {
      filename: "shared.jpg",
      regions: ["zh-CN", "en-US"],
      urls: ["shared-cn-url", "shared-us-url"],
      titles: ["中国共享", "美国共享"],
      startdates: ["20260723", "20260723"],
      copyrights: ["", ""],
      copyrightlinks: ["", ""],
    },
    photo("us-old.jpg", "美国旧图", "en-US"),
  ]);

  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);

  expect(await screen.findByRole("button", { name: "中国大陆" })).toBeVisible();
  expect(screen.getByRole("button", { name: "美国" }).querySelector("img")).toHaveAttribute("src", "https://example.test/us-old.jpg");
  const thumbnails = screen.getAllByRole("button", { name: /中国大陆|美国/ }).map((button) => button.querySelector("img")?.src);
  expect(new Set(thumbnails).size).toBe(2);
});

test("does not fall back when one merged collection contains multiple valid regions", async () => {
  vi.mocked(tauri.wallpapers.getTodayCollection).mockResolvedValue([{
    filename: "shared.jpg",
    regions: ["zh-CN", "en-US"],
    urls: ["https://example.test/cn.jpg", "https://example.test/us.jpg"],
    titles: ["山谷", "Valley"],
    startdates: ["20260723", "20260723"],
    copyrights: ["介绍", "Copyright"],
    copyrightlinks: ["source-cn", "source-us"],
  }]);
  render(<ToastProvider><TodayPage favoriteIds={new Set()} onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  expect(await screen.findByRole("heading", { name: "山谷" })).toBeVisible();
  expect(tauri.wallpapers.getRecent).not.toHaveBeenCalled();
});
