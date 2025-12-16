// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  interface UserConfig {
    auto_shuffle: boolean;
    interval: number;
    show_layer: boolean;
  }
  type BingImage = {
    copyrights: string[];
    copyrightlinks: string[];
    startdates: string[];
    titles: string[];
    urls: string[];
  };
}

export {};
