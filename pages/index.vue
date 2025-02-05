<template>
  <div class="w-full px-6">
    <template
      v-for="[name, list] of Object.entries(lists).sort(([_1, a], [_2, b]) => ids.indexOf(a.id) - ids.indexOf(b.id))"
    >
      <div class="px-6 pt-4 pb-8">
        <h2 class="text-xl font-bold">{{ name }}</h2>
        <UCarousel
          v-slot="{ item: manga }"
          :items="list.manga"
          :ui="{ indicators: { wrapper: '-bottom-4' }}"
          arrows
          indicators
        >
          <div v-if="typeof manga === 'string'" class="px-2 py-1">
            <USkeleton class="w-[15rem] aspect-[2/3] h-auto rounded" />
          </div>
          <nuxt-link
            v-else
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
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
const ids = [
  // Custom
  "8605b858-f366-4775-8d2a-3eccc78bde19",
  // Self Published
  "f66ebc10-ef89-46d1-be96-bb704559e04a",
  // Staff Picks
  "805ba886-dd99-4aa4-b460-4bd7c7b71352",
  // Featured by Supporters
  "5c5e6e39-0b4b-413e-be59-27b1ba03d1b9",
]

import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import type { Manga, Paginated, List } from '~/types';

const lists = ref<{ [name: string]: Pick<List, Exclude<keyof List, 'manga'>> & { manga: Manga[] | string[] } }>({});
const covers = ref<{[k: string]: string}>({});

onMounted(async () => {
  let custom_lists = await invoke<{ [name: string]: List }>('get_lists', { lists: ids });
  console.log(custom_lists);
  lists.value = custom_lists;

  let manga_ids: string[] = [];
  for (const list of Object.values(custom_lists)) {
    for (const id of list.manga) {
      if (!manga_ids.includes(id)) manga_ids.push(id);
    }
  }

  // PERF: Ensure all items are retrieved. Otherwise it should paginate.
  const manga = await invoke<Paginated<Manga>>('list_manga', { filters: {
      limit: Math.min(manga_ids.length, 100),
      ids: manga_ids,
      includes: ["cover_art"]
  }});

  manga.data.forEach(m => {
    invoke<string>('get_cover_art', { manga: m, size: 'large' })
      .then(cover => covers.value[m.id] = convertFileSrc(cover))
      .catch(err => console.error(err));
  })

  for (const name of Object.keys(custom_lists)) {
    lists.value[name] = {
      ...custom_lists[name],
      manga: manga.data.filter(m => custom_lists[name].manga.includes(m.id)),
    };
  }
});
</script>
