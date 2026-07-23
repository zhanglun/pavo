export function formatFolioDate(value: string) {
  const year = value.slice(0, 4);
  const month = value.slice(4, 6);
  const dayOfMonth = value.slice(6, 8);
  const monthDay = `${month}.${dayOfMonth}`;
  const day = new Date(`${value.slice(0, 4)}-${value.slice(4, 6)}-${value.slice(6, 8)}T00:00:00`).getDay();
  const monthLabel = ["JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE", "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER"][Number(month) - 1];
  return {
    year,
    monthDay,
    monthLabel,
    dayOfMonth,
    weekday: ["周日", "周一", "周二", "周三", "周四", "周五", "周六"][day],
    weekdayLong: ["星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"][day],
  };
}

export function localDateKey(value = new Date()) {
  return `${value.getFullYear()}${String(value.getMonth() + 1).padStart(2, "0")}${String(value.getDate()).padStart(2, "0")}`;
}
