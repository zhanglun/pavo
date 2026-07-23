import { fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { act } from "react";
import { afterEach, expect, test, vi } from "vitest";
import { ToastProvider, useToast } from "./ToastProvider";
import { ToastViewport } from "./ToastViewport";

function Trigger() {
  const { show } = useToast();
  return <><button onClick={() => show({ tone: "info", message: "正在设置壁纸…", persistent: true })}>显示提示</button><button onClick={() => show({ tone: "success", message: "壁纸已设置" })}>更新提示</button></>;
}

afterEach(() => vi.useRealTimers());

test("renders notifications in a styled live region outside page flow", async () => {
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "更新提示" }));

  const status = screen.getByRole("status");
  expect(status).toHaveTextContent("壁纸已设置");
  expect(status).toHaveAttribute("data-tone", "success");
  expect(status.className).not.toBe("");
});

test("replaces the previous dismissal timer when a message changes", async () => {
  vi.useFakeTimers();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);
  fireEvent.click(screen.getByRole("button", { name: "显示提示" }));
  act(() => vi.advanceTimersByTime(2000));
  fireEvent.click(screen.getByRole("button", { name: "更新提示" }));

  act(() => vi.advanceTimersByTime(500));
  expect(screen.getByRole("status")).toHaveTextContent("壁纸已设置");
});

test("keeps a persistent progress message visible until it is replaced", () => {
  vi.useFakeTimers();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);
  fireEvent.click(screen.getByRole("button", { name: "显示提示" }));

  act(() => vi.advanceTimersByTime(10_000));

  expect(screen.getByRole("status")).toHaveTextContent("正在设置壁纸…");
});
