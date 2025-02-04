<template>
  <div class="w-full px-6">
    <template v-if="manga?.data?.length">
      <UCarousel
        v-slot="{ item: manga }"
        :items="manga.data"
        :ui="{ indicators: { wrapper: '-bottom-4' }}"
        arrows
        indicators
      >
        <nuxt-link
          class="px-2 py-1 w-[15rem] relative flex flex-col items-start"
          :to="`/manga/${manga.id}`"
        >
          <div
            class="dark:bg-zinc-700 w-[14rem] aspect-[2/3] h-auto overflow-hidden rounded flex justify-center items-center"
          >
            <img
              v-if="covers[manga.id]"
              :src="covers[manga.id]"
              class="aspect-[2/3] h-auto w-full"
              draggable="false"
            />
            <USkeleton v-else class="aspect-[2/3] h-auto w-full" />
          </div>
          <span class="line-clamp-3 text-ellipsis" :title="manga?.attributes.title.en">{{ manga?.attributes.title.en }}</span>
        </nuxt-link>
      </UCarousel>
    </template>
  </div>
</template>

<script setup lang="ts">
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import type { Manga, Paginated, Volume } from '~/types';

const manga = ref<Paginated<Manga>>({} as any);
const covers = ref<{[k: string]: string}>({});

onMounted(async () => {
  await invoke<Paginated<Manga>>('list_manga', { filters: { includes: ["cover_art"], content_rating: ["safe"] }})
    .then(out => manga.value = out);

  for (const m of manga.value.data) {
    covers.value[m.id] = convertFileSrc(await invoke<string>('get_cover_art', { manga: m, size: "large" }));
  }
});
</script>
