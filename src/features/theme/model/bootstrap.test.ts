import { beforeEach, expect, test, vi } from "vitest";
import { applyInitialTheme } from "./bootstrap";

beforeEach(() => {
  localStorage.clear();
  delete document.documentElement.dataset.theme;
  window.matchMedia = vi.fn().mockReturnValue({ matches: false });
});

test("applies the persisted dark theme before React renders", () => {
  localStorage.setItem("pavo.theme-preference", "dark");

  applyInitialTheme();

  expect(document.documentElement.dataset.theme).toBe("dark");
});

test("resolves the system preference when no explicit theme is stored", () => {
  window.matchMedia = vi.fn().mockReturnValue({ matches: true });

  applyInitialTheme();

  expect(document.documentElement.dataset.theme).toBe("dark");
});
