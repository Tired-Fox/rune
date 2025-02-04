<template>
  <div>
    <div
      v-if="chapter"
    >
      <h1 class="text-2xl font-bold">
        {{ chapter.attributes.title?.length ? chapter.attributes.title : `Chapter ${chapter.attributes.chapter}` }}
      </h1>

      <div class="flex flex-col items-center w-full">
        <img
          v-for="page in Object.entries(pages).sort((a, b) => Number(a[0]) - Number(b[0]))"
          :id="`page-${page[0]}`"
          :key="page[1]"
          :src="page[1]"
          class="w-auto h-auto max-w-[800px]"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, convertFileSrc, Channel } from '@tauri-apps/api/core';
import type { Chapter, Manga } from '~/types';

const route = useRoute();

const mangaId = route.params.mangaId;
const chapterId = route.params.chapterId;

const manga: Ref<Manga> = ref(null as any);
const chapter: Ref<Chapter> = ref(null as any);
const cover: Ref<string|null> = ref(null);
const pages: Ref<{[k: number]: string}> = ref({});

onMounted(async () => {
  await invoke<Manga>("get_manga", { id: mangaId, includes: ["cover_art"] })
    .then(response => manga.value = response);

  await invoke<Chapter>("get_chapter", { id: chapterId })
    .then(response => chapter.value = response);

  invoke<string>("get_cover_art", { manga: manga.value })
    .then(response => cover.value = convertFileSrc(response));

  const onPage = new Channel<[number, string]>();
  onPage.onmessage = message => {
      console.log(message);
      let [index, path] = message;
      pages.value[index] = convertFileSrc(path);
  };
  await invoke("get_pages", { manga: manga.value.id, chapter: chapter.value.id, onPage });
})
</script>
