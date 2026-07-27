import { createContext, useContext, useEffect, useRef, useState, type ReactNode } from "react";

type Toast = { tone: "info" | "success" | "error"; message: string; persistent?: boolean; durationMs?: number } | null;
const ToastContext = createContext<{ toast: Toast; show: (toast: NonNullable<Toast>) => void } | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toast, setToast] = useState<Toast>(null);
  const timer = useRef<number | undefined>(undefined);
  useEffect(() => () => window.clearTimeout(timer.current), []);
  const show = (value: NonNullable<Toast>) => {
    window.clearTimeout(timer.current);
    setToast(value);
    timer.current = value.persistent ? undefined : window.setTimeout(() => setToast(null), value.durationMs ?? 2400);
  };
  return <ToastContext.Provider value={{ toast, show }}>{children}</ToastContext.Provider>;
}

export function useToast() {
  const value = useContext(ToastContext);
  if (!value) throw new Error("ToastProvider is required");
  return value;
}
