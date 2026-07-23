import type { Wallpaper } from "../model/types";
import { formatFolioDate } from "../../../shared/utils/date";

export function RegionRail({ items, selectedId, showDate = false, onSelect }: { items: Wallpaper[]; selectedId: string; showDate?: boolean; onSelect: (item: Wallpaper) => void }) {
  return <div>{items.map((item, index) => {
    const selected = item.id === selectedId;
    return <button key={item.id} aria-current={selected ? "true" : undefined} aria-pressed={selected} aria-label={item.region} onClick={() => onSelect(item)}>
      <span data-region-thumb><img src={item.imageUrl} alt="" /></span>
      <span data-region-copy><span>{item.region}</span><small>{showDate ? formatFolioDate(item.date).monthDay : String(index + 1).padStart(2, "0")}</small></span>
    </button>;
  })}</div>;
}
