import { useCallback, useEffect, useLayoutEffect, useRef, useState, type CSSProperties } from "react";
import { createPortal } from "react-dom";
import { MoreHorizontal } from "lucide-react";
import { Tooltip } from "../tooltip/Tooltip";
import styles from "./Menu.module.css";

type Item = { id: string; label: string; onSelect: () => void; disabled?: boolean };
type Rect = { left: number; right: number; top: number; bottom: number; width: number; height: number };
type Size = { width: number; height: number };
type MenuPosition = { left: number; top: number; side: "top" | "bottom"; arrowLeft: number };

const GAP = 8;
const VIEWPORT_MARGIN = 8;
const ARROW_MARGIN = 12;

export function getMenuPosition(trigger: Rect, menu: Size, viewport: Size): MenuPosition {
  const maxLeft = Math.max(VIEWPORT_MARGIN, viewport.width - menu.width - VIEWPORT_MARGIN);
  const left = Math.min(Math.max(trigger.right - menu.width, VIEWPORT_MARGIN), maxLeft);
  const roomAbove = trigger.top - VIEWPORT_MARGIN;
  const roomBelow = viewport.height - trigger.bottom - VIEWPORT_MARGIN;
  const side = roomAbove >= menu.height + GAP || roomAbove >= roomBelow ? "top" : "bottom";
  const preferredTop = side === "top" ? trigger.top - menu.height - GAP : trigger.bottom + GAP;
  const maxTop = Math.max(VIEWPORT_MARGIN, viewport.height - menu.height - VIEWPORT_MARGIN);
  const top = Math.min(Math.max(preferredTop, VIEWPORT_MARGIN), maxTop);
  const triggerCenter = trigger.left + trigger.width / 2;
  const arrowLeft = Math.min(Math.max(triggerCenter - left, ARROW_MARGIN), menu.width - ARROW_MARGIN);
  return { left, top, side, arrowLeft };
}

export function Menu({ label, items }: { label: string; items: Item[] }) {
  const [open, setOpen] = useState(false);
  const [position, setPosition] = useState<MenuPosition | null>(null);
  const wrapper = useRef<HTMLSpanElement>(null);
  const trigger = useRef<HTMLButtonElement>(null);
  const menu = useRef<HTMLSpanElement>(null);

  const updatePosition = useCallback(() => {
    if (!trigger.current || !menu.current) return;
    setPosition(getMenuPosition(
      trigger.current.getBoundingClientRect(),
      menu.current.getBoundingClientRect(),
      { width: window.innerWidth, height: window.innerHeight },
    ));
  }, []);

  useLayoutEffect(() => {
    if (!open) {
      setPosition(null);
      return;
    }
    updatePosition();
  }, [open, updatePosition]);

  useEffect(() => {
    const close = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
        trigger.current?.focus();
      }
    };
    const outside = (event: MouseEvent) => {
      const target = event.target as Node;
      if (!wrapper.current?.contains(target) && !menu.current?.contains(target)) setOpen(false);
    };
    const reposition = () => updatePosition();
    window.addEventListener("keydown", close);
    window.addEventListener("mousedown", outside);
    window.addEventListener("resize", reposition);
    window.addEventListener("scroll", reposition, true);
    return () => {
      window.removeEventListener("keydown", close);
      window.removeEventListener("mousedown", outside);
      window.removeEventListener("resize", reposition);
      window.removeEventListener("scroll", reposition, true);
    };
  }, [updatePosition]);

  const moveFocus = (event: React.KeyboardEvent<HTMLSpanElement>) => {
    if (!event.key.startsWith("Arrow")) return;
    const buttons = Array.from(event.currentTarget.querySelectorAll<HTMLButtonElement>("[role='menuitem']:not(:disabled)"));
    const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
    const direction = event.key === "ArrowDown" ? 1 : -1;
    buttons[(index + direction + buttons.length) % buttons.length]?.focus();
    event.preventDefault();
  };

  const menuStyle = position ? {
    left: position.left,
    top: position.top,
    visibility: "visible",
    "--menu-arrow-left": `${position.arrowLeft}px`,
  } as CSSProperties : undefined;

  return <>
    <span className={styles.root} ref={wrapper}>
      <Tooltip align="end" disabled={open} label={label}>
        <button className={styles.trigger} ref={trigger} aria-label={label} aria-haspopup="menu" aria-expanded={open} onClick={() => setOpen(!open)}>
          <MoreHorizontal size={15} strokeWidth={1.75} />
        </button>
      </Tooltip>
    </span>
    {open && createPortal(
      <span
        className={styles.menu}
        ref={menu}
        role="menu"
        data-side={position?.side}
        style={menuStyle}
        onKeyDown={moveFocus}
      >
        {items.map((item) => <button className={styles.item} key={item.id} role="menuitem" disabled={item.disabled} onClick={() => { setOpen(false); item.onSelect(); }}>{item.label}</button>)}
      </span>,
      document.body,
    )}
  </>;
}
