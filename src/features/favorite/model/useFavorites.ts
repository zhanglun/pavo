import { useState } from "react";
import { tauri } from "../../../shared/tauri/client";
import type { FavoriteItem } from "../../../entities/settings/model/types";
export function useFavorites() { const [favoriteIds, setFavoriteIds] = useState<Set<string>>(new Set()); const toggle = async (item: FavoriteItem) => { const exists = favoriteIds.has(item.filename); await (exists ? tauri.favorites.remove(item.filename) : tauri.favorites.add(item)); setFavoriteIds(current => { const next = new Set(current); exists ? next.delete(item.filename) : next.add(item.filename); return next; }); }; return { favoriteIds, toggle }; }
