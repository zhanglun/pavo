import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect, test, vi } from "vitest";
import { tauri } from "../../../shared/tauri/client";
import { ToastProvider } from "../../../shared/ui/toast/ToastProvider";
import { ToastViewport } from "../../../shared/ui/toast/ToastViewport";
import { useSetWallpaper } from "./useSetWallpaper";

type ProgressPayload = { operationId: string; phase: "downloading" | "applying"; percent?: number };
let progressListener: ((event: { payload: ProgressPayload }) => void) | undefined;

vi.mock("../../../shared/tauri/client", () => ({
  tauri: {
    wallpapers: { setAsDesktop: vi.fn() },
    events: {
      listen: vi.fn((_name: string, listener: (event: { payload: ProgressPayload }) => void) => {
        progressListener = listener;
        return Promise.resolve(() => undefined);
      }),
    },
  },
}));

function Trigger() {
  const wallpaper = useSetWallpaper();
  const label = wallpaper.phase === "downloading"
    ? `下载原图 ${wallpaper.percent}%`
    : wallpaper.pending
      ? "正在应用…"
      : "设为桌面";
  return <button disabled={wallpaper.pending} onClick={() => void wallpaper.setWallpaper("wallpaper-url")}>{label}</button>;
}

beforeEach(() => {
  progressListener = undefined;
  vi.clearAllMocks();
});

test("reports download percentage in the action before applying", async () => {
  let finish: () => void = () => undefined;
  vi.mocked(tauri.wallpapers.setAsDesktop).mockImplementation(() => new Promise((resolve) => { finish = () => resolve({ confirmed: true }); }));
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "设为桌面" }));

  await waitFor(() => expect(tauri.events.listen).toHaveBeenCalledWith("wallpaper:set-progress", expect.any(Function)));
  const operationId = vi.mocked(tauri.wallpapers.setAsDesktop).mock.calls[0][1];
  act(() => progressListener?.({ payload: { operationId, phase: "downloading", percent: 42 } }));
  expect(await screen.findByRole("button", { name: "下载原图 42%" })).toBeDisabled();
  act(() => progressListener?.({ payload: { operationId, phase: "applying" } }));
  expect(await screen.findByRole("button", { name: "正在应用…" })).toBeDisabled();
  act(() => finish());
  await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("壁纸已设置"));
});

test("does not report success when the system has not confirmed the wallpaper", async () => {
  vi.mocked(tauri.wallpapers.setAsDesktop).mockResolvedValue({ confirmed: false });
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "设为桌面" }));

  expect(await screen.findByRole("status")).toHaveTextContent("已交给系统，桌面仍在刷新");
  expect(screen.queryByText("壁纸已设置")).not.toBeInTheDocument();
});
