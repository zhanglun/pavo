import { ToastProvider } from "../shared/ui/toast/ToastProvider";
import { ToastViewport } from "../shared/ui/toast/ToastViewport";
import { AppShell } from "./AppShell";
export function App() { return <ToastProvider><AppShell /><ToastViewport /></ToastProvider>; }
