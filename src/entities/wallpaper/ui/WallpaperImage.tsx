import { useState } from "react";
import type { Wallpaper } from "../model/types";
export function WallpaperImage({ wallpaper }: { wallpaper: Wallpaper }) { const [failed, setFailed] = useState(false); return failed ? <div role="img" aria-label="图片暂不可用">图片暂不可用</div> : <img src={wallpaper.imageUrl} alt={wallpaper.title} onError={() => setFailed(true)} />; }
