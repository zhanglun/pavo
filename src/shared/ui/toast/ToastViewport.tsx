import { useToast } from "./ToastProvider";
import styles from "./ToastViewport.module.css";
export function ToastViewport() { const { toast } = useToast(); return toast ? <div className={styles.toast} data-tone={toast.tone} role="status">{toast.message}</div> : null; }
