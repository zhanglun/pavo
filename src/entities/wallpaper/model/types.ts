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
};
