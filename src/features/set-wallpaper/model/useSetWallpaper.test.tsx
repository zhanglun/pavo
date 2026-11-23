import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect, test, vi } from "vitest";
import { tauri } from "../../../shared/tauri/client";
import { ToastProvider } from "../../../shared/ui/toast/ToastProvider";
import { ToastViewport } from "../../../shared/ui/toast/ToastViewport";
import { useSetWallpaper } from "./useSetWallpaper";

vi.mock("../../../shared/tauri/client", () => ({ tauri: { wallpapers: { setAsDesktop: vi.fn() } } }));

// 各测试隔离：默认成功，避免前序测试的手动 promise 实现泄漏
beforeEach(() => {
  vi.mocked(tauri.wallpapers.setAsDesktop).mockResolvedValue("ok");
});

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

test("names the target screen in toasts when setting per monitor", async () => {
  const user = userEvent.setup();
  function Trigger() {
    const wallpaper = useSetWallpaper();
    return <button onClick={() => void wallpaper.setWallpaper("wallpaper-url", { id: "m-2", name: "屏幕 2" })}>设置</button>;
  }
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "设置" }));

  expect(tauri.wallpapers.setAsDesktop).toHaveBeenCalledWith("wallpaper-url", "m-2");
  await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("已设置到「屏幕 2」"));
});

test("surfaces backend errors like a disconnected monitor verbatim", async () => {
  vi.mocked(tauri.wallpapers.setAsDesktop).mockRejectedValue("显示器不存在或已断开");
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "设置" }));

  await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("显示器不存在或已断开"));
});
