export type SchedulerPhoto = {
  filename: string;
  regions: string[];
  urls: string[];
  titles: string[];
  startdates: string[];
  copyrights: string[];
  copyrightlinks: string[];
};

export type Wallpaper = {
  id: string;
  filename: string;
  regionCode: string;
  region: string;
  imageUrl: string;
  title: string;
  date: string;
  copyright: string;
  sourceUrl: string;
  /** 这张图被哪些地区使用（历史档案合并同图多地区时填充） */
  regionCodes?: string[];
};
