<template>
  <div class="h-full w-full relative overflow-auto">
    <ReaderControls
      v-model:zoom="zoom"
      v-model:page="page"
      :count="Object.keys(pages).length"

      @prev-page="prev"
      @next-page="next"
      @reset-page="home"
      @zoom-in="zoomIn"
      @zoom-out="zoomOut"

      :page-controls="!strip"
      zoom-controls
    />
    <ReaderStrip
      v-if="strip"
      :pages="pages"
      :zoom="zoom"
    />
    <ReaderFlip
      v-else
      :pages="pages"
      :page="page"
      :zoom="zoom"
    />
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { useMagicKeys, whenever } from '@vueuse/core';
import { logicNot, logicAnd } from '@vueuse/math';

import type { Chapter, Manga } from '~/types';

const LONG_STRIP = "3e2b8dae-350e-4ab8-a8ce-016e844b9f0d";
const ZOOM_STEP = 0.05;

const route = useRoute();
const mangaId = route.params.mangaId;
const chapterId = route.params.chapterId;

const manga: Ref<Manga> = ref(null as any);
const chapter: Ref<Chapter> = ref(null as any);
const pages: Ref<string[]> = ref([]);
const page = ref<number>(0);
const strip = ref(false);
const zoom = ref(1);

const home = () => strip.value ? window.scrollTo(0, 0) : page.value = 0;
const end = () => strip.value ? window.scrollTo(0, document.body.scrollHeight) : page.value = Object.keys(pages.value).length - 1;
const zoomIn = () => zoom.value = Math.min(zoom.value + ZOOM_STEP, 2);
const zoomOut = () => zoom.value = Math.max(zoom.value - ZOOM_STEP, ZOOM_STEP);
const positiveWheel = (event: any) => (event.wheelDelta && event.weelDelta > 0) || event.deltaY < 0;

function next() {
  const length = Object.keys(pages.value).length;
  if (Object.keys(pages.value).length && page.value < length - 1) {
    page.value = page.value + 1;
    if (zoom.value > 1) {
      zoom.value = 1;
    }
  }
}

function prev() {
  const length = Object.keys(pages.value).length;
  if (length && page.value > 0) {
    page.value = page.value - 1;
    if (zoom.value > 1) {
      zoom.value = 1;
    }
  }
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

definePageMeta({
  layout: 'reader'
});

onMounted(() => {
  window.addEventListener('wheel', (event: any) => {
    if (positiveWheel(event) && event.ctrlKey) {
      zoomIn();
    } else if (event.ctrlKey) {
      zoomOut();
    }
  });
});

onMounted(() => {
  invoke<Manga>("get_manga", { id: mangaId })
    .then(response => {
      manga.value = response;
      strip.value = manga.value.attributes.tags.some(t => t.id === LONG_STRIP);
      if (strip.value) {
        zoom.value = 0.7;
      }
    });

  invoke<Chapter>("get_chapter", { id: chapterId })
    .then(response => chapter.value = response);

  invoke<string[]>('get_pages', {
    chapter: chapterId,
    save: true 
  })
    .then(response => pages.value = response);
});
</script>
