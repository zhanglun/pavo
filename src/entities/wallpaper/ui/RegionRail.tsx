import type { Wallpaper } from "../model/types";
export function RegionRail({ items, selectedId, onSelect }: { items: Wallpaper[]; selectedId: string; onSelect: (item: Wallpaper) => void }) {
  return <div>{items.map((item) => <button key={item.id} aria-pressed={item.id === selectedId} aria-label={item.region} onClick={() => onSelect(item)}><img src={item.imageUrl} alt="" /><span>{item.region}</span></button>)}</div>;
}
