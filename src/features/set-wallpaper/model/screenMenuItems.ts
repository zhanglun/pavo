import type { MonitorInfo, MonitorSnapshot } from "../../../entities/monitor/model/types";
import type { WallpaperTarget } from "./useSetWallpaper";

export type ScreenMenuItem = {
  id: string;
  label: string;
  group?: string;
  disabled?: boolean;
  onSelect: () => void;
};

function monitorLabel(monitor: MonitorInfo, index: number): string {
  const name = monitor.name || `显示器 ${index + 1}`;
  const parts = [name, `${monitor.width}×${monitor.height}`];
  if (monitor.is_primary) parts.push("主");
  return parts.join(" · ");
}

/** 由枚举快照生成"设为指定屏幕"菜单项；能力不可用或单屏时返回空（单屏与"所有屏幕"等价，避免困惑）。 */
export function buildScreenMenuItems(
  snapshot: MonitorSnapshot | null,
  pending: boolean,
  onPick: (target: WallpaperTarget) => void,
): ScreenMenuItem[] {
  if (!snapshot?.per_monitor || snapshot.monitors.length <= 1) return [];
  return snapshot.monitors.map((monitor, index) => {
    const name = monitorLabel(monitor, index);
    return {
      id: `set-screen-${monitor.id}`,
      label: name,
      group: "设为指定屏幕",
      disabled: pending,
      onSelect: () => onPick({ id: monitor.id, name }),
    };
  });
}
