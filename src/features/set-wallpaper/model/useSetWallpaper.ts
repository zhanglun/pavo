import { useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import { useToast } from "../../../shared/ui/toast/ToastProvider";
export function useSetWallpaper() { const [pending, setPending] = useState(false); const { show } = useToast(); const setWallpaper = async (url: string) => { setPending(true); try { await tauri.wallpapers.setAsDesktop(url); show({ tone: "success", message: "壁纸已设置" }); } catch { show({ tone: "error", message: "设置失败，请重试" }); } finally { setPending(false); } }; return { pending, setWallpaper }; }
