import { useCallback, useState } from "react";
import type { MonitorSnapshot } from "../../../entities/monitor/model/types";
import { tauri } from "../../../shared/tauri/client";

/** 屏幕快照的惰性加载：在打开屏幕选择菜单 / 设置页挂载时拉取，热插拔后即为最新。 */
export function useMonitors() {
  const [snapshot, setSnapshot] = useState<MonitorSnapshot | null>(null);
  const load = useCallback(async () => {
    try {
      setSnapshot(await tauri.wallpapers.listMonitors());
    } catch {
      // 能力查询失败时保持 null，UI 隐藏逐屏入口（避免死路径）
      setSnapshot(null);
    }
  }, []);
  return { snapshot, load };
}
