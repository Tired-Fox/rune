export type Paginated<T> = {
  total: number,
  limit: number,
  offset: number,
  data: T[]
};

export type Demographic =
   'shounen'
   | 'shoujo'
   | 'josei'
   | 'seinen';

export type Status =
    | 'ongoing'
    | 'completed'
    | 'hiatus'
    | 'cancelled';

export type ContentRating =
    | 'safe'
    | 'suggestive'
    | 'erotica'
    | 'pornographic';

export type TagGroup =
    | 'content'
    | 'format'
    | 'genre'
    | 'theme';

export type MangaState =
    | 'draft'
    | 'submitted'
    | 'published'
    | 'rejected';

export type Relation =
    | 'monochrome'
    | 'colored'
    | 'preserialization'
    | 'serialization'
    | 'prequel'
    | 'sequel'
    | 'main_story'
    | 'side_story'
    | 'adapted_from'
    | 'spin_off'
    | 'based_on'
    | 'doujinshi'
    | 'same_franchise'
    | 'shared_universe'
    | 'alternate_story'
    | 'alternate_version';

export type Relationship = {
    id: string,
    related?: Relation,
    type: string,
    attributes?: {[k: string]: string}
};

export type TagAttributes = {
    name: {[k:string]: string},
    description: {[k:string]: string},
    group: TagGroup,
    version: number,
};

export type Tag = {
    id: string,
    attributes: TagAttributes,
    relationships: Relationship[],
};

export type MangaAttributes = {
    title: {[k: string]: string},
    altTitles: {[k: string]: string}[],
    description: {[k: string]: string},
    isLocked: boolean,
    links: {[k: string]: string},
    originalLanguage: string,
    lastVolume?: string,
    lastChapter?: string,
    publicationDemographic?: Demographic,
    status?: Status,
    year?: number,
    contentRating: ContentRating,
    chapterNumbersResetOnNewVolume: boolean,
    availableTranslatedLanguages: string[],
    latestUploadedChapter: string,
    tags: Tag[],
    state: MangaState,
    version: number,
    createdAt: string,
    updatedAt: string,
};

export type Manga = {
    id: string,
    attributes: MangaAttributes,
    relationships: Relationship[],
};


export type ChapterAttributes = {
    title?: string,
    volume?: string,
    chapter?: string,
    externalUrl?: string,
    pages: number,
    version: number,
    translatedLanguage?: string,
    uploader?: string,
    createdAt?: string,
    updatedAt?: string,
    publishedAt?: string,
    readableAt?: string,
};

export type Chapter = {
    id: string,
    attributes: ChapterAttributes,
    relationships: Relationship[],
};

export type VolumeChapter = {
    chapter: string,
    id: string,
    others: string[],
    count: number,
};

export type Volume = {
    volume: string,
    count: number,
    chapters: {[k: string]: VolumeChapter},
};
