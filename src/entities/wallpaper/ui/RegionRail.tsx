import type { Wallpaper } from "../model/types";
export function RegionRail({ items, selectedId, onSelect }: { items: Wallpaper[]; selectedId: string; onSelect: (item: Wallpaper) => void }) { return <div>{items.map(item => <button key={item.id} aria-pressed={item.id === selectedId} onClick={() => onSelect(item)}>{item.region}</button>)}</div>; }
