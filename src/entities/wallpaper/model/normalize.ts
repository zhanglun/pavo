import type { SchedulerPhoto, Wallpaper } from "./types";

const regionNames: Record<string, string> = {
  "zh-CN": "中国大陆", "en-US": "美国", "fr-FR": "法国", "de-DE": "德国",
  "ja-JP": "日本", "en-CA": "加拿大", "en-GB": "英国", "en-IN": "印度", "it-IT": "意大利",
};

const regionOrder = ["zh-CN", "en-US", "fr-FR", "de-DE", "ja-JP", "en-CA", "en-GB", "en-IN", "it-IT"];
const regionRank = (regionCode: string) => {
  const index = regionOrder.indexOf(regionCode);
  return index === -1 ? regionOrder.length : index;
};
const imageIdentity = (item: Wallpaper) => item.filename.split("_")[0] || item.filename;

export function normalizeCollection(photo: SchedulerPhoto): Wallpaper[] {
  return photo.regions.flatMap((regionCode, index) => {
    const imageUrl = photo.urls[index];
    const title = photo.titles[index];
    const date = photo.startdates[index];
    if (!imageUrl || !title || !date) return [];
    return [{
      id: `${photo.filename}:${regionCode}:${date}:${imageUrl}`,
      filename: photo.filename,
      regionCode,
      region: regionNames[regionCode] ?? regionCode,
      imageUrl,
      title,
      date,
      copyright: photo.copyrights[index] ?? "",
      sourceUrl: photo.copyrightlinks[index] ?? "",
    }];
  });
}

function sortWallpapers(items: Wallpaper[]) {
  return [...items].sort((left, right) => right.date.localeCompare(left.date) || regionRank(left.regionCode) - regionRank(right.regionCode));
}

function uniqueBy(items: Wallpaper[], keyOf: (item: Wallpaper) => string) {
  const seen = new Set<string>();
  return items.filter((item) => {
    const key = keyOf(item);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

export function selectTodayWallpapers(collection: SchedulerPhoto[], today: string) {
  const matches = sortWallpapers(collection.flatMap(normalizeCollection).filter((item) => item.date === today));
  return uniqueBy(matches, (item) => item.regionCode);
}

export function selectRecentWallpapers(collection: SchedulerPhoto[], today: string, days: number) {
  const reference = Date.UTC(Number(today.slice(0, 4)), Number(today.slice(4, 6)) - 1, Number(today.slice(6, 8)));
  const withinRange = (date: string) => {
    const value = Date.UTC(Number(date.slice(0, 4)), Number(date.slice(4, 6)) - 1, Number(date.slice(6, 8)));
    const difference = (reference - value) / 86_400_000;
    return Number.isFinite(difference) && difference >= 0 && difference <= days;
  };
  const matches = sortWallpapers(collection.flatMap(normalizeCollection).filter((item) => withinRange(item.date)));
  return uniqueBy(matches, (item) => `${item.date}\0${item.regionCode}\0${item.imageUrl}`);
}

export function selectHistoryWallpapers(collection: SchedulerPhoto[], today: string, days: number) {
  return uniqueBy(selectRecentWallpapers(collection, today, days), (item) => `${item.date}\0${imageIdentity(item)}`);
}

export function selectRegionalFallbackWallpapers(collection: SchedulerPhoto[], today: string, days: number) {
  const recent = selectRecentWallpapers(collection, today, days);
  const regions = uniqueBy(recent, (item) => item.regionCode)
    .sort((left, right) => regionRank(left.regionCode) - regionRank(right.regionCode))
    .map((item) => item.regionCode);
  const usedImages = new Set<string>();

  return regions.flatMap((regionCode) => {
    const candidate = recent.find((item) => item.regionCode === regionCode && !usedImages.has(imageIdentity(item)));
    if (!candidate) return [];
    usedImages.add(imageIdentity(candidate));
    return [candidate];
  });
}
