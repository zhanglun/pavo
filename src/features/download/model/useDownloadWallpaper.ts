import { useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import { useToast } from "../../../shared/ui/toast/ToastProvider";
export function useDownloadWallpaper() { const [pending, setPending] = useState(false); const { show } = useToast(); const download = async (url: string) => { setPending(true); try { await tauri.wallpapers.download(url); show({ tone: "success", message: "下载完成" }); } catch { show({ tone: "error", message: "下载失败，请重试" }); } finally { setPending(false); } }; return { pending, download }; }
