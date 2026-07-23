import type { SchedulerPhoto, Wallpaper } from "./types";

const regionNames: Record<string, string> = {
  "zh-CN": "中国大陆", "en-US": "美国", "fr-FR": "法国", "de-DE": "德国",
  "ja-JP": "日本", "en-CA": "加拿大", "en-GB": "英国", "en-IN": "印度", "it-IT": "意大利",
};

export function normalizeCollection(photo: SchedulerPhoto): Wallpaper[] {
  return photo.regions.flatMap((regionCode, index) => {
    const imageUrl = photo.urls[index];
    const title = photo.titles[index];
    const date = photo.startdates[index];
    if (!imageUrl || !title || !date) return [];
    return [{
      id: `${photo.filename}:${regionCode}`,
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
