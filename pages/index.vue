<template>
  <div class="w-full px-6 flex justify-center grid grid-cols-5 gap-y-4">
    <template v-if="Object.keys(manga).length">
      <div
        v-for="({ details, cover }, id) in manga"
        :key="id"
        class="flex flex-col gap-1 w-40 mx-auto"
      >
        <div
          class="dark:bg-zinc-700 w-40 h-auto aspect-2/3 overflow-hidden rounded"
        >
          <img
            v-if="cover"
            :src="cover"
            class="aspect-2/3 h-auto w-full"
          />
        </div>
        <nuxt-link
          :to="`manga/${id}`"
        >
          {{ details.attributes.title.en }}
        </nuxt-link>
      </div>
    </template>
    <template v-else>
      <div
        v-for="index in 10"
        :key="index"
        class="flex flex-col gap-1 w-40 mx-auto"
      >
        <div
          class="dark:bg-zinc-700 w-40 h-auto aspect-2/3 overflow-hidden rounded"
        >
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { invoke, convertFileSrc } from '@tauri-apps/api/core';

const manga: Ref<{ [k: string]: { details: object, cover: string|null }}> = ref({});

onMounted(async () => {
  await invoke('list_manga', { filters: { includes: ["cover_art"] }})
    .then(response => {
      for (const m of response.data) {
        manga.value[m.id] = { details: m };
      }
    });

  for (const [id, m] of Object.entries(manga.value)) {
    invoke<string>('get_cover_art', { manga: m.details, size: "large" })
      .then(cover => {
        if (!!manga.value[id]) {
          manga.value[id].cover = convertFileSrc(cover);
        }
      })
        .catch(error => console.log(error));
  }
})
</script>
