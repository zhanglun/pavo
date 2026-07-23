import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import type { Wallpaper } from "../../../entities/wallpaper/model/types";
import { tauri } from "../../../shared/tauri/client";
import { useFavorites } from "./useFavorites";

vi.mock("../../../shared/tauri/client", () => ({
  tauri: { favorites: { list: vi.fn(), add: vi.fn(), remove: vi.fn() } },
}));

const wallpaper: Wallpaper = {
  id: "mountain-cn",
  filename: "mountain.jpg",
  regionCode: "zh-CN",
  region: "中国",
  imageUrl: "https://example.com/mountain.jpg",
  title: "山谷",
  date: "20260723",
  copyright: "摄影者",
  sourceUrl: "https://example.com/source",
};

beforeEach(() => vi.clearAllMocks());

const savedFavorite = {
  filename: wallpaper.filename,
  url: wallpaper.imageUrl,
  title: wallpaper.title,
  startdate: wallpaper.date,
  copyright: wallpaper.copyright,
  copyrightlink: wallpaper.sourceUrl,
};

test("loads persisted favorites by URL and adds a wallpaper using the Rust model", async () => {
  vi.mocked(tauri.favorites.list).mockResolvedValue([savedFavorite]);
  vi.mocked(tauri.favorites.add).mockResolvedValue({} as never);
  const { result } = renderHook(() => useFavorites());
  await waitFor(() => expect(result.current.favoriteIds.has(wallpaper.imageUrl)).toBe(true));
  await act(() => result.current.toggle({ ...wallpaper, filename: "lake.jpg", id: "lake-cn", imageUrl: "https://example.com/lake.jpg" }));
  expect(tauri.favorites.add).toHaveBeenCalledWith(expect.objectContaining({ filename: "lake.jpg", url: "https://example.com/lake.jpg" }));
});

test("removes an existing favorite", async () => {
  vi.mocked(tauri.favorites.list).mockResolvedValue([savedFavorite]);
  vi.mocked(tauri.favorites.remove).mockResolvedValue({} as never);
  const { result } = renderHook(() => useFavorites());
  await waitFor(() => expect(result.current.favoriteIds.has(wallpaper.imageUrl)).toBe(true));
  await act(() => result.current.toggle(wallpaper));
  expect(tauri.favorites.remove).toHaveBeenCalledWith(wallpaper.imageUrl);
  expect(result.current.favoriteIds.has(wallpaper.imageUrl)).toBe(false);
});
