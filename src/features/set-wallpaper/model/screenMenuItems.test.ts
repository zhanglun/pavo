import { expect, test, vi } from "vitest";
import type { MonitorInfo, MonitorSnapshot } from "../../../entities/monitor/model/types";
import { buildScreenMenuItems } from "./screenMenuItems";

const makeSnapshot = (per_monitor: boolean, monitors: MonitorInfo[]): MonitorSnapshot => ({ per_monitor, monitors });

test("returns no items when the system does not support per-monitor wallpapers", () => {
  const items = buildScreenMenuItems({ per_monitor: false, monitors: [] }, false, vi.fn());
  expect(items).toEqual([]);
});

test("returns no items when no monitors are connected", () => {
  const items = buildScreenMenuItems({ per_monitor: true, monitors: [] }, false, vi.fn());
  expect(items).toEqual([]);
});

test("returns no items on a single monitor (equivalent to all screens)", () => {
  const items = buildScreenMenuItems(
    makeSnapshot(true, [{ id: "m-1", name: "A", index: 0, width: 100, height: 100, is_primary: false, current_filename: null }]),
    false,
    vi.fn(),
  );
  expect(items).toEqual([]);
});

test("builds grouped items with resolution, primary marker and stable ids", () => {
  const snapshot = makeSnapshot(true, [
    { id: "m-1", name: "DELL U2723QE", index: 0, width: 3840, height: 2160, is_primary: true, current_filename: null },
    { id: "m-2", name: "", index: 1, width: 1920, height: 1080, is_primary: false, current_filename: null },
  ]);
  const onPick = vi.fn();
  const items = buildScreenMenuItems(snapshot, false, onPick);

  expect(items).toHaveLength(2);
  expect(items[0]).toMatchObject({ id: "set-screen-m-1", label: "DELL U2723QE · 3840×2160 · 主", group: "设为指定屏幕" });
  // 无 EDID 名称时回落为"显示器 N"
  expect(items[1].label).toBe("显示器 2 · 1920×1080");

  items[0].onSelect();
  expect(onPick).toHaveBeenCalledWith({ id: "m-1", name: "DELL U2723QE · 3840×2160 · 主" });
});

test("marks items disabled while a set operation is pending", () => {
  const snapshot = makeSnapshot(true, [
    { id: "m-1", name: "A", index: 0, width: 100, height: 100, is_primary: false, current_filename: null },
  ]);
  const items = buildScreenMenuItems(snapshot, true, vi.fn());
  expect(items.every((item) => item.disabled)).toBe(true);
});
