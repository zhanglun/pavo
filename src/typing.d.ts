// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  // interface Error {}
  // interface Locals {}
  // interface PageData {}
  // interface PageState {}
  // interface Platform {}
  interface UserConfig {
    auto_shuffle: boolean;
    interval: number;
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
