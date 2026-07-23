import { describe, expect, test } from "vitest";
import { normalizeCollection } from "./normalize";

describe("normalizeCollection", () => {
  test("skips only malformed region entries", () => {
    expect(normalizeCollection({
      filename: "OHR.Sample.jpg",
      regions: ["zh-CN", "en-US"],
      urls: ["https://example.test/cn.jpg", ""],
      titles: ["山谷", "Valley"],
      startdates: ["20260723", "20260723"],
      copyrights: ["版权", "Copyright"],
      copyrightlinks: ["https://example.test/cn", "https://example.test/us"],
    })).toEqual([
      expect.objectContaining({ filename: "OHR.Sample.jpg", region: "中国大陆" }),
    ]);
  });
});
