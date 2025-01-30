<template>
  <div class="w-full px-6 flex justify-center grid grid-cols-5">
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
          :src="`data:${cover[0]};base64,${cover[1]}`"
          class="aspect-2/3 h-auto w-full"
        />
      </div>
      <nuxt-link
        :to="`manga/${id}`"
      >
        {{ details.attributes.title.en }}
      </nuxt-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const manga: Ref<{ [k: string]: { details: object, cover: [string|null, string] }}> = ref({});

onMounted(async () => {
  await invoke('list_manga', { filters: {
      //ids: [
      //  "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc", // The Legendary Hero is an Academy Honors Student
      //  "7f491e32-3934-4e1a-a8b5-2510aecd40d9", // Cleric of Decay
      //],
      includes: ["cover_art"]
    }
  })
    .then(response => {
      for (const m of response.data) {
        manga.value[m.id] = { details: m };
      }
    });

  console.log("Loading images now")
  nextTick(() => {
    for (const [id, m] of Object.entries(manga.value)) {
      invoke('get_cover_art', { manga: m.details, size: "small" })
        .then(cover => {
          if (!!manga.value[id]) {
            manga.value[id].cover = cover;
          }
        })
          .catch(error => console.log(error));
    }
  })
})
</script>
