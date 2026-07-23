import { cloneElement, useEffect, useId, useRef, useState, type ReactElement } from "react";
import styles from "./Tooltip.module.css";

type Props = {
  children: ReactElement;
  disabled?: boolean;
  label: string;
  side?: "top" | "bottom";
};

export function Tooltip({ children, disabled = false, label, side = "top" }: Props) {
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
      {cloneElement(children, { "aria-describedby": open ? id : undefined } as object)}
      {open && !disabled && <span className={styles.tooltip} data-side={side} id={id} role="tooltip">{label}</span>}
    </span>
  );
}
