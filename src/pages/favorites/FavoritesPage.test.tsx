import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { tauri } from "../../shared/tauri/client";
import { ToastProvider } from "../../shared/ui/toast/ToastProvider";
import { FavoritesPage } from "./FavoritesPage";

vi.mock("../../shared/tauri/client", () => ({ tauri: { favorites: { list: vi.fn() }, wallpapers: { setAsDesktop: vi.fn(), download: vi.fn() } } }));
vi.mock("../../shared/platform/shell", () => ({ openExternal: vi.fn() }));

const favorite = { filename: "lake.jpg", url: "https://example.com/lake.jpg", title: "静湖", startdate: "20260722", copyright: "摄影者", copyrightlink: "https://example.com/source" };

beforeEach(() => { vi.clearAllMocks(); vi.mocked(tauri.favorites.list).mockResolvedValue([favorite]); });

test("renders a two-column folio and removes with a heart instead of visible cancellation copy", async () => {
  const onToggle = vi.fn();
  render(<ToastProvider><FavoritesPage onToggleFavorite={onToggle} refreshSignal={0} /></ToastProvider>);
  expect(await screen.findByRole("heading", { name: "收藏册页" })).toBeInTheDocument();
  expect(screen.queryByText("取消收藏")).not.toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "取消收藏：静湖" }));
  expect(onToggle).toHaveBeenCalledWith(expect.objectContaining({ filename: "lake.jpg", imageUrl: favorite.url }));
});

test("shows an intentional empty state", async () => {
  vi.mocked(tauri.favorites.list).mockResolvedValue([]);
  render(<ToastProvider><FavoritesPage onToggleFavorite={vi.fn()} refreshSignal={0} /></ToastProvider>);
  expect(await screen.findByText("还没有收藏的壁纸")).toBeInTheDocument();
});
