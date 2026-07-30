import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi, test, expect } from "vitest";
import { Menu, getMenuPosition } from "./Menu";

test("closes on Escape and restores focus", async () => {
  const user = userEvent.setup();
  render(<Menu label="更多操作" items={[{ id: "download", label: "下载原图", onSelect: vi.fn() }]} />);
  const trigger = screen.getByRole("button", { name: "更多操作" });
  await user.click(trigger);
  await user.keyboard("{Escape}");
  expect(screen.queryByRole("menu")).not.toBeInTheDocument();
  expect(trigger).toHaveFocus();
});

test("uses an accessible label while keeping the trigger visually quiet", () => {
  render(<Menu label="更多操作：山谷" items={[]} />);
  const trigger = screen.getByRole("button", { name: "更多操作：山谷" });
  // 触发器是图标按钮（无可见文字），accessible name 由 aria-label 提供
  expect(trigger).toHaveAttribute("aria-label", "更多操作：山谷");
  expect(trigger.querySelector("svg")).toBeInTheDocument();
});

test("keeps a menu inside the left viewport edge", () => {
  expect(getMenuPosition(
    { left: 4, right: 36, top: 200, bottom: 232, width: 32, height: 32 },
    { width: 154, height: 96 },
    { width: 320, height: 480 },
  )).toMatchObject({ left: 8, top: 96, side: "top" });
});

test("opens below the trigger when there is not enough room above", () => {
  expect(getMenuPosition(
    { left: 260, right: 292, top: 20, bottom: 52, width: 32, height: 32 },
    { width: 154, height: 96 },
    { width: 320, height: 480 },
  )).toMatchObject({ left: 138, top: 60, side: "bottom" });
});

test("renders the open menu outside its positioned wrapper", async () => {
  const user = userEvent.setup();
  const { container } = render(<Menu label="更多操作" items={[{ id: "download", label: "下载原图", onSelect: vi.fn() }]} />);

  await user.click(screen.getByRole("button", { name: "更多操作" }));

  const menu = screen.getByRole("menu");
  expect(container.contains(menu)).toBe(false);
  expect(document.body.contains(menu)).toBe(true);
});
