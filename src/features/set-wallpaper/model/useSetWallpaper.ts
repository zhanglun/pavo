import { useEffect, useRef, useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import { useToast } from "../../../shared/ui/toast/ToastProvider";

type SetWallpaperPhase = "idle" | "preparing" | "downloading" | "applying";
type SetWallpaperProgress = {
  operationId: string;
  phase: "downloading" | "applying";
  percent?: number;
};

export function useSetWallpaper() {
  const [phase, setPhase] = useState<SetWallpaperPhase>("idle");
  const [percent, setPercent] = useState(0);
  const operationIdRef = useRef("");
  const { show } = useToast();

  useEffect(() => {
    let cleanup: (() => void) | undefined;
    const listen = tauri.events?.listen;
    if (!listen) return;
    void Promise.resolve(listen<SetWallpaperProgress>("wallpaper:set-progress", ({ payload }) => {
      if (payload.operationId !== operationIdRef.current) return;
      setPhase(payload.phase);
      if (payload.phase === "downloading") setPercent(payload.percent ?? 0);
    })).then((unlisten) => { cleanup = unlisten; });
    return () => cleanup?.();
  }, []);

  const setWallpaper = async (url: string) => {
    const operationId = crypto.randomUUID();
    operationIdRef.current = operationId;
    setPercent(0);
    setPhase("preparing");
    try {
      const result = await tauri.wallpapers.setAsDesktop(url, operationId);
      if (result.confirmed) {
        show({ tone: "success", message: "壁纸已设置", durationMs: 3000 });
      } else {
        show({ tone: "info", message: "已交给系统，桌面仍在刷新", durationMs: 4000 });
      }
    } catch {
      show({ tone: "error", message: "设置失败，请重试" });
    } finally {
      operationIdRef.current = "";
      setPhase("idle");
    }
  };

  return { pending: phase !== "idle", phase, percent, setWallpaper };
}
