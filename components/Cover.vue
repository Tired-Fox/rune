<template>
  <img
    :src="typeof source === 'object' ? `https://uploads.mangadex.org/covers/${source.mangaId}/${source.fileName}.512.jpg` : convertFileSrc(source)"
  />
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core';

defineProps<{
  source: {
    mangaId: string,
    fileName: string,
  }
  | string
}>();
</script>
