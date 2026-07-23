import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi, test, expect } from "vitest";
import { Menu } from "./Menu";

test("closes on Escape and restores focus", async () => {
  const user = userEvent.setup();
  render(<Menu label="更多操作" items={[{ id: "download", label: "下载原图", onSelect: vi.fn() }]} />);
  const trigger = screen.getByRole("button", { name: "更多操作" });
  await user.click(trigger);
  await user.keyboard("{Escape}");
  expect(screen.queryByRole("menu")).not.toBeInTheDocument();
  expect(trigger).toHaveFocus();
});
