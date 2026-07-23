import { confirm, message } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { checkForUpdates } from "./updater";

vi.mock("@tauri-apps/plugin-dialog", () => ({ confirm: vi.fn(), message: vi.fn() }));
vi.mock("@tauri-apps/plugin-process", () => ({ relaunch: vi.fn() }));
vi.mock("@tauri-apps/plugin-updater", () => ({ check: vi.fn() }));

describe("checkForUpdates", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("reports when the current version is already latest", async () => {
    vi.mocked(check).mockResolvedValue(null);

    await checkForUpdates();

    expect(message).toHaveBeenCalledWith("当前已是最新版本", { title: "Pavo", kind: "info" });
    expect(relaunch).not.toHaveBeenCalled();
  });

  test("installs a confirmed update and relaunches the application", async () => {
    const downloadAndInstall = vi.fn().mockResolvedValue(undefined);
    vi.mocked(check).mockResolvedValue({ version: "1.2.3", downloadAndInstall } as never);
    vi.mocked(confirm).mockResolvedValue(true);

    await checkForUpdates();

    expect(confirm).toHaveBeenCalledWith("发现新版本 1.2.3，是否现在安装？", { title: "Pavo", kind: "info" });
    expect(downloadAndInstall).toHaveBeenCalledOnce();
    expect(relaunch).toHaveBeenCalledOnce();
  });

  test("does not install an update when the user cancels", async () => {
    const downloadAndInstall = vi.fn();
    vi.mocked(check).mockResolvedValue({ version: "1.2.3", downloadAndInstall } as never);
    vi.mocked(confirm).mockResolvedValue(false);

    await checkForUpdates();

    expect(downloadAndInstall).not.toHaveBeenCalled();
    expect(relaunch).not.toHaveBeenCalled();
  });

  test("reports update failures without leaking a rejected promise", async () => {
    vi.mocked(check).mockRejectedValue(new Error("offline"));

    await expect(checkForUpdates()).resolves.toBeUndefined();
    expect(message).toHaveBeenCalledWith("检查更新失败，请稍后再试", { title: "Pavo", kind: "error" });
  });

  test("shares one update flow when multiple triggers run concurrently", async () => {
    let finishCheck: (value: null) => void = () => undefined;
    vi.mocked(check).mockImplementation(() => new Promise((resolve) => { finishCheck = resolve; }));

    const first = checkForUpdates();
    const second = checkForUpdates();
    expect(check).toHaveBeenCalledOnce();

    finishCheck(null);
    await Promise.all([first, second]);

    expect(message).toHaveBeenCalledOnce();
  });
});
