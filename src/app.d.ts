// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
    interface UserConfig {
      auto_shuffle: boolean;
      interval: number;
    }
  }
  type BingImage = {
    bot: number;
    copyright: string;
    copyrightlink: string;
    drk: number;
    enddate: string;
    fullstartdate: string;
    hs: string[];
    hsh: string;
    quiz: string;
    startdate: string;
    title: string;
    top: number;
    url: string;
    urlbase: string;
    wp: boolean;
  };
}

export {};
