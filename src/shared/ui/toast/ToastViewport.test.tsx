import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";
import { ToastProvider, useToast } from "./ToastProvider";
import { ToastViewport } from "./ToastViewport";

function Trigger() {
  const { show } = useToast();
  return <button onClick={() => show({ tone: "success", message: "壁纸已设置" })}>显示提示</button>;
}

test("renders notifications in a styled live region outside page flow", async () => {
  const user = userEvent.setup();
  render(<ToastProvider><Trigger /><ToastViewport /></ToastProvider>);

  await user.click(screen.getByRole("button", { name: "显示提示" }));

  const status = screen.getByRole("status");
  expect(status).toHaveTextContent("壁纸已设置");
  expect(status).toHaveAttribute("data-tone", "success");
  expect(status.className).not.toBe("");
});
