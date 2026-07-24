import { useEffect, useRef, useState } from "react";
import { MoreHorizontal } from "lucide-react";
import { Tooltip } from "../tooltip/Tooltip";
import styles from "./Menu.module.css";
type Item = { id: string; label: string; onSelect: () => void; disabled?: boolean };
export function Menu({ label, items }: { label: string; items: Item[] }) {
  const [open, setOpen] = useState(false);
  const wrapper = useRef<HTMLSpanElement>(null);
  const trigger = useRef<HTMLButtonElement>(null);
  useEffect(() => {
    const close = (event: KeyboardEvent) => { if (event.key === "Escape") { setOpen(false); trigger.current?.focus(); } };
    const outside = (event: MouseEvent) => { if (!wrapper.current?.contains(event.target as Node)) setOpen(false); };
    window.addEventListener("keydown", close); window.addEventListener("mousedown", outside);
    return () => { window.removeEventListener("keydown", close); window.removeEventListener("mousedown", outside); };
  }, []);
  const moveFocus = (event: React.KeyboardEvent<HTMLSpanElement>) => {
    if (!event.key.startsWith("Arrow")) return;
    const buttons = Array.from(event.currentTarget.querySelectorAll<HTMLButtonElement>("[role='menuitem']:not(:disabled)"));
    const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
    const direction = event.key === "ArrowDown" ? 1 : -1;
    buttons[(index + direction + buttons.length) % buttons.length]?.focus();
    event.preventDefault();
  };
  return <span className={styles.root} ref={wrapper}><Tooltip align="end" disabled={open} label={label}><button className={styles.trigger} ref={trigger} aria-label={label} aria-haspopup="menu" aria-expanded={open} onClick={() => setOpen(!open)}><MoreHorizontal size={15} strokeWidth={1.75} /></button></Tooltip>{open && <span className={styles.menu} role="menu" onKeyDown={moveFocus}>{items.map((item) => <button className={styles.item} key={item.id} role="menuitem" disabled={item.disabled} onClick={() => { setOpen(false); item.onSelect(); }}>{item.label}</button>)}</span>}</span>;
}
