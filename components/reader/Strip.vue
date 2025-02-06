<template>
  <div class="mx-auto flex flex-col items-center w-[800px] py-6 origin-top" :style="{
    zoom: zoom,
  }">
    <img
      v-for="(page, i) in pages"
      :key="page"
      :id="`page-${i}`"
      :src="page"
      class="w-full h-auto"
    />
    <Skeleton v-if="!pages.length" class="w-full h-full max-w-[800px] aspect-[2/3] rounded" />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  zoom: number,
  pages: string[]
}>()
</script>
