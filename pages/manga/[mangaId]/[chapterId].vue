<template>
  <div v-if="chapter" class="h-full w-full relative overflow-auto pb-14">
    <div class="fixed h-0 left-0 right-0 bottom-6 z-20 px-2 flex justify-between text-xs">
      <div class="w-fit flex items-center gap-[.1rem] py-0 shadow shadow-black/20 dark:shadow-black rounded-full">
        <button @click="zoomIn" class="px-2 py-1 bg-cool-200/60 dark:bg-cool-800/60 rounded-l-full">
          <ChevronLeft class="w-3" />
        </button>
        <button class="px-2 bg-cool-200/60 dark:bg-cool-800/60 h-8 w-full grid items-center" @click="resetZoom">
          {{ Math.round(zoom * 100) }}%
        </button>
        <button @click="zoomOut" class="px-2 py-1 bg-cool-200/60 dark:bg-cool-800/60 rounded-r-full">
          <ChevronRight class="w-3" />
        </button>
      </div>
      <div v-if="!strip"
        class="w-fit flex items-center gap-[.1rem] py-0 shadow shadow-black/20 dark:shadow-black rounded-full">
        <button @click="prev" class="px-2 py-1 bg-cool-200/60 dark:bg-cool-800/60 rounded-l-full">
          <ChevronLeft class="w-3" />
        </button>
        <button class="px-2 bg-cool-200/60 dark:bg-cool-800/60 h-8 w-full grid items-center" @click="home">
          {{ currentPage + 1 }} / {{ Object.keys(pages).length }}
        </button>
        <button @click="next" class="px-2 py-1 bg-cool-200/60 dark:bg-cool-800/60 rounded-r-full">
          <ChevronRight class="w-3" />
        </button>
      </div>
    </div>
    <div v-if="strip" class="mx-auto flex flex-col items-center w-[800px] py-6 origin-top" :style="{
      zoom: zoom,
    }">
      <img
        v-for="page in Object.entries(pages).sort((a, b) => a[0].localeCompare(b[0], undefined, { numeric: true, sensitivity: 'base' }))"
        :id="`page-${page[0]}`" :key="page[1]" :src="page[1]" class="w-full h-auto" />
    </div>
    <div v-else class="relative w-full h-full">
      <div ref="root" class="absolute top-0 left-0 bottom-0 right-0 z-10">
        <div class="absolute left-0 bottom-0 top-0 right-1/2 w-1/2 text-transparent" @click="prev">
          .
        </div>
        <div class="absolute left-1/2 bottom-0 top-0 right-0 text-transparent" @click="next">
          .
        </div>
      </div>
      <div
        class="w-[100vw] h-[calc(100vh-7rem)] flex justify-center items-center"
        :style="{
          zoom: zoom
        }"
      >
        <div v-if="Object.keys(pages).length > currentPage"
          class="w-[100vw] h-[calc(100vh-7rem)] rounded border border-rose-500" :style="{
            backgroundImage: `url('${pages[currentPage]}')`,
            backgroundSize: 'contain',
            backgroundRepeat: 'no-repeat',
            backgroundPosition: 'center',
          }">
        </div>
        <USkeleton v-else class="w-full h-full max-w-[800px] aspect-[2/3]" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
import { invoke, convertFileSrc, Channel } from '@tauri-apps/api/core';
import type { Chapter, Manga } from '~/types';
import { task } from '~/util/task';
import { useMagicKeys, whenever } from '@vueuse/core';
import { logicNot, logicAnd } from '@vueuse/math';

const LONG_STRIP = "3e2b8dae-350e-4ab8-a8ce-016e844b9f0d";
const ZOOM_STEP = 0.05;

const route = useRoute();
const root = ref<HtmlDivElement | null>(null);
const abortController = new AbortController();

const mangaId = route.params.mangaId;
const chapterId = route.params.chapterId;

const manga: Ref<Manga> = ref(null as any);
const chapter: Ref<Chapter> = ref(null as any);
const pages: Ref<{ [k: number]: string }> = ref({});

const strip = computed(() => !!manga.value?.attributes.tags.find(t => t.id === LONG_STRIP));
const defaultZoom = computed(() => strip.value ? .70 : 1);

const zoom = ref(1);

const currentPage = ref<number>(0);
function next() {
  const length = Object.keys(pages.value).length;
  if (Object.keys(pages.value).length && currentPage.value < length - 1) {
    currentPage.value = currentPage.value + 1;
    if (zoom.value > 1) {
      zoom.value = 1;
    }
  }
}
function prev() {
  const length = Object.keys(pages.value).length;
  if (length && currentPage.value > 0) {
    currentPage.value = currentPage.value - 1;
    if (zoom.value > 1) {
      zoom.value = 1;
    }
  }
}
function home() {
  if (strip.value)
    window.scrollTo(0, 0);
  else
    currentPage.value = 0;
}
function end() {
  if (strip.value)
    window.scrollTo(0, document.body.scrollHeight);
  else
    currentPage.value = Object.keys(pages.value).length - 1;
}

const { HOME, END, ArrowRight, ArrowLeft } = useMagicKeys({
  onEventFired(e) {
    if (!strip.value) {
      if (e.key === 'arrowright' || e.key === 'arrowleft') {
        e.preventDefault()
      }
    }
  }
});
whenever(logicAnd(ArrowRight, logicNot(strip)), next);
whenever(logicAnd(ArrowLeft, logicNot(strip)), prev);
whenever(logicAnd(HOME, logicNot(strip)), home)
whenever(logicAnd(END, logicNot(strip)), end)

function zoomIn() {
  zoom.value = Math.min(zoom.value + ZOOM_STEP, 2);
}
function resetZoom() {
  zoom.value = defaultZoom.value;
}
function zoomOut() {
  zoom.value = Math.max(zoom.value - ZOOM_STEP, ZOOM_STEP);
}

async function setup() {

}

onMounted(async () => {
  await invoke<Manga>("get_manga", { id: mangaId })
    .then(response => manga.value = response);

  await invoke<Chapter>("get_chapter", { id: chapterId })
    .then(response => chapter.value = response);

  nextTick(() => {
    zoom.value = strip.value ? .70 : 1
  });

  window.addEventListener('wheel', (event) => {
    console.log(event);
    if ((event.wheelDelta && event.weelDelta > 0) || event.deltaY < 0) {
      if (event.ctrlKey) {
        zoomIn();
      }
    } else if (event.ctrlKey) {
      zoomOut();
    }
  });

  const onPage = new Channel<[number, string][]>();
  onPage.onmessage = message => {
    for (const [index, path] of message) {
      pages.value[index] = convertFileSrc(path);
    }
  };

  task("get_pages", { manga: manga.value.id, chapter: chapter.value.id, onPage }, { signal: abortController.signal });
})

onUnmounted(() => {
  abortController.abort()
  if (manga.value && chapter.value) {
    invoke('remove_cache', { targets: [
    {
      manga: manga.value.id,
      chapters: [chapter.value.id]
    }
    ]})
  }
});
</script>
