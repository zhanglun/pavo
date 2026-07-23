import { ToastProvider } from "../shared/ui/toast/ToastProvider";
import { ToastViewport } from "../shared/ui/toast/ToastViewport";
export function App() { return <ToastProvider><main aria-label="Pavo" role="application" /><ToastViewport /></ToastProvider>; }
