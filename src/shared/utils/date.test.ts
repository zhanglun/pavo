import { expect, test } from "vitest";
import { formatFolioDate } from "./date";

test("formats a folio date", () => {
  expect(formatFolioDate("20260723")).toEqual({ year: "2026", monthDay: "07.23", weekday: "周四" });
});
