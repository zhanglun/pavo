import { useEffect, useState } from "react";
import packageJson from "../../../package.json";
import type { ThemePreference, UserConfig } from "../../entities/settings/model/types";
import { CACHE_RETENTION_DAYS, HISTORY_RANGE_DAYS, ROTATE_INTERVALS } from "../../entities/settings/model/values";
import { getAutostartEnabled } from "../../shared/platform/autostart";
import { openExternal } from "../../shared/platform/shell";
import { checkForUpdates } from "../../shared/platform/updater";
import { tauri } from "../../shared/tauri/client";
import { useToast } from "../../shared/ui/toast/ToastProvider";
import styles from "./SettingsPage.module.css";

const REPOSITORY = "https://github.com/zhanglun/pavo";
const resolveTheme = (value: ThemePreference) => value === "system" ? (window.matchMedia?.("(prefers-color-scheme: dark)").matches ? "dark" : "light") : value;

function ToggleRow({ label, value, onChange }: { label: string; value: boolean; onChange: (value: boolean) => void }) {
  return <button className={styles.toggle} aria-label={label} aria-pressed={value} onClick={() => onChange(!value)}><span>{label}</span><em>{value ? "已开启" : "已关闭"}</em></button>;
}

export function SettingsPage({ onBack, onHistoryRangeChanged }: { onBack: () => void; onHistoryRangeChanged: () => void }) {
  const [config, setConfig] = useState<UserConfig | null>(null);
  const [error, setError] = useState(false);
  const { show } = useToast();

  useEffect(() => {
    let active = true;
    Promise.all([tauri.settings.get(), getAutostartEnabled()]).then(async ([stored, actualAutostart]) => {
      const synced = stored.auto_start === actualAutostart ? stored : await tauri.settings.setAutoStart(actualAutostart);
      if (active) setConfig(synced);
    }).catch(() => { if (active) setError(true); });
    return () => { active = false; };
  }, []);

  const persist = async (request: Promise<UserConfig>, after?: () => void) => {
    try { const next = await request; setConfig(next); after?.(); }
    catch { show({ tone: "error", message: "设置未保存，请重试" }); }
  };
  const runAction = async (request: Promise<unknown>, success?: string) => {
    try { await request; if (success) show({ tone: "success", message: success }); }
    catch { show({ tone: "error", message: "操作失败，请重试" }); }
  };
  const saveTheme = async (preference: ThemePreference) => persist(tauri.settings.setThemePreference(preference), () => {
    localStorage.setItem("pavo.theme-preference", preference);
    document.documentElement.dataset.theme = resolveTheme(preference);
    window.dispatchEvent(new CustomEvent("pavo:theme-preference-changed", { detail: preference }));
  });

  if (error) return <section className={styles.state}>设置暂时无法读取，请稍后重试。</section>;
  if (!config) return <section className={styles.state}>正在读取设置…</section>;
  return <section className={styles.page} aria-labelledby="settings-title">
    <header><button aria-label="返回今日" onClick={onBack}>←</button><div><p>PREFERENCES</p><h1 id="settings-title">设置</h1></div></header>
    <section className={styles.group}><h2>外观</h2><fieldset className={styles.segment}><legend>主题</legend>{([['system','跟随系统'],['light','浅色'],['dark','深色']] as const).map(([value,label]) => <label key={value}><input type="radio" name="theme" checked={config.theme_preference === value} onChange={() => void saveTheme(value)} />{label}</label>)}</fieldset></section>
    <section className={styles.group}><h2>启动与更新</h2><ToggleRow label="开机自动启动" value={config.auto_start} onChange={(value) => void persist(tauri.settings.setAutoStart(value))} /><ToggleRow label="每日自动更新" value={config.auto_daily_update} onChange={(value) => void persist(tauri.settings.setAutoDailyUpdate(value))} /><button className={styles.linkRow} onClick={() => void runAction(checkForUpdates(), "已完成更新检查")}><span>检查更新</span><em>当前 {packageJson.version}</em></button></section>
    <section className={styles.group}><h2>壁纸轮播</h2><ToggleRow label="自动轮播壁纸" value={config.auto_rotate} onChange={(value) => void persist(tauri.settings.setAutoRotate(value))} />{config.auto_rotate && <><label className={styles.selectRow}>切换间隔<select aria-label="切换间隔" value={config.rotate_interval_minutes} onChange={(event) => void persist(tauri.settings.setRotateInterval(Number(event.target.value)))}>{ROTATE_INTERVALS.map((value) => <option key={value} value={value}>{value} 分钟</option>)}</select></label><fieldset className={styles.segment}><legend>轮播顺序</legend>{([['Sequential','顺序'],['Random','随机']] as const).map(([value,label]) => <label key={value}><input type="radio" name="rotate-mode" checked={config.rotate_mode === value} onChange={() => void persist(tauri.settings.setRotateMode(value))} />{label}</label>)}</fieldset></>}</section>
    <section className={styles.group}><h2>资料库与存储</h2><label className={styles.selectRow}>历史范围<select aria-label="历史范围" value={config.history_range_days} onChange={(event) => void persist(tauri.settings.setHistoryRangeDays(Number(event.target.value)), onHistoryRangeChanged)}>{HISTORY_RANGE_DAYS.map((value) => <option key={value} value={value}>{value} 天</option>)}</select></label><label className={styles.selectRow}>缓存保留<select aria-label="缓存保留" value={config.cache_retention_days} onChange={(event) => void persist(tauri.settings.setCacheRetentionDays(Number(event.target.value)))}>{CACHE_RETENTION_DAYS.map((value) => <option key={value} value={value}>{value} 天</option>)}</select></label><button className={styles.linkRow} onClick={() => void runAction(tauri.settings.revealLog())}><span>打开日志</span><em>↗</em></button></section>
    <section className={styles.group}><h2>支持</h2><div className={styles.infoRow}><span>版本</span><em>{packageJson.version}</em></div><button className={styles.linkRow} onClick={() => void openExternal(REPOSITORY)}><span>GitHub</span><em>↗</em></button><button className={styles.linkRow} onClick={() => void openExternal(`${REPOSITORY}/issues/new`)}><span>反馈问题</span><em>↗</em></button></section>
  </section>;
}
