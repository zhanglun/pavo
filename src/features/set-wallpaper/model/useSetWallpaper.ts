import { useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import { useToast } from "../../../shared/ui/toast/ToastProvider";

export type WallpaperTarget = { id: string; name: string } | undefined;

export function useSetWallpaper() {
  const [pending, setPending] = useState(false);
  const { show } = useToast();
  const setWallpaper = async (url: string, monitor?: WallpaperTarget) => {
    setPending(true);
    show({ tone: "info", message: monitor ? `正在设置「${monitor.name}」…` : "正在设置壁纸…", persistent: true });
    try {
      // 缺省时不传第二参，保持"设为所有屏幕"的旧调用形态
      if (monitor) await tauri.wallpapers.setAsDesktop(url, monitor.id);
      else await tauri.wallpapers.setAsDesktop(url);
      show({ tone: "success", message: monitor ? `已设置到「${monitor.name}」` : "壁纸已设置" });
    } catch (error) {
      // 后端错误为中文描述（显示器不存在/系统不支持逐屏等），优先展示
      show({ tone: "error", message: typeof error === "string" ? error : "设置失败，请重试" });
    } finally {
      setPending(false);
    }
  };
  return { pending, setWallpaper };
}
