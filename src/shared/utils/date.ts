export function formatFolioDate(value: string) {
  const year = value.slice(0, 4);
  const monthDay = `${value.slice(4, 6)}.${value.slice(6, 8)}`;
  const day = new Date(`${value.slice(0, 4)}-${value.slice(4, 6)}-${value.slice(6, 8)}T00:00:00`).getDay();
  return { year, monthDay, weekday: ["周日", "周一", "周二", "周三", "周四", "周五", "周六"][day] };
}

export function localDateKey(value = new Date()) {
  return `${value.getFullYear()}${String(value.getMonth() + 1).padStart(2, "0")}${String(value.getDate()).padStart(2, "0")}`;
}
