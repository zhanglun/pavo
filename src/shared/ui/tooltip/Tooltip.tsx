import { cloneElement, useEffect, useId, useRef, useState, type ReactElement } from "react";
import styles from "./Tooltip.module.css";

type Props = {
  align?: "start" | "center" | "end";
  children: ReactElement;
  disabled?: boolean;
  label: string;
  side?: "top" | "bottom";
};

export function Tooltip({ align = "center", children, disabled = false, label, side = "top" }: Props) {
  const id = useId();
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const [open, setOpen] = useState(false);

  useEffect(() => () => {
    if (timer.current) clearTimeout(timer.current);
  }, []);

  const cancelTimer = () => {
    if (timer.current) clearTimeout(timer.current);
    timer.current = undefined;
  };
  const hide = () => {
    cancelTimer();
    setOpen(false);
  };
  const showNow = () => {
    cancelTimer();
    if (!disabled) setOpen(true);
  };
  const showLater = () => {
    cancelTimer();
    if (!disabled) timer.current = setTimeout(() => setOpen(true), 800);
  };
  useEffect(() => {
    if (!disabled) return;
    cancelTimer();
    setOpen(false);
  }, [disabled]);

  const visible = open && !disabled;
  const originalDescription = (children.props as { "aria-describedby"?: string })["aria-describedby"];
  const describedBy = [originalDescription, visible ? id : undefined].filter(Boolean).join(" ") || undefined;

  return (
    <span
      className={styles.root}
      data-tooltip-root
      onBlur={hide}
      onFocus={showNow}
      onKeyDown={(event) => { if (event.key === "Escape") hide(); }}
      onMouseEnter={showLater}
      onMouseLeave={hide}
    >
      {cloneElement(children, { "aria-describedby": describedBy } as object)}
      {visible && <span className={styles.tooltip} data-align={align} data-side={side} id={id} role="tooltip">{label}</span>}
    </span>
  );
}
