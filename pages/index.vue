<template>
  <h1 class="text-3xl font-bold">Hello, world</h1>
  <ul>
    <li
      v-for="({ details, cover }, id) in manga"
      :key="id"
    >
      <img
        v-if="cover"
        :src="`data:${cover[0]};base64,${cover[1]}`"
        loading="lazy"
      />
      <a
        :href="`/manga/${id}`"
      >
        {{ details.attributes.title.en }}
      </a>
    </li>
  </ul>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const manga: Ref<{ [k: string]: { details: object, cover: [string|null, string] }}> = ref({});

onMounted(() => {
  invoke('list_manga', { filters: {
      ids: [
        "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc", // The Legendary Hero is an Academy Honors Student
        "7f491e32-3934-4e1a-a8b5-2510aecd40d9", // Cleric of Decay
      ],
      includes: ["cover_art"]
    }
  })
    .then(response => {
      for (const m of response.data) {
        manga.value[m.id] = { details: m };
      }

      for (const [id, m] of Object.entries(manga.value)) {
          invoke('get_cover_image', { manga: m.details, size: "small" })
          .then(cover => {
            if (!!manga.value[id]) {
              manga.value[id].cover = cover;
            }
          })
            .catch(error => console.log(error));
      }
    });
})
</script>
