import { expect, test } from "vitest";
import { formatFolioDate, localDateKey } from "./date";

test("formats a folio date", () => {
  expect(formatFolioDate("20260723")).toEqual({
    year: "2026",
    monthDay: "07.23",
    monthLabel: "JULY",
    dayOfMonth: "23",
    weekday: "周四",
    weekdayLong: "星期四",
  });
});

test("formats a local date key without timezone conversion", () => {
  expect(localDateKey(new Date(2026, 6, 23, 23, 30))).toBe("20260723");
});
