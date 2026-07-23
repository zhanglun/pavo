import { describe, expect, test } from "vitest";
import { normalizeCollection, selectHistoryWallpapers, selectRecentWallpapers, selectRegionalFallbackWallpapers, selectTodayWallpapers } from "./normalize";

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

  test("today shows the latest available wallpaper for each region regardless of date mismatch", () => {
    // 时区错配下，en-US 的最新图可能还是 20260722，但它是该地区当前最新可得的，
    // 应当展示而非过滤掉。每地区取 date 最大的一张，按地区顺序排列。
    expect(selectTodayWallpapers([mixed], "20260723").map((item) => item.title)).toEqual(["中国", "US", "日本"]);
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

    // 新逻辑：每地区取最新一张。en-US(22)、zh-CN(23)、ja-JP(23) 都保留，
    // zh-CN 在 mixed 和 duplicate 中都有 23 号，保留后遍历到的 duplicate。
    expect(selectTodayWallpapers([mixed, duplicateChina], "20260723").map((item) => item.regionCode)).toEqual(["zh-CN", "en-US", "ja-JP"]);
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

  test("fallback chooses one recent and visually distinct wallpaper for each region", () => {
    const shared = {
      filename: "shared.jpg",
      regions: ["zh-CN", "en-US", "fr-FR"],
      urls: ["shared-cn-url", "shared-us-url", "shared-fr-url"],
      titles: ["中国共享", "美国共享", "法国共享"],
      startdates: ["20260723", "20260723", "20260723"],
      copyrights: ["", "", ""],
      copyrightlinks: ["", "", ""],
    };
    const usAlternative = { ...shared, filename: "us-old.jpg", regions: ["en-US"], urls: ["us-old"], titles: ["美国旧图"], startdates: ["20260722"], copyrights: [""], copyrightlinks: [""] };
    const franceAlternative = { ...shared, filename: "france-old.jpg", regions: ["fr-FR"], urls: ["france-old"], titles: ["法国旧图"], startdates: ["20260721"], copyrights: [""], copyrightlinks: [""] };

    const selected = selectRegionalFallbackWallpapers([shared, usAlternative, franceAlternative], "20260723", 7);

    expect(selected.map((item) => item.regionCode)).toEqual(["zh-CN", "en-US", "fr-FR"]);
    expect(new Set(selected.map((item) => item.imageUrl)).size).toBe(3);
    expect(selected.map((item) => item.imageUrl)).toEqual(["shared-cn-url", "us-old", "france-old"]);
  });

  test("normalized ids distinguish the same regional file reused on another date", () => {
    const reused = {
      ...mixed,
      regions: ["zh-CN", "zh-CN"],
      urls: ["same-url", "same-url"],
      titles: ["第一天", "第二天"],
      startdates: ["20260723", "20260722"],
      copyrights: ["", ""],
      copyrightlinks: ["", ""],
    };

    const ids = normalizeCollection(reused).map((item) => item.id);
    expect(new Set(ids).size).toBe(2);
  });

  test("history collapses regional variants of one visual but preserves another date", () => {
    const shared = {
      ...mixed,
      filename: "OHR.Shared_ZH-CN.jpg",
      regions: ["zh-CN", "en-US", "zh-CN"],
      urls: ["shared-cn", "shared-us", "shared-yesterday"],
      titles: ["中国共享", "美国共享", "昨日共享"],
      startdates: ["20260723", "20260723", "20260722"],
      copyrights: ["", "", ""],
      copyrightlinks: ["", "", ""],
    };

    const selected = selectHistoryWallpapers([shared], "20260723", 7);

    expect(selected.map((item) => item.title)).toEqual(["中国共享", "昨日共享"]);
  });
});
