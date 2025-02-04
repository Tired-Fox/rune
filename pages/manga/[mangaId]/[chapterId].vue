<template>
  <div
    v-if="chapter"
    class="h-full"
  >
    <div v-if="strip" class="flex flex-col items-center w-full py-6">
      <img
        v-for="page in Object.entries(pages).sort((a, b) => a[0].localeCompare(b[0], undefined, { numeric: true, sensitivity: 'base' }))"
        :id="`page-${page[0]}`"
        :key="page[1]"
        :src="page[1]"
        class="w-auto h-auto max-w-[800px]"
      />
    </div>
    <div v-else class="relative w-full h-full py-6">
      <div class="absolute top-0 left-0 bottom-0 right-0 z-10">
        <button class="h-full w-1/2 text-transparent"
          @click="prev"
        >
          .
        </button>
        <button class="h-full w-1/2 text-transparent"
          @click="next"
        >
          .
        </button>
      </div>
      <div
        v-if="Object.keys(pages).length > currentPage"
        class="relative w-full h-full max-h-full rounded overflow-hidden"
        :style="{
          backgroundImage: `url('${pages[currentPage]}')`,
          backgroundSize: 'contain',
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'center',
        }"
      >
      </div>
      <USkeleton v-else class="w-full h-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, convertFileSrc, Channel } from '@tauri-apps/api/core';
import type { Chapter, Manga } from '~/types';
import { task } from '~/util/task';

const LONG_STRIP = "3e2b8dae-350e-4ab8-a8ce-016e844b9f0d";

const route = useRoute();

const mangaId = route.params.mangaId;
const chapterId = route.params.chapterId;

const manga: Ref<Manga> = ref(null as any);
const chapter: Ref<Chapter> = ref(null as any);
const cover: Ref<string|null> = ref(null);
const pages: Ref<{[k: number]: string}> = ref({});

const abortController = new AbortController();

const strip = computed(() => !!manga?.value.attributes.tags.find(t => t.id === LONG_STRIP));

const currentPage = ref<number>(0);
function next() {
  const length = Object.keys(pages.value).length;
  if (Object.keys(pages.value).length && currentPage.value < length - 1) {
    currentPage.value = currentPage.value + 1;
  }
}
function prev() {
  const length = Object.keys(pages.value).length;
  if (length && currentPage.value > 0) {
    currentPage.value = currentPage.value - 1;
  }
}

onMounted(async () => {
  await invoke<Manga>("get_manga", { id: mangaId, includes: ["cover_art"] })
    .then(response => manga.value = response);

  await invoke<Chapter>("get_chapter", { id: chapterId })
    .then(response => chapter.value = response);

  invoke<string>("get_cover_art", { manga: manga.value })
    .then(response => cover.value = convertFileSrc(response));

  invoke<string>("get_cover_art", { manga: manga.value })
    .then(response => cover.value = convertFileSrc(response));

  const onPage = new Channel<[number, string][]>();
  onPage.onmessage = message => {
    for (const [index, path] of message) {
      pages.value[index] = convertFileSrc(path);
    }
  };

  await task("get_pages", { manga: manga.value.id, chapter: chapter.value.id, onPage }, { signal: abortController.signal });
})

onUnmounted(() => {
  abortController.abort()
  //invoke('remove_cache', { targets: [
  //  {
  //    manga: manga.value.id,
  //    chapters: [chapter.value.id]
  //  }
  //]})
});
</script>
