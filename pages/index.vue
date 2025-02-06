<template>
  <div class="w-full flex flex-col gap-4">
    <template
      v-for="[name, list] of Object.entries(lists).sort(([_1, a], [_2, b]) => ids.indexOf(a.id) - ids.indexOf(b.id))"
    >
      <div class="px-12">
        <h2 class="text-xl font-bold">{{ name }}</h2>
        <Carousel>
          <CarouselContent class="-ml-4">
            <CarouselItem
              v-for="manga of list.manga"
              :key="JSON.stringify(manga)"
              class="sm:basis-1/2 md:basis-1/3 lg:basis-1/5 pl-4 max-w-[40ch]"
            >
              <div v-if="typeof manga === 'string'" class="py-1">
                <Skeleton class="w-full aspect-[2/3] h-auto rounded" />
              </div>
              <nuxt-link
                v-else
                class="py-1 w-full relative flex flex-col items-start"
                :to="`/manga/${manga.id}`"
              >
                <div
                  class="dark:bg-zinc-700 w-full aspect-[2/3] h-auto overflow-hidden rounded flex justify-center items-center"
                >
                  <Cover
                    class="aspect-[2/3] h-auto w-full"
                    :source="{ mangaId: manga.id, fileName: manga.relationships.find((r: Relationship) => r.type === 'cover_art')?.attributes?.fileName as any }"
                    draggable="false"
                  />
                </div>
                <span class="line-clamp-3 text-ellipsis" :title="manga?.attributes.title.en">{{ manga?.attributes.title.en }}</span>
              </nuxt-link>
            </CarouselItem>
          </CarouselContent>
          <CarouselPrevious />
          <CarouselNext />
        </Carousel>
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

import { invoke } from '@tauri-apps/api/core';
import type { Manga, Paginated, CustomList, List, Relationship } from '~/types';

const lists = ref<{ [name: string]: Pick<List, Exclude<keyof List, 'manga'>> & { manga: Manga[] | string[] } }>({});

onMounted(() => {
  for (const id of ids) {
    invoke<CustomList>('get_list', { id })
      .then(customList => {
        const name = customList.attributes.name;
        lists.value[name] = {
          id: customList.id,
          version: customList.attributes.version,
          visibility: customList.attributes.visibility,
          manga: customList.relationships.filter((r: Relationship) => r.type === 'manga').map((r: Relationship) => r.id),
          user: customList.relationships.find((r: Relationship) => r.type === 'user')?.id,
        };
        invoke<Paginated<Manga>>('list_manga', { filters: {
            limit: lists.value[name].manga.length,
            ids: lists.value[name].manga,
            includes: ["cover_art"]
        }})
          .then(manga => lists.value[name].manga = manga.data);
      });
  }
});
</script>
