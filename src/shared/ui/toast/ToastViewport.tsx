import { useToast } from "./ToastProvider";
export function ToastViewport() { const { toast } = useToast(); return toast ? <div role="status">{toast.message}</div> : null; }
