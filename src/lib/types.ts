export interface GifApiResponse {
  results?: ResultsEntity[];
  error?: string;
}
export interface ResultsEntity {
  id: string;
  title: string;
  content_description: string;
  content_rating: string;
  h1_title: string;
  media?: MediaEntity[];
  bg_color: string;
  created: number;
  itemurl: string;
  url: string;
  tags?: null[];
  flags?: null[];
  shares: number;
  hasaudio: boolean;
  hascaption: boolean;
  source_id: string;
  composite?: null;
}
export interface MediaEntity {
  loopedmp4: Loopedmp4OrTinymp4OrMp4OrNanomp4;
  tinymp4: Loopedmp4OrTinymp4OrMp4OrNanomp4;
  tinygif: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  mp4: Loopedmp4OrTinymp4OrMp4OrNanomp4;
  nanowebm: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  mediumgif: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  nanomp4: Loopedmp4OrTinymp4OrMp4OrNanomp4;
  gif: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  nanogif: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  webm: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
  tinywebm: TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm;
}
export interface Loopedmp4OrTinymp4OrMp4OrNanomp4 {
  duration: number;
  size: number;
  preview: string;
  dims?: number[];
  url: string;
}
export interface TinygifOrNanowebmOrMediumgifOrGifOrNanogifOrWebmOrTinywebm {
  dims: number[];
  preview: string;
  url: string;
  size: number;
}
