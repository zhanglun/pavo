import { describe, expect, test } from "vitest";
import { normalizeCollection, selectRecentWallpapers, selectTodayWallpapers } from "./normalize";

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

describe("wallpaper selectors", () => {
  const mixed = {
    filename: "OHR.Shared_ZH-CN.jpg",
    regions: ["en-US", "zh-CN", "ja-JP"],
    urls: ["https://example.test/us.jpg", "https://example.test/cn.jpg", "https://example.test/jp.jpg"],
    titles: ["US", "中国", "日本"],
    startdates: ["20260722", "20260723", "20260723"],
    copyrights: ["US copy", "CN copy", "JP copy"],
    copyrightlinks: ["us-source", "cn-source", "jp-source"],
  };

  test("today includes only entries whose own date matches and follows region order", () => {
    expect(selectTodayWallpapers([mixed], "20260723").map((item) => item.title)).toEqual(["中国", "日本"]);
  });

  test("recent entries are sorted by date descending then stable region order", () => {
    const olderChina = { ...mixed, filename: "older.jpg", regions: ["zh-CN"], urls: ["older-cn"], titles: ["旧中国"], startdates: ["20260721"], copyrights: [""], copyrightlinks: [""] };
    const ancient = { ...olderChina, filename: "ancient.jpg", titles: ["过期内容"], startdates: ["20260701"] };
    expect(selectRecentWallpapers([ancient, olderChina, mixed], "20260723", 7).map((item) => item.title)).toEqual(["中国", "日本", "US", "旧中国"]);
  });

  test("today keeps only one wallpaper for each region", () => {
    const duplicateChina = {
      ...mixed,
      filename: "OHR.Duplicate_ZH-CN.jpg",
      regions: ["zh-CN"],
      urls: ["https://example.test/cn-duplicate.jpg"],
      titles: ["重复中国"],
      startdates: ["20260723"],
      copyrights: [""],
      copyrightlinks: [""],
    };

    expect(selectTodayWallpapers([mixed, duplicateChina], "20260723").map((item) => item.regionCode)).toEqual(["zh-CN", "ja-JP"]);
  });

  test("recent removes exact duplicate records without collapsing different dates or regions", () => {
    const exactDuplicate = {
      ...mixed,
      filename: "OHR.Duplicate_ZH-CN.jpg",
      regions: ["zh-CN"],
      urls: ["https://example.test/cn.jpg"],
      titles: ["中国"],
      startdates: ["20260723"],
      copyrights: ["CN copy"],
      copyrightlinks: ["cn-source"],
    };
    const reusedLater = {
      ...exactDuplicate,
      filename: "OHR.Reused_ZH-CN.jpg",
      startdates: ["20260722"],
    };
    const reusedElsewhere = {
      ...exactDuplicate,
      filename: "OHR.Reused_EN-US.jpg",
      regions: ["en-US"],
    };

    const selected = selectRecentWallpapers([mixed, exactDuplicate, reusedLater, reusedElsewhere], "20260723", 7);
    expect(selected.filter((item) => item.imageUrl === "https://example.test/cn.jpg")).toHaveLength(3);
  });
});
