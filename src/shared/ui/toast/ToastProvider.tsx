import { createContext, useContext, useState, type ReactNode } from "react";

type Toast = { tone: "success" | "error"; message: string } | null;
const ToastContext = createContext<{ toast: Toast; show: (toast: NonNullable<Toast>) => void } | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toast, setToast] = useState<Toast>(null);
  const show = (value: NonNullable<Toast>) => {
    setToast(value);
    window.setTimeout(() => setToast(null), 2400);
  };
  return <ToastContext.Provider value={{ toast, show }}>{children}</ToastContext.Provider>;
}

export function useToast() {
  const value = useContext(ToastContext);
  if (!value) throw new Error("ToastProvider is required");
  return value;
}
