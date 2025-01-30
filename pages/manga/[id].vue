<template>
  <div v-if="manga">
    <h1 class="text-2xl font-bold">{{ manga.attributes.title.en }}</h1>
    <img
      v-if="cover"
      :src="`data:${cover[0]};base64,${cover[1]}`"
      loading="lazy"
      class="aspect-2/3 h-auto w-full"
    />
  </div>
  <div v-else>Loading...</div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const route = useRoute();
const id = route.params.id;
const manga: Ref<object|null> = ref(null);
const cover: Ref<[string|null, string]|null> = ref(null);

onMounted(async () => {
  await invoke("get_manga", { id, includes: ["cover_art"] })
    .then(response => {
      manga.value = response;
    });

  await invoke("get_cover_art", { manga: manga.value })
    .then(response => cover.value = response);
})
</script>
