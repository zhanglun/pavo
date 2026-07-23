import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test, vi } from "vitest";
import { tauri } from "../../../shared/tauri/client";
import { ToastProvider } from "../../../shared/ui/toast/ToastProvider";
import { ToastViewport } from "../../../shared/ui/toast/ToastViewport";
import { useSetWallpaper } from "./useSetWallpaper";

vi.mock("../../../shared/tauri/client", () => ({ tauri: { wallpapers: { setAsDesktop: vi.fn() } } }));

function Trigger() {
  const wallpaper = useSetWallpaper();
  return <button disabled={wallpaper.pending} onClick={() => void wallpaper.setWallpaper("wallpaper-url")}>设置</button>;
}

test("shows progress until setting the wallpaper finishes", async () => {
  let finish: () => void = () => undefined;
  vi.mocked(tauri.wallpapers.setAsDesktop).mockImplementation(() => new Promise((resolve) => { finish = () => resolve("ok"); }));
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "设置" }));

  expect(screen.getByRole("status")).toHaveTextContent("正在设置壁纸…");
  expect(screen.getByRole("button", { name: "设置" })).toBeDisabled();
  finish();
  await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("壁纸已设置"));
});
