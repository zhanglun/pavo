import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi } from "vitest";
import { App } from "./App";

vi.mock("../shared/tauri/client", () => ({ tauri: {
  favorites: { list: vi.fn().mockResolvedValue([]), add: vi.fn(), remove: vi.fn() },
  wallpapers: { getTodayCollection: vi.fn().mockResolvedValue([]), getRecent: vi.fn().mockResolvedValue([]), forceRefresh: vi.fn(), setAsDesktop: vi.fn(), download: vi.fn() },
  settings: { get: vi.fn().mockResolvedValue({ history_range_days: 7 }) },
  events: { listen: vi.fn() },
} }));

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
