import { fireEvent, render, screen } from "@testing-library/react";
import { act } from "react";
import { afterEach, describe, expect, test, vi } from "vitest";
import { Tooltip } from "./Tooltip";

describe("Tooltip", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  test("shows immediately on keyboard focus and links the trigger to the tooltip", () => {
    render(<Tooltip label="刷新"><button>↻</button></Tooltip>);

    const trigger = screen.getByRole("button");
    fireEvent.focus(trigger);

    const tooltip = screen.getByRole("tooltip");
    expect(tooltip).toHaveTextContent("刷新");
    expect(trigger).toHaveAttribute("aria-describedby", tooltip.id);
  });

  test("waits before showing on hover and closes with Escape", () => {
    vi.useFakeTimers();
    render(<Tooltip label="设置"><button>⚙</button></Tooltip>);

    const trigger = screen.getByRole("button");
    fireEvent.mouseEnter(trigger);
    expect(screen.queryByRole("tooltip")).not.toBeInTheDocument();

    act(() => vi.advanceTimersByTime(800));
    expect(screen.getByRole("tooltip")).toBeInTheDocument();

    fireEvent.keyDown(trigger, { key: "Escape" });
    expect(screen.queryByRole("tooltip")).not.toBeInTheDocument();
  });
});
