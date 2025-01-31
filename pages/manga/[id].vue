<template>
  <div class="px-4">
    <div class="mx-6 flex gap-4 h-100">
      <div
          class="flex-none aspect-2/3 h-full w-auto dark:bg-zinc-800 rounded-md overflow-hidden"
      >
        <img
          v-if="cover"
          :src="cover"
          class="aspect-2/3 h-auto w-full"
        />
      </div>

      <div v-if="manga" class="flex flex-col h-100">
        <h1 class="text-2xl font-bold">{{ manga.attributes.title.en }}</h1>
        <p class="grow mt-4 overflow-y-auto p-4 rounded-md shadow shadow-black dark:bg-zinc-800">{{ manga.attributes.description.en }}</p>
      </div>
    </div>
    <div v-if="manga" class="px-6">
      <small class="px-2 py-0 rounded-md border border-green-500 bg-green-500/50 w-fit">
        {{ manga.attributes.contentRating }}
      </small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, convertFileSrc } from '@tauri-apps/api/core';

const route = useRoute();
const id = route.params.id;
const manga: Ref<object|null> = ref(null);
const cover: Ref<string|null> = ref(null);

onMounted(async () => {
  invoke("get_manga", { id, includes: ["cover_art"] })
    .then(response => {
      manga.value = response;
      invoke<string>("get_cover_art", { manga: manga.value, size: 'large' })
        .then(response => cover.value = convertFileSrc(response));
    });
})
</script>
