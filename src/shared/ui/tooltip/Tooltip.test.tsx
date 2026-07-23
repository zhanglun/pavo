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

  test("cancels a pending tooltip when it becomes disabled", () => {
    vi.useFakeTimers();
    const { rerender } = render(<Tooltip label="更多操作"><button>⋯</button></Tooltip>);
    const trigger = screen.getByRole("button");

    fireEvent.mouseEnter(trigger);
    rerender(<Tooltip disabled label="更多操作"><button>⋯</button></Tooltip>);
    act(() => vi.advanceTimersByTime(800));

    expect(screen.queryByRole("tooltip")).not.toBeInTheDocument();
    expect(trigger).not.toHaveAttribute("aria-describedby");
  });

  test("exposes edge alignment for viewport-safe positioning", () => {
    render(<Tooltip align="end" label="隐藏到托盘"><button>−</button></Tooltip>);
    fireEvent.focus(screen.getByRole("button"));

    expect(screen.getByRole("tooltip")).toHaveAttribute("data-align", "end");
  });
});
