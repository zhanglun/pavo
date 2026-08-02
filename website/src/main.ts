/// <reference types="vite/client" />
import "./styles.css";

type Stats = { downloads?: number; stars?: number };

// 从同源 stats.json 读取下载量/星标数（由 update-stats workflow 每天生成）。
// 客户端不直接调 GitHub API，规避 60/小时 未认证限流。
async function loadStats(): Promise<void> {
  const el = document.querySelector<HTMLElement>("[data-stats]");
  if (!el) return;
  try {
    const res = await fetch(`${import.meta.env.BASE_URL}stats.json`);
    if (!res.ok) return;
    const stats = (await res.json()) as Stats;
    // 占位数据（downloads=0）时不展示，避免显示无意义数字
    if (!stats.downloads || stats.downloads <= 0) return;
    const d = document.getElementById("stat-downloads");
    const s = document.getElementById("stat-stars");
    if (d) d.textContent = stats.downloads.toLocaleString("en-US");
    if (s) s.textContent = String(stats.stars ?? 0);
    el.hidden = false;
  } catch {
    /* 静默失败，不影响页面渲染 */
  }
}

void loadStats();
